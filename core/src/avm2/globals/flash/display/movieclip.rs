//! `flash.display.MovieClip` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::array::ArrayStorage;
use crate::avm2::class::Class;
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{ArrayObject, Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::display_object::{MovieClip, Scene, TDisplayObject};
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;
use ruffle_types::string::{AvmString, WString};
use ruffle_types::tag_utils::SwfMovie;
use std::sync::Arc;

/// Implements `flash.display.MovieClip`'s instance constructor.
pub fn instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;

        if this.as_display_object().is_none() {
            let class_object = this
                .instance_of()
                .ok_or("Attempted to construct MovieClip on a bare object")?;
            let movie = Arc::new(SwfMovie::empty(activation.context.swf.version()));
            let new_do =
                MovieClip::new_with_avm2(movie, this, class_object, activation.context.gc_context);

            this.init_display_object(activation.context.gc_context, new_do.into());
        }
    }
    Ok(Value::Undefined)
}

/// Implements `flash.display.MovieClip`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `addFrameScript`, an undocumented method of `MovieClip` used to
/// specify what methods of a clip's class run on which frames.
pub fn add_frame_script<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        for (frame_id, callable) in args.chunks_exact(2).map(|s| (s[0], s[1])) {
            let frame_id = frame_id.coerce_to_u32(activation)? as u16 + 1;
            let callable = callable.as_callable(activation, None, None)?;

            mc.register_frame_script(frame_id, callable, &mut activation.context);
        }
    } else {
        log::error!("Attempted to add frame scripts to non-MovieClip this!");
    }

    Ok(Value::Undefined)
}

/// Implements `currentFrame`.
pub fn current_frame<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        if let Some(Scene {
            name: _,
            start,
            length: _,
        }) = mc.current_scene()
        {
            return Ok(((mc.current_frame() + 1) - start).into());
        } else {
            return Ok(mc.current_frame().into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `currentFrameLabel`.
pub fn current_frame_label<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        return Ok(mc
            .current_label()
            .and_then(|(label, start_frame)| {
                if start_frame < mc.current_frame() {
                    None
                } else {
                    Some(AvmString::new(activation.context.gc_context, label).into())
                }
            })
            .unwrap_or(Value::Null));
    }

    Ok(Value::Undefined)
}

/// Implements `currentLabel`.
pub fn current_label<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        return Ok(mc
            .current_label()
            .map(|(label, _start_frame)| {
                AvmString::new(activation.context.gc_context, label).into()
            })
            .unwrap_or(Value::Null));
    }

    Ok(Value::Undefined)
}

/// Given a scene, produce its name, length, and a list of frame labels.
///
/// The intended purpose of this output is to be sent directly into the
/// constructor of `flash.display.Scene`.
fn labels_for_scene<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    mc: MovieClip<'gc, B>,
    scene: &Scene,
) -> Result<(String, u16, Object<'gc, B>), Error> {
    let Scene {
        name: scene_name,
        start: scene_start,
        length: scene_length,
    } = scene;
    let frame_label_class = activation.context.avm2.classes().framelabel;
    let labels = mc.labels_in_range(*scene_start, scene_start + scene_length);
    let mut frame_labels = Vec::with_capacity(labels.len());

    for (name, frame) in labels {
        let name: Value<'gc, B> = AvmString::new(activation.context.gc_context, name).into();
        let local_frame = frame - scene_start + 1;
        let args = [name, local_frame.into()];
        let frame_label = frame_label_class.construct(activation, &args)?;

        frame_labels.push(Some(frame_label.into()));
    }

    Ok((
        scene_name.to_string(),
        *scene_length,
        ArrayObject::from_storage(activation, ArrayStorage::from_storage(frame_labels))?,
    ))
}

/// Implements `currentLabels`.
pub fn current_labels<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        let scene = mc.current_scene().unwrap_or_else(|| Scene {
            name: WString::default(),
            start: 0,
            length: mc.total_frames(),
        });
        return Ok(labels_for_scene(activation, mc, &scene)?.2.into());
    }

    Ok(Value::Undefined)
}

/// Implements `currentScene`.
pub fn current_scene<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        let scene = mc.current_scene().unwrap_or_else(|| Scene {
            name: WString::default(),
            start: 0,
            length: mc.total_frames(),
        });
        let (scene_name, scene_length, scene_labels) = labels_for_scene(activation, mc, &scene)?;
        let scene_class = activation.context.avm2.classes().scene;
        let args = [
            AvmString::new_utf8(activation.context.gc_context, scene_name).into(),
            scene_labels.into(),
            scene_length.into(),
        ];

        let scene = scene_class.construct(activation, &args)?;

        return Ok(scene.into());
    }

    Ok(Value::Undefined)
}

/// Implements `scenes`.
pub fn scenes<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        let mut mc_scenes = mc.scenes();
        if mc.scenes().is_empty() {
            mc_scenes.push(Scene {
                name: WString::default(),
                start: 0,
                length: mc.total_frames(),
            });
        }

        let mut scene_objects = Vec::with_capacity(mc_scenes.len());
        for scene in mc_scenes {
            let (scene_name, scene_length, scene_labels) =
                labels_for_scene(activation, mc, &scene)?;
            let scene_class = activation.context.avm2.classes().scene;
            let args = [
                AvmString::new_utf8(activation.context.gc_context, scene_name).into(),
                scene_labels.into(),
                scene_length.into(),
            ];

            let scene = scene_class.construct(activation, &args)?;

            scene_objects.push(Some(scene.into()));
        }

        return Ok(ArrayObject::from_storage(
            activation,
            ArrayStorage::from_storage(scene_objects),
        )?
        .into());
    }

    Ok(Value::Undefined)
}

