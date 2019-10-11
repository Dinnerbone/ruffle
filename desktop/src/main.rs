mod audio;
mod navigator;
mod render;

use crate::render::GliumRenderBackend;
use glutin::{
    dpi::{LogicalSize, PhysicalPosition},
    ContextBuilder, ElementState, EventsLoop, MouseButton, WindowBuilder, WindowEvent,
};
use ruffle_core::{backend::render::RenderBackend, Player, swf};
use std::path::{PathBuf, Path};
use std::time::{Duration, Instant};
use structopt::StructOpt;
use std::fs;
use csv::Writer;
use std::io::{Read, Error};
use swf::{Header, Tag};
use swf::read::Reader;
use path_slash::PathExt;
use serde::Serialize;
use std::ffi::OsStr;
use swf::avm1::types::Action;
use std::fs::File;
use std::panic::catch_unwind;
use std::any::Any;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    input_path: PathBuf,
}

fn visit_dirs(dir: &Path, paths: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, paths)?;
            } else if path.is_file() && path.extension().and_then(OsStr::to_str) == Some("swf") {
                paths.push(path.to_owned());
            }
        }
    }
    Ok(())
}

#[derive(Default, Serialize)]
struct Row {
    file: Option<String>,
    error: Option<String>,
    version: u8,
    script_limits: i32,
    show_frame: i32,
    protect: i32,
    csm_text_settings: i32,
    debug_id: i32,
    define_binary_data: i32,
    define_bits: i32,
    define_bits_jpeg2: i32,
    define_bits_jpeg3: i32,
    define_bits_lossless: i32,
    define_button: i32,
    define_button2: i32,
    define_button_color_transform: i32,
    define_button_sound: i32,
    define_edit_text: i32,
    define_font: i32,
    define_font2: i32,
    define_font4: i32,
    define_font_align_zones: i32,
    define_font_info: i32,
    define_font_name: i32,
    define_morph_shape: i32,
    define_scaling_grid: i32,
    define_shape: i32,
    define_sound: i32,
    define_sprite: i32,
    define_text: i32,
    define_video_stream: i32,
    do_abc: i32,
    do_action: i32,
    do_init_action: i32,
    enable_debugger: i32,
    enable_telemetry: i32,
    end: i32,
    metadata: i32,
    import_assets: i32,
    jpeg_tables: i32,
    set_background_color: i32,
    set_tab_index: i32,
    sound_stream_block: i32,
    sound_stream_head: i32,
    sound_stream_head2: i32,
    start_sound: i32,
    start_sound2: i32,
    symbol_class: i32,
    place_object: i32,
    remove_object: i32,
    video_frame: i32,
    file_attributes: i32,
    frame_label: i32,
    define_scene_and_frame_label_data: i32,
    product_info: i32,
    export_assets: i32,
    unknown: i32,
    action_add: i32,
    action_add2: i32,
    action_and: i32,
    action_ascii_to_char: i32,
    action_bit_and: i32,
    action_bit_l_shift: i32,
    action_bit_or: i32,
    action_bit_r_shift: i32,
    action_bit_u_r_shift: i32,
    action_bit_xor: i32,
    action_call: i32,
    action_call_function: i32,
    action_call_method: i32,
    action_cast_op: i32,
    action_char_to_ascii: i32,
    action_clone_sprite: i32,
    action_constant_pool: i32,
    action_decrement: i32,
    action_define_function: i32,
    action_define_function2: i32,
    action_define_local: i32,
    action_define_local2: i32,
    action_delete: i32,
    action_delete2: i32,
    action_divide: i32,
    action_end_drag: i32,
    action_enumerate: i32,
    action_enumerate2: i32,
    action_equals: i32,
    action_equals2: i32,
    action_extends: i32,
    action_get_member: i32,
    action_get_property: i32,
    action_get_time: i32,
    action_get_url: i32,
    action_get_url2: i32,
    action_get_variable: i32,
    action_goto_frame: i32,
    action_goto_frame2: i32,
    action_goto_label: i32,
    action_greater: i32,
    action_if: i32,
    action_implements_op: i32,
    action_increment: i32,
    action_init_array: i32,
    action_init_object: i32,
    action_instance_of: i32,
    action_jump: i32,
    action_less: i32,
    action_less2: i32,
    action_m_b_ascii_to_char: i32,
    action_m_b_char_to_ascii: i32,
    action_m_b_string_extract: i32,
    action_m_b_string_length: i32,
    action_modulo: i32,
    action_multiply: i32,
    action_new_method: i32,
    action_new_object: i32,
    action_next_frame: i32,
    action_not: i32,
    action_or: i32,
    action_play: i32,
    action_pop: i32,
    action_previous_frame: i32,
    action_push: i32,
    action_push_duplicate: i32,
    action_random_number: i32,
    action_remove_sprite: i32,
    action_return: i32,
    action_set_member: i32,
    action_set_property: i32,
    action_set_target: i32,
    action_set_target2: i32,
    action_set_variable: i32,
    action_stack_swap: i32,
    action_start_drag: i32,
    action_stop: i32,
    action_stop_sounds: i32,
    action_store_register: i32,
    action_strict_equals: i32,
    action_string_add: i32,
    action_string_equals: i32,
    action_string_extract: i32,
    action_string_greater: i32,
    action_string_length: i32,
    action_string_less: i32,
    action_subtract: i32,
    action_target_path: i32,
    action_throw: i32,
    action_to_integer: i32,
    action_to_number: i32,
    action_to_string: i32,
    action_toggle_quality: i32,
    action_trace: i32,
    action_try: i32,
    action_type_of: i32,
    action_wait_for_frame: i32,
    action_wait_for_frame2: i32,
    action_with: i32,
    action_unknown: i32,
}

