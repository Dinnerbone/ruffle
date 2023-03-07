use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::domain::Domain;
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::object::{ClassObject, FunctionObject, Object, ScriptObject, TObject};
use crate::avm2::scope::{Scope, ScopeChain};
use crate::avm2::script::Script;
use crate::avm2::Avm2;
use crate::avm2::Error;
use crate::avm2::Multiname;
use crate::avm2::Namespace;
use crate::avm2::QName;
use crate::string::AvmString;
use crate::tag_utils::{self, ControlFlow, SwfMovie, SwfSlice, SwfStream};
use gc_arena::{Collect, GcCell, MutationContext};
use std::sync::Arc;
use swf::TagCode;

mod array;
mod boolean;
mod class;
mod date;
mod error;
pub mod flash;
mod function;
mod global_scope;
mod int;
mod json;
mod math;
mod namespace;
mod number;
mod object;
mod qname;
mod regexp;
mod string;
mod toplevel;
mod r#uint;
mod vector;
mod xml;
mod xml_list;

/// This structure represents all system builtin classes.
#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct SystemClasses<'gc> {
    pub object: ClassObject<'gc>,
    pub function: ClassObject<'gc>,
    pub class: ClassObject<'gc>,
    pub global: ClassObject<'gc>,
    pub string: ClassObject<'gc>,
    pub boolean: ClassObject<'gc>,
    pub number: ClassObject<'gc>,
    pub int: ClassObject<'gc>,
    pub uint: ClassObject<'gc>,
    pub namespace: ClassObject<'gc>,
    pub array: ClassObject<'gc>,
    pub movieclip: ClassObject<'gc>,
    pub framelabel: ClassObject<'gc>,
    pub scene: ClassObject<'gc>,
    pub application_domain: ClassObject<'gc>,
    pub event: ClassObject<'gc>,
    pub fullscreenevent: ClassObject<'gc>,
    pub video: ClassObject<'gc>,
    pub xml: ClassObject<'gc>,
    pub xml_list: ClassObject<'gc>,
    pub display_object: ClassObject<'gc>,
    pub shape: ClassObject<'gc>,
    pub textfield: ClassObject<'gc>,
    pub textformat: ClassObject<'gc>,
    pub graphics: ClassObject<'gc>,
    pub loader: ClassObject<'gc>,
    pub loaderinfo: ClassObject<'gc>,
    pub bytearray: ClassObject<'gc>,
    pub stage: ClassObject<'gc>,
    pub sprite: ClassObject<'gc>,
    pub simplebutton: ClassObject<'gc>,
    pub regexp: ClassObject<'gc>,
    pub vector: ClassObject<'gc>,
    pub soundtransform: ClassObject<'gc>,
    pub soundchannel: ClassObject<'gc>,
    pub bitmap: ClassObject<'gc>,
    pub bitmapdata: ClassObject<'gc>,
    pub date: ClassObject<'gc>,
    pub qname: ClassObject<'gc>,
    pub mouseevent: ClassObject<'gc>,
    pub progressevent: ClassObject<'gc>,
    pub textevent: ClassObject<'gc>,
    pub errorevent: ClassObject<'gc>,
    pub ioerrorevent: ClassObject<'gc>,
    pub securityerrorevent: ClassObject<'gc>,
    pub transform: ClassObject<'gc>,
    pub colortransform: ClassObject<'gc>,
    pub matrix: ClassObject<'gc>,
    pub illegaloperationerror: ClassObject<'gc>,
    pub eventdispatcher: ClassObject<'gc>,
    pub rectangle: ClassObject<'gc>,
    pub keyboardevent: ClassObject<'gc>,
    pub point: ClassObject<'gc>,
    pub rangeerror: ClassObject<'gc>,
    pub referenceerror: ClassObject<'gc>,
    pub argumenterror: ClassObject<'gc>,
    pub typeerror: ClassObject<'gc>,
    pub verifyerror: ClassObject<'gc>,
    pub ioerror: ClassObject<'gc>,
    pub eoferror: ClassObject<'gc>,
    pub uncaughterrorevents: ClassObject<'gc>,
    pub statictext: ClassObject<'gc>,
    pub textlinemetrics: ClassObject<'gc>,
    pub stage3d: ClassObject<'gc>,
    pub context3d: ClassObject<'gc>,
    pub indexbuffer3d: ClassObject<'gc>,
    pub vertexbuffer3d: ClassObject<'gc>,
    pub program3d: ClassObject<'gc>,
    pub urlvariables: ClassObject<'gc>,
    pub bevelfilter: ClassObject<'gc>,
    pub bitmapfilter: ClassObject<'gc>,
    pub blurfilter: ClassObject<'gc>,
    pub colormatrixfilter: ClassObject<'gc>,
    pub convolutionfilter: ClassObject<'gc>,
    pub displacementmapfilter: ClassObject<'gc>,
    pub dropshadowfilter: ClassObject<'gc>,
    pub glowfilter: ClassObject<'gc>,
    pub gradientbevelfilter: ClassObject<'gc>,
    pub gradientglowfilter: ClassObject<'gc>,
    pub texture: ClassObject<'gc>,
    pub cubetexture: ClassObject<'gc>,
    pub rectangletexture: ClassObject<'gc>,
}