/// Implements `framesLoaded`.
pub fn frames_loaded<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        return Ok(mc.frames_loaded().into());
    }

    Ok(Value::Undefined)
}

/// Implements `isPlaying`.
pub fn is_playing<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        return Ok((mc.programmatically_played() && mc.playing()).into());
    }

    Ok(Value::Undefined)
}

/// Implements `totalFrames`.
pub fn total_frames<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        return Ok(mc.total_frames().into());
    }

    Ok(Value::Undefined)
}

/// Implements `gotoAndPlay`.
pub fn goto_and_play<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        mc.set_programmatically_played(activation.context.gc_context);
        goto_frame(activation, mc, args, false)?;
    }

    Ok(Value::Undefined)
}

/// Implements `gotoAndStop`.
pub fn goto_and_stop<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        goto_frame(activation, mc, args, true)?;
    }

    Ok(Value::Undefined)
}

pub fn goto_frame<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    mc: MovieClip<'gc, B>,
    args: &[Value<'gc, B>],
    stop: bool,
) -> Result<(), Error> {
    let frame_or_label = args.get(0).cloned().unwrap_or(Value::Null);

    let scene = match args.get(1).cloned().unwrap_or(Value::Null) {
        Value::Null => None,
        v => mc
            .scene_label_to_number(&v.coerce_to_string(activation)?)
            .map(|v| v.saturating_sub(1)),
    }
    .unwrap_or(0) as u32;
    let frame = match frame_or_label {
        Value::Integer(i) => i as u32 + scene,
        Value::Unsigned(i) => i + scene,
        frame_or_label => {
            let frame_or_label = frame_or_label.coerce_to_string(activation)?;
            if let Ok(frame) = frame_or_label.parse::<u32>() {
                frame + scene
            } else {
                if let Some(scene) = args.get(1).cloned() {
                    //If the user specified a scene, we need to validate that
                    //the requested frame exists within that scene.
                    let scene = scene.coerce_to_string(activation)?;
                    if !mc.frame_exists_within_scene(&frame_or_label, &scene) {
                        return Err(format!(
                            "ArgumentError: Frame label {} not found in scene {}",
                            frame_or_label, scene
                        )
                        .into());
                    }
                }

                mc.frame_label_to_number(&frame_or_label).ok_or_else(|| {
                    format!(
                        "ArgumentError: {} is not a valid frame label.",
                        frame_or_label
                    )
                })? as u32
            }
        }
    };

    mc.goto_frame(&mut activation.context, frame as u16, stop);

    Ok(())
}

/// Implements `stop`.
pub fn stop<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        mc.stop(&mut activation.context);
    }

    Ok(Value::Undefined)
}

/// Implements `play`.
pub fn play<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        mc.set_programmatically_played(activation.context.gc_context);
        mc.play(&mut activation.context);
    }

    Ok(Value::Undefined)
}

/// Implements `prevFrame`.
pub fn prev_frame<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        mc.prev_frame(&mut activation.context);
    }

    Ok(Value::Undefined)
}

/// Implements `nextFrame`.
pub fn next_frame<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        mc.next_frame(&mut activation.context);
    }

    Ok(Value::Undefined)
}

/// Implements `prevScene`.
pub fn prev_scene<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        if let Some(Scene {
            name: _,
            start,
            length: _,
        }) = mc.previous_scene()
        {
            mc.goto_frame(&mut activation.context, start, false);
        }
    }

    Ok(Value::Undefined)
}

/// Implements `nextScene`.
pub fn next_scene<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mc) = this
        .and_then(|o| o.as_display_object())
        .and_then(|dobj| dobj.as_movie_clip())
    {
        if let Some(Scene {
            name: _,
            start,
            length: _,
        }) = mc.next_scene()
        {
            mc.goto_frame(&mut activation.context, start, false);
        }
    }

    Ok(Value::Undefined)
}

/// Construct `MovieClip`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.display"), "MovieClip"),
        Some(QName::new(Namespace::package("flash.display"), "Sprite").into()),
        Method::from_builtin(instance_init, "<MovieClip instance initializer>", mc),
        Method::from_builtin(class_init, "<MovieClip class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    let public_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[
        ("currentFrame", Some(current_frame), None),
        ("currentFrameLabel", Some(current_frame_label), None),
        ("currentLabel", Some(current_label), None),
        ("currentLabels", Some(current_labels), None),
        ("currentScene", Some(current_scene), None),
        ("scenes", Some(scenes), None),
        ("framesLoaded", Some(frames_loaded), None),
        ("isPlaying", Some(is_playing), None),
        ("totalFrames", Some(total_frames), None),
    ];
    write.define_public_builtin_instance_properties(mc, public_instance_properties);

    let public_instance_methods: &[(&str, NativeMethodImpl<B>)] = &[
        ("addFrameScript", add_frame_script),
        ("gotoAndPlay", goto_and_play),
        ("gotoAndStop", goto_and_stop),
        ("stop", stop),
        ("play", play),
        ("prevFrame", prev_frame),
        ("nextFrame", next_frame),
        ("prevScene", prev_scene),
        ("nextScene", next_scene),
    ];
    write.define_public_builtin_instance_methods(mc, public_instance_methods);

    class
}