fn check_file(path: &PathBuf, file: String) -> Row {
    match std::fs::read(path) {
        Ok(swf_data) => {
            match swf::read::read_swf(&swf_data[..]) {
                Ok(swf) => {
                    let mut row = Row { file: Some(file), version: swf.header.version, ..Default::default() };

                    for tag in swf.tags {
                        match tag {
                            Tag::ScriptLimits{..} => { row.script_limits += 1; }
                            Tag::ShowFrame => { row.show_frame += 1; },
                            Tag::Protect(_) => { row.protect += 1; },
                            Tag::CsmTextSettings(_) => { row.csm_text_settings += 1; },
                            Tag::DebugId(_) => { row.debug_id += 1; },
                            Tag::DefineBinaryData{..} => { row.define_binary_data += 1; },
                            Tag::DefineBits{..} => { row.define_bits += 1; },
                            Tag::DefineBitsJpeg2{..} => { row.define_bits_jpeg2 += 1; },
                            Tag::DefineBitsJpeg3(_) => { row.define_bits_jpeg3 += 1; },
                            Tag::DefineBitsLossless(_) => { row.define_bits_lossless += 1; },
                            Tag::DefineButton(_) => { row.define_button += 1; },
                            Tag::DefineButton2(_) => { row.define_button2 += 1; },
                            Tag::DefineButtonColorTransform{..} => { row.define_button_color_transform += 1; },
                            Tag::DefineButtonSound(_) => { row.define_button_sound += 1; },
                            Tag::DefineEditText(_) => { row.define_edit_text += 1; },
                            Tag::DefineFont(_) => { row.define_font += 1; },
                            Tag::DefineFont2(_) => { row.define_font2 += 1; },
                            Tag::DefineFont4(_) => { row.define_font4 += 1; },
                            Tag::DefineFontAlignZones{..} => { row.define_font_align_zones += 1; },
                            Tag::DefineFontInfo(_) => { row.define_font_info += 1; },
                            Tag::DefineFontName{..} => { row.define_font_name += 1; },
                            Tag::DefineMorphShape(_) => { row.define_morph_shape += 1; },
                            Tag::DefineScalingGrid{..} => { row.define_scaling_grid += 1; },
                            Tag::DefineShape(_) => { row.define_shape += 1; },
                            Tag::DefineSound(_) => { row.define_sound += 1; },
                            Tag::DefineSprite(_) => { row.define_sprite += 1; },
                            Tag::DefineText(_) => { row.define_text += 1; },
                            Tag::DefineVideoStream(_) => { row.define_video_stream += 1; },
                            Tag::DoAbc(_) => { row.do_abc += 1; },
                            Tag::DoAction(actions) => {
                                row.do_action += 1;
                                let len = actions.len();
                                let mut reader = swf::avm1::read::Reader::new(&actions, swf.header.version);
                                while reader.pos() < len {
                                    match reader.read_action() {
                                        Ok(action) => {
                                            if action.is_some() {
                                                match action.unwrap() {
                                                    Action::Add => { row.action_add += 1; },
                                                    Action::Add2 => { row.action_add2 += 1; },
                                                    Action::And => { row.action_and += 1; },
                                                    Action::AsciiToChar => { row.action_ascii_to_char += 1; },
                                                    Action::BitAnd => { row.action_bit_and += 1; },
                                                    Action::BitLShift => { row.action_bit_l_shift += 1; },
                                                    Action::BitOr => { row.action_bit_or += 1; },
                                                    Action::BitRShift => { row.action_bit_r_shift += 1; },
                                                    Action::BitURShift => { row.action_bit_u_r_shift += 1; },
                                                    Action::BitXor => { row.action_bit_xor += 1; },
                                                    Action::Call => { row.action_call += 1; },
                                                    Action::CallFunction => { row.action_call_function += 1; },
                                                    Action::CallMethod => { row.action_call_method += 1; },
                                                    Action::CastOp => { row.action_cast_op += 1; },
                                                    Action::CharToAscii => { row.action_char_to_ascii += 1; },
                                                    Action::CloneSprite => { row.action_clone_sprite += 1; },
                                                    Action::ConstantPool(_) => { row.action_constant_pool += 1; },
                                                    Action::Decrement => { row.action_decrement += 1; },
                                                    Action::DefineFunction {..} => { row.action_define_function += 1; },
                                                    Action::DefineFunction2(_) => { row.action_define_function2 += 1; },
                                                    Action::DefineLocal => { row.action_define_local += 1; },
                                                    Action::DefineLocal2 => { row.action_define_local2 += 1; },
                                                    Action::Delete => { row.action_delete += 1; },
                                                    Action::Delete2 => { row.action_delete2 += 1; },
                                                    Action::Divide => { row.action_divide += 1; },
                                                    Action::EndDrag => { row.action_end_drag += 1; },
                                                    Action::Enumerate => { row.action_enumerate += 1; },
                                                    Action::Enumerate2 => { row.action_enumerate2 += 1; },
                                                    Action::Equals => { row.action_equals += 1; },
                                                    Action::Equals2 => { row.action_equals2 += 1; },
                                                    Action::Extends => { row.action_extends += 1; },
                                                    Action::GetMember => { row.action_get_member += 1; },
                                                    Action::GetProperty => { row.action_get_property += 1; },
                                                    Action::GetTime => { row.action_get_time += 1; },
                                                    Action::GetUrl {..} => { row.action_get_url += 1; },
                                                    Action::GetUrl2 {..} => { row.action_get_url2 += 1; },
                                                    Action::GetVariable => { row.action_get_variable += 1; },
                                                    Action::GotoFrame(_) => { row.action_goto_frame += 1; },
                                                    Action::GotoFrame2 {..} => { row.action_goto_frame2 += 1; },
                                                    Action::GotoLabel(_) => { row.action_goto_label += 1; },
                                                    Action::Greater => { row.action_greater += 1; },
                                                    Action::If {..} => { row.action_if += 1; },
                                                    Action::ImplementsOp => { row.action_implements_op += 1; },
                                                    Action::Increment => { row.action_increment += 1; },
                                                    Action::InitArray => { row.action_init_array += 1; },
                                                    Action::InitObject => { row.action_init_object += 1; },
                                                    Action::InstanceOf => { row.action_instance_of += 1; },
                                                    Action::Jump {..} => { row.action_jump += 1; },
                                                    Action::Less => { row.action_less += 1; },
                                                    Action::Less2 => { row.action_less2 += 1; },
                                                    Action::MBAsciiToChar => { row.action_m_b_ascii_to_char += 1; },
                                                    Action::MBCharToAscii => { row.action_m_b_char_to_ascii += 1; },
                                                    Action::MBStringExtract => { row.action_m_b_string_extract += 1; },
                                                    Action::MBStringLength => { row.action_m_b_string_length += 1; },
                                                    Action::Modulo => { row.action_modulo += 1; },
                                                    Action::Multiply => { row.action_multiply += 1; },
                                                    Action::NewMethod => { row.action_new_method += 1; },
                                                    Action::NewObject => { row.action_new_object += 1; },
                                                    Action::NextFrame => { row.action_next_frame += 1; },
                                                    Action::Not => { row.action_not += 1; },
                                                    Action::Or => { row.action_or += 1; },
                                                    Action::Play => { row.action_play += 1; },
                                                    Action::Pop => { row.action_pop += 1; },
                                                    Action::PreviousFrame => { row.action_previous_frame += 1; },
                                                    Action::Push(_) => { row.action_push += 1; },
                                                    Action::PushDuplicate => { row.action_push_duplicate += 1; },
                                                    Action::RandomNumber => { row.action_random_number += 1; },
                                                    Action::RemoveSprite => { row.action_remove_sprite += 1; },
                                                    Action::Return => { row.action_return += 1; },
                                                    Action::SetMember => { row.action_set_member += 1; },
                                                    Action::SetProperty => { row.action_set_property += 1; },
                                                    Action::SetTarget(_) => { row.action_set_target += 1; },
                                                    Action::SetTarget2 => { row.action_set_target2 += 1; },
                                                    Action::SetVariable => { row.action_set_variable += 1; },
                                                    Action::StackSwap => { row.action_stack_swap += 1; },
                                                    Action::StartDrag => { row.action_start_drag += 1; },
                                                    Action::Stop => { row.action_stop += 1; },
                                                    Action::StopSounds => { row.action_stop_sounds += 1; },
                                                    Action::StoreRegister(_) => { row.action_store_register += 1; },
                                                    Action::StrictEquals => { row.action_strict_equals += 1; },
                                                    Action::StringAdd => { row.action_string_add += 1; },
                                                    Action::StringEquals => { row.action_string_equals += 1; },
                                                    Action::StringExtract => { row.action_string_extract += 1; },
                                                    Action::StringGreater => { row.action_string_greater += 1; },
                                                    Action::StringLength => { row.action_string_length += 1; },
                                                    Action::StringLess => { row.action_string_less += 1; },
                                                    Action::Subtract => { row.action_subtract += 1; },
                                                    Action::TargetPath => { row.action_target_path += 1; },
                                                    Action::Throw => { row.action_throw += 1; },
                                                    Action::ToInteger => { row.action_to_integer += 1; },
                                                    Action::ToNumber => { row.action_to_number += 1; },
                                                    Action::ToString => { row.action_to_string += 1; },
                                                    Action::ToggleQuality => { row.action_toggle_quality += 1; },
                                                    Action::Trace => { row.action_trace += 1; },
                                                    Action::Try(_) => { row.action_try += 1; },
                                                    Action::TypeOf => { row.action_type_of += 1; },
                                                    Action::WaitForFrame {..} => { row.action_wait_for_frame += 1; },
                                                    Action::WaitForFrame2 {..} => { row.action_wait_for_frame2 += 1; },
                                                    Action::With {..} => { row.action_with += 1; },
                                                    Action::Unknown {..} => { row.action_unknown += 1; },
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            row.error = Some(format!("Failed to parse action: {}", e).to_string());
                                            return row;
                                        },
                                    }
                                }
                            },
                            Tag::DoInitAction{..} => { row.do_init_action += 1; },
                            Tag::EnableDebugger(_) => { row.enable_debugger += 1; },
                            Tag::EnableTelemetry{..} => { row.enable_telemetry += 1; },
                            Tag::End => { row.end += 1; },
                            Tag::Metadata(_) => { row.metadata += 1; },
                            Tag::ImportAssets{..} => { row.import_assets += 1; },
                            Tag::JpegTables(_) => { row.jpeg_tables += 1; },
                            Tag::SetBackgroundColor(_) => { row.set_background_color += 1; },
                            Tag::SetTabIndex{..} => { row.set_tab_index += 1; },
                            Tag::SoundStreamBlock(_) => { row.sound_stream_block += 1; },
                            Tag::SoundStreamHead(_) => { row.sound_stream_head += 1; },
                            Tag::SoundStreamHead2(_) => { row.sound_stream_head2 += 1; },
                            Tag::StartSound(_) => { row.start_sound += 1; },
                            Tag::StartSound2{..} => { row.start_sound2 += 1; },
                            Tag::SymbolClass(_) => { row.symbol_class += 1; },
                            Tag::PlaceObject(_) => { row.place_object += 1; },
                            Tag::RemoveObject(_) => { row.remove_object += 1; },
                            Tag::VideoFrame(_) => { row.video_frame += 1; },
                            Tag::FileAttributes(_) => { row.file_attributes += 1; },
                            Tag::FrameLabel(_) => { row.frame_label += 1; },
                            Tag::DefineSceneAndFrameLabelData(_) => { row.define_scene_and_frame_label_data += 1; },
                            Tag::ProductInfo(_) => { row.product_info += 1; },
                            Tag::Unknown{..} => { row.unknown += 1; },
                            Tag::ExportAssets(_) => { row.export_assets += 1; },
                        };
                    }

                    return row;
                },
                Err(e) => {
                   return Row { file: Some(file), error: Some(format!("Failed to parse swf: {}", e).to_string()), ..Default::default() };
                },
            }
        },
        Err(e) => {
            return Row { file: Some(file), error: Some(format!("Failed to read file: {}", e).to_string()), ..Default::default() }
        },
    };
}

fn main() {
    env_logger::init();

    let mut paths: Vec<PathBuf> = Vec::new();
    let base = Path::new("E:/Dropbox/flashed/samples/Newgrounds.com/Newgrounds.com");

    visit_dirs(
        base,
        &mut paths,
    ).unwrap();

    let mut writer = Writer::from_path("results.csv").unwrap();
    let mut done = 0;
    let mut last_percent = 0;
    let total = paths.len();

    paths.iter().for_each(|path| {
        let file = path.strip_prefix(base).unwrap().to_slash().unwrap();

        match catch_unwind(|| {
            check_file(path, file.clone())
        }) {
            Ok(row) => {
                writer.serialize(row);
            },
            Err(e) => {
                match e.downcast::<String>() {
                    Ok(e) => {
                        log::error!("Panic in {}! {:?}", file, e);
                        writer.serialize(Row { file: Some(file), error: Some(format!("PANIC: {:?}", e).to_string()), ..Default::default() });},
                    Err(e) => {
                        log::error!("Panic in {}! {:?}", file, e);
                        writer.serialize(Row { file: Some(file), error: Some(format!("PANIC: {:?}", e).to_string()), ..Default::default() });},
                }
            },
        }

        done = done + 1;
        let percent = ((done as f32) / (total as f32) * 100.0) as i32;
        if percent != last_percent {
            last_percent = percent;
            log::info!("{} percent...", percent);
            writer.flush();
        }
//        if let Ok(swf_data) =  {
//            for tag in &swf.tags {
//                match tag {
//                    Tag::DefineShape { .. } => {
//                        println!("{:?} has {:?}", path, tag);
//                        return;
//                    }
//                    _ => {}
//                }
//            }
//            writer.flush();
//        }
    });
}

fn run_player(input_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let swf_data = std::fs::read(input_path)?;

    let mut events_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new().with_title("Ruffle");
    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4)
        .with_srgb(true)
        .with_stencil_buffer(8)
        .build_windowed(window_builder, &events_loop)?;
    let audio = audio::CpalAudioBackend::new()?;
    let renderer = GliumRenderBackend::new(windowed_context)?;
    let navigator = navigator::ExternalNavigatorBackend::new(); //TODO: actually implement this backend type
    let display = renderer.display().clone();
    let mut player = Player::new(renderer, audio, navigator, swf_data)?;
    player.set_is_playing(true); // Desktop player will auto-play.

    let logical_size: LogicalSize = (player.movie_width(), player.movie_height()).into();
    let hidpi_factor = display.gl_window().get_hidpi_factor();

    display
        .gl_window()
        .resize(logical_size.to_physical(hidpi_factor));

    display.gl_window().set_inner_size(logical_size);

    let mut mouse_pos = PhysicalPosition::new(0.0, 0.0);
    let mut time = Instant::now();
    loop {
        // Poll UI events
        let mut request_close = false;
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::Resized(logical_size) => {
                        let size = logical_size.to_physical(hidpi_factor);
                        player.set_viewport_dimensions(
                            size.width.ceil() as u32,
                            size.height.ceil() as u32,
                        );
                        player.renderer_mut().set_viewport_dimensions(
                            size.width.ceil() as u32,
                            size.height.ceil() as u32,
                        );
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let position = position.to_physical(hidpi_factor);
                        mouse_pos = position;
                        let event = ruffle_core::PlayerEvent::MouseMove {
                            x: position.x,
                            y: position.y,
                        };
                        player.handle_event(event);
                    }
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state: pressed,
                        ..
                    } => {
                        let event = if pressed == ElementState::Pressed {
                            ruffle_core::PlayerEvent::MouseDown {
                                x: mouse_pos.x,
                                y: mouse_pos.y,
                            }
                        } else {
                            ruffle_core::PlayerEvent::MouseUp {
                                x: mouse_pos.x,
                                y: mouse_pos.y,
                            }
                        };
                        player.handle_event(event);
                    }
                    WindowEvent::CursorLeft { .. } => {
                        player.handle_event(ruffle_core::PlayerEvent::MouseLeft)
                    }
                    WindowEvent::CloseRequested => request_close = true,
                    _ => (),
                }
            }
        });

        if request_close {
            break;
        }

        let new_time = Instant::now();
        let dt = new_time.duration_since(time).as_millis();
        time = new_time;

        player.tick(dt as f64);

        std::thread::sleep(Duration::from_millis(1000 / 60));
    }
    Ok(())
}