impl<'gc> SystemClasses<'gc> {
    /// Construct a minimal set of system classes necessary for bootstrapping
    /// player globals.
    ///
    /// All other system classes aside from the three given here will be set to
    /// the empty object also handed to this function. It is the caller's
    /// responsibility to instantiate each class and replace the empty object
    /// with that.
    fn new(
        object: ClassObject<'gc>,
        function: ClassObject<'gc>,
        class: ClassObject<'gc>,
        global: ClassObject<'gc>,
    ) -> Self {
        SystemClasses {
            object,
            function,
            class,
            global,
            // temporary initialization
            string: object,
            boolean: object,
            number: object,
            int: object,
            uint: object,
            namespace: object,
            array: object,
            movieclip: object,
            framelabel: object,
            scene: object,
            application_domain: object,
            event: object,
            fullscreenevent: object,
            video: object,
            xml: object,
            xml_list: object,
            display_object: object,
            shape: object,
            textfield: object,
            textformat: object,
            graphics: object,
            loader: object,
            loaderinfo: object,
            bytearray: object,
            stage: object,
            sprite: object,
            simplebutton: object,
            regexp: object,
            vector: object,
            soundtransform: object,
            soundchannel: object,
            bitmap: object,
            bitmapdata: object,
            date: object,
            qname: object,
            mouseevent: object,
            progressevent: object,
            textevent: object,
            errorevent: object,
            ioerrorevent: object,
            securityerrorevent: object,
            transform: object,
            colortransform: object,
            matrix: object,
            illegaloperationerror: object,
            eventdispatcher: object,
            rectangle: object,
            keyboardevent: object,
            point: object,
            rangeerror: object,
            referenceerror: object,
            argumenterror: object,
            typeerror: object,
            verifyerror: object,
            ioerror: object,
            eoferror: object,
            uncaughterrorevents: object,
            statictext: object,
            textlinemetrics: object,
            stage3d: object,
            context3d: object,
            indexbuffer3d: object,
            vertexbuffer3d: object,
            program3d: object,
            urlvariables: object,
            bevelfilter: object,
            bitmapfilter: object,
            blurfilter: object,
            colormatrixfilter: object,
            convolutionfilter: object,
            displacementmapfilter: object,
            dropshadowfilter: object,
            glowfilter: object,
            gradientbevelfilter: object,
            gradientglowfilter: object,
            texture: object,
            cubetexture: object,
            rectangletexture: object,
        }
    }
}

/// Add a free-function builtin to the global scope.
fn function<'gc>(
    activation: &mut Activation<'_, 'gc>,
    package: impl Into<AvmString<'gc>>,
    name: &'static str,
    nf: NativeMethodImpl,
    script: Script<'gc>,
) -> Result<(), Error<'gc>> {
    let (_, mut global, mut domain) = script.init();
    let mc = activation.context.gc_context;
    let scope = activation.create_scopechain();
    let qname = QName::new(
        Namespace::package(package, activation.context.gc_context),
        name,
    );
    let method = Method::from_builtin(nf, name, mc);
    let as3fn = FunctionObject::from_method(activation, method, scope, None, None).into();
    domain.export_definition(qname, script, mc);
    global.install_const_late(mc, qname, as3fn, activation.avm2().classes().function);

    Ok(())
}

/// Add a fully-formed class object builtin to the global scope.
///
/// This allows the caller to pre-populate the class's prototype with dynamic
/// properties, if necessary.
fn dynamic_class<'gc>(
    mc: MutationContext<'gc, '_>,
    class_object: ClassObject<'gc>,
    script: Script<'gc>,
    // The `ClassObject` of the `Class` class
    class_class: ClassObject<'gc>,
) {
    let (_, mut global, mut domain) = script.init();
    let class = class_object.inner_class_definition();
    let name = class.read().name();

    global.install_const_late(mc, name, class_object.into(), class_class);
    domain.export_definition(name, script, mc)
}

/// Add a class builtin to the global scope.
///
/// This function returns the class object and class prototype as a class, which
/// may be stored in `SystemClasses`
fn class<'gc>(
    class_def: GcCell<'gc, Class<'gc>>,
    script: Script<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<ClassObject<'gc>, Error<'gc>> {
    let (_, mut global, mut domain) = script.init();

    let class_read = class_def.read();
    let super_class = if let Some(sc_name) = class_read.super_class_name() {
        let super_class: Result<Object<'gc>, Error<'gc>> = activation
            .resolve_definition(sc_name)
            .ok()
            .and_then(|v| v)
            .and_then(|v| v.as_object())
            .ok_or_else(|| {
                format!(
                    "Could not resolve superclass {} when defining global class {}",
                    sc_name.to_qualified_name(activation.context.gc_context),
                    class_read
                        .name()
                        .to_qualified_name(activation.context.gc_context)
                )
                .into()
            });
        let super_class = super_class?
            .as_class_object()
            .ok_or_else(|| Error::from("Base class of a global class is not a class"))?;

        Some(super_class)
    } else {
        None
    };

    let class_name = class_read.name();
    drop(class_read);

    let class_object = ClassObject::from_class(activation, class_def, super_class)?;
    global.install_const_late(
        activation.context.gc_context,
        class_name,
        class_object.into(),
        activation.avm2().classes().class,
    );
    domain.export_definition(class_name, script, activation.context.gc_context);
    domain.export_class(class_def, activation.context.gc_context);

    Ok(class_object)
}

macro_rules! avm2_system_class {
    ($field:ident, $activation:ident, $class:expr, $script:expr) => {
        let class_object = class($class, $script, $activation)?;

        let sc = $activation.avm2().system_classes.as_mut().unwrap();
        sc.$field = class_object;
    };
}

/// Initialize the player global domain.
///
/// This should be called only once, to construct the global scope of the
/// player. It will return a list of prototypes it has created, which should be
/// stored on the AVM. All relevant declarations will also be attached to the
/// given domain.
pub fn load_player_globals<'gc>(
    activation: &mut Activation<'_, 'gc>,
    domain: Domain<'gc>,
) -> Result<(), Error<'gc>> {
    let mc = activation.context.gc_context;

    let globals = ScriptObject::custom_object(activation.context.gc_context, None, None);
    let gs = ScopeChain::new(domain).chain(mc, &[Scope::new(globals)]);
    let script = Script::empty_script(mc, globals, domain);

    // Set the outer scope of this activation to the global scope.
    activation.set_outer(gs);

    // public / root package
    //
    // This part of global initialization is very complicated, because
    // everything has to circularly reference everything else:
    //
    //  - Object is an instance of itself, as well as it's prototype
    //  - All other types are instances of Class, which is an instance of
    //    itself
    //  - Function's prototype is an instance of itself
    //  - All methods created by the above-mentioned classes are also instances
    //    of Function
    //  - All classes are put on Global's trait list, but Global needs
    //    to be initialized first, but you can't do that until Object/Class are ready.
    //
    // Hence, this ridiculously complicated dance of classdef, type allocation,
    // and partial initialization.
    let object_classdef = object::create_class(activation);
    let object_class = ClassObject::from_class_partial(activation, object_classdef, None)?;
    let object_proto = ScriptObject::custom_object(mc, Some(object_class), None);
    domain.export_class(object_classdef, mc);

    let fn_classdef = function::create_class(activation);
    let fn_class = ClassObject::from_class_partial(activation, fn_classdef, Some(object_class))?;
    let fn_proto = ScriptObject::custom_object(mc, Some(fn_class), Some(object_proto));
    domain.export_class(fn_classdef, mc);

    let class_classdef = class::create_class(activation);
    let class_class =
        ClassObject::from_class_partial(activation, class_classdef, Some(object_class))?;
    let class_proto = ScriptObject::custom_object(mc, Some(object_class), Some(object_proto));
    domain.export_class(class_classdef, mc);

    let global_classdef = global_scope::create_class(activation);
    let global_class =
        ClassObject::from_class_partial(activation, global_classdef, Some(object_class))?;
    let global_proto = ScriptObject::custom_object(mc, Some(object_class), Some(object_proto));
    domain.export_class(global_classdef, mc);

    // Now to weave the Gordian knot...
    object_class.link_prototype(activation, object_proto)?;
    object_class.link_type(activation, class_proto, class_class);

    fn_class.link_prototype(activation, fn_proto)?;
    fn_class.link_type(activation, class_proto, class_class);

    class_class.link_prototype(activation, class_proto)?;
    class_class.link_type(activation, class_proto, class_class);

    global_class.link_prototype(activation, global_proto)?;
    global_class.link_type(activation, class_proto, class_class);

    // At this point, we need at least a partial set of system classes in
    // order to continue initializing the player. The rest of the classes
    // are set to a temporary class until we have a chance to initialize them.

    activation.context.avm2.system_classes = Some(SystemClasses::new(
        object_class,
        fn_class,
        class_class,
        global_class,
    ));

    // Our activation environment is now functional enough to finish
    // initializing the core class weave. The order of initialization shouldn't
    // matter here, as long as all the initialization machinery can see and
    // link the various system types together correctly.
    let class_class = class_class.into_finished_class(activation)?;
    let fn_class = fn_class.into_finished_class(activation)?;
    let object_class = object_class.into_finished_class(activation)?;
    let _global_class = global_class.into_finished_class(activation)?;

    globals.set_proto(mc, global_proto);
    globals.set_instance_of(mc, global_class);
    globals.fork_vtable(activation.context.gc_context);

    // From this point, `globals` is safe to be modified

    dynamic_class(mc, object_class, script, class_class);
    dynamic_class(mc, fn_class, script, class_class);
    dynamic_class(mc, class_class, script, class_class);

    // After this point, it is safe to initialize any other classes.
    // Make sure to initialize superclasses *before* their subclasses!

    avm2_system_class!(string, activation, string::create_class(activation), script);
    avm2_system_class!(
        boolean,
        activation,
        boolean::create_class(activation),
        script
    );
    avm2_system_class!(number, activation, number::create_class(activation), script);
    avm2_system_class!(int, activation, int::create_class(activation), script);
    avm2_system_class!(uint, activation, uint::create_class(activation), script);
    avm2_system_class!(
        namespace,
        activation,
        namespace::create_class(activation),
        script
    );
    avm2_system_class!(qname, activation, qname::create_class(activation), script);
    avm2_system_class!(array, activation, array::create_class(activation), script);

    function(activation, "", "trace", toplevel::trace, script)?;
    function(
        activation,
        "__ruffle__",
        "log_warn",
        toplevel::log_warn,
        script,
    )?;
    function(
        activation,
        "__ruffle__",
        "stub_method",
        toplevel::stub_method,
        script,
    )?;
    function(
        activation,
        "__ruffle__",
        "stub_getter",
        toplevel::stub_getter,
        script,
    )?;
    function(
        activation,
        "__ruffle__",
        "stub_setter",
        toplevel::stub_setter,
        script,
    )?;
    function(
        activation,
        "__ruffle__",
        "stub_constructor",
        toplevel::stub_constructor,
        script,
    )?;
    function(activation, "", "isFinite", toplevel::is_finite, script)?;
    function(activation, "", "isNaN", toplevel::is_nan, script)?;
    function(activation, "", "parseInt", toplevel::parse_int, script)?;
    function(activation, "", "parseFloat", toplevel::parse_float, script)?;
    function(activation, "", "escape", toplevel::escape, script)?;
    function(activation, "", "encodeURI", toplevel::encode_uri, script)?;
    function(
        activation,
        "",
        "encodeURIComponent",
        toplevel::encode_uri_component,
        script,
    )?;

    avm2_system_class!(regexp, activation, regexp::create_class(activation), script);
    avm2_system_class!(vector, activation, vector::create_class(activation), script);

    avm2_system_class!(date, activation, date::create_class(activation), script);

    // package `flash.system`
    avm2_system_class!(
        application_domain,
        activation,
        flash::system::application_domain::create_class(activation),
        script
    );

    // package `flash.text`
    class(
        flash::text::font::create_class(activation),
        script,
        activation,
    )?;

    // Inside this call, the macro `avm2_system_classes_playerglobal`
    // triggers classloading. Therefore, we run `load_playerglobal`
    // relative late, so that it can access classes defined before
    // this call.
    load_playerglobal(activation, domain)?;

    // Everything after the `load_playerglobal` call needs classes
    // defined in the playerglobal swf.

    // package `flash.media`
    class(
        flash::media::sound::create_class(activation),
        script,
        activation,
    )?;
    avm2_system_class!(
        soundtransform,
        activation,
        flash::media::soundtransform::create_class(activation),
        script
    );
    class(
        flash::media::soundmixer::create_class(activation),
        script,
        activation,
    )?;
    avm2_system_class!(
        soundchannel,
        activation,
        flash::media::soundchannel::create_class(activation),
        script
    );

    Ok(())
}

/// This file is built by 'core/build_playerglobal/'
/// See that tool, and 'core/src/avm2/globals/README.md', for more details
const PLAYERGLOBAL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/playerglobal.swf"));

mod native {
    include!(concat!(env!("OUT_DIR"), "/native_table.rs"));
}

/// Loads classes from our custom 'playerglobal' (which are written in ActionScript)
/// into the environment. See 'core/src/avm2/globals/README.md' for more information
fn load_playerglobal<'gc>(
    activation: &mut Activation<'_, 'gc>,
    domain: Domain<'gc>,
) -> Result<(), Error<'gc>> {
    activation.avm2().native_method_table = native::NATIVE_METHOD_TABLE;
    activation.avm2().native_instance_allocator_table = native::NATIVE_INSTANCE_ALLOCATOR_TABLE;
    activation.avm2().native_instance_init_table = native::NATIVE_INSTANCE_INIT_TABLE;

    let movie = SwfMovie::from_data(PLAYERGLOBAL, "file:///".into(), None)
        .expect("playerglobal.swf should be valid");

    let slice = SwfSlice::from(Arc::new(movie));

    let mut reader = slice.read_from(0);

    let tag_callback = |reader: &mut SwfStream<'_>, tag_code, _tag_len| {
        if tag_code == TagCode::DoAbc2 {
            let do_abc = reader
                .read_do_abc_2()
                .expect("playerglobal.swf should be valid");
            Avm2::do_abc(&mut activation.context, do_abc.data, do_abc.flags, domain)
                .expect("playerglobal.swf should be valid");
        } else if tag_code != TagCode::End {
            panic!("playerglobal should only contain `DoAbc2` tag - found tag {tag_code:?}")
        }
        Ok(ControlFlow::Continue)
    };

    let _ = tag_utils::decode_tags(&mut reader, tag_callback);
    macro_rules! avm2_system_classes_playerglobal {
        ($activation:expr, $script:expr, [$(($package:expr, $class_name:expr, $field:ident)),* $(,)?]) => {
            $(
                let name = Multiname::new(Namespace::package($package, activation.context.gc_context), $class_name);
                let class_object = activation.resolve_class(&name)?;
                let sc = $activation.avm2().system_classes.as_mut().unwrap();
                sc.$field = class_object;
            )*
        }
    }

    // This acts the same way as 'avm2_system_class', but for classes
    // declared in 'playerglobal'. Classes are declared as ("package", "class", field_name),
    // and are stored in 'avm2().system_classes'
    avm2_system_classes_playerglobal!(
        activation,
        script,
        [
            ("", "ArgumentError", argumenterror),
            ("", "RangeError", rangeerror),
            ("", "ReferenceError", referenceerror),
            ("", "TypeError", typeerror),
            ("", "VerifyError", verifyerror),
            ("", "XML", xml),
            ("", "XMLList", xml_list),
            ("flash.display", "Bitmap", bitmap),
            ("flash.display", "BitmapData", bitmapdata),
            ("flash.display", "Scene", scene),
            ("flash.display", "FrameLabel", framelabel),
            ("flash.display", "Graphics", graphics),
            ("flash.display", "Loader", loader),
            ("flash.display", "LoaderInfo", loaderinfo),
            ("flash.display", "MovieClip", movieclip),
            ("flash.display", "Shape", shape),
            ("flash.display", "SimpleButton", simplebutton),
            ("flash.display", "Sprite", sprite),
            ("flash.display", "Stage", stage),
            ("flash.display", "Stage3D", stage3d),
            ("flash.display3D", "Context3D", context3d),
            ("flash.display3D", "IndexBuffer3D", indexbuffer3d),
            ("flash.display3D", "Program3D", program3d),
            ("flash.display3D.textures", "CubeTexture", cubetexture),
            ("flash.display3D.textures", "Texture", texture),
            (
                "flash.display3D.textures",
                "RectangleTexture",
                rectangletexture
            ),
            ("flash.display3D", "VertexBuffer3D", vertexbuffer3d),
            (
                "flash.errors",
                "IllegalOperationError",
                illegaloperationerror
            ),
            ("flash.errors", "IOError", ioerror),
            ("flash.errors", "EOFError", eoferror),
            ("flash.events", "Event", event),
            ("flash.events", "EventDispatcher", eventdispatcher),
            ("flash.events", "TextEvent", textevent),
            ("flash.events", "ErrorEvent", errorevent),
            ("flash.events", "KeyboardEvent", keyboardevent),
            ("flash.events", "ProgressEvent", progressevent),
            ("flash.events", "SecurityErrorEvent", securityerrorevent),
            ("flash.events", "IOErrorEvent", ioerrorevent),
            ("flash.events", "MouseEvent", mouseevent),
            ("flash.events", "FullScreenEvent", fullscreenevent),
            ("flash.events", "UncaughtErrorEvents", uncaughterrorevents),
            ("flash.geom", "Matrix", matrix),
            ("flash.geom", "Point", point),
            ("flash.geom", "Rectangle", rectangle),
            ("flash.geom", "Transform", transform),
            ("flash.geom", "ColorTransform", colortransform),
            ("flash.net", "URLVariables", urlvariables),
            ("flash.utils", "ByteArray", bytearray),
            ("flash.text", "StaticText", statictext),
            ("flash.text", "TextFormat", textformat),
            ("flash.text", "TextField", textfield),
            ("flash.text", "TextLineMetrics", textlinemetrics),
            ("flash.filters", "BevelFilter", bevelfilter),
            ("flash.filters", "BitmapFilter", bitmapfilter),
            ("flash.filters", "BlurFilter", blurfilter),
            ("flash.filters", "ColorMatrixFilter", colormatrixfilter),
            ("flash.filters", "ConvolutionFilter", convolutionfilter),
            (
                "flash.filters",
                "DisplacementMapFilter",
                displacementmapfilter
            ),
            ("flash.filters", "DropShadowFilter", dropshadowfilter),
            ("flash.filters", "GlowFilter", glowfilter),
            ("flash.filters", "GradientBevelFilter", gradientbevelfilter),
            ("flash.filters", "GradientGlowFilter", gradientglowfilter),
        ]
    );

    // Domain memory must be initialized after playerglobals is loaded because it relies on ByteArray.
    domain.init_default_domain_memory(activation)?;
    Ok(())
}
