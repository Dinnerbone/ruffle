use crate::custom_event::RuffleEvent;
use anyhow::{Context, Error};
use arboard::Clipboard;
use gilrs::{EventType, Gilrs};
use rfd::{MessageButtons, MessageDialog, MessageLevel};
use ruffle_core::backend::ui::{FullscreenError, GamepadHandle, MouseCursor, UiBackend};
use std::rc::Rc;
use tracing::error;
use winit::event_loop::EventLoopProxy;
use winit::window::{Fullscreen, Window};

pub struct DesktopUiBackend {
    window: Rc<Window>,
    cursor_visible: bool,
    clipboard: Clipboard,
    gilrs: Option<Gilrs>,
    event_loop: EventLoopProxy<RuffleEvent>,
}

impl DesktopUiBackend {
    pub fn new(window: Rc<Window>, event_loop: EventLoopProxy<RuffleEvent>) -> Result<Self, Error> {
        let gilrs = match Gilrs::new() {
            Ok(gilrs) => Some(gilrs),
            Err(e) => {
                tracing::warn!("Gamepad support not available: {e}");
                None
            }
        };
        Ok(Self {
            window,
            cursor_visible: true,
            clipboard: Clipboard::new().context("Couldn't get platform clipboard")?,
            gilrs,
            event_loop,
        })
    }

    pub fn poll_gamepad_events(&mut self) {
        if let Some(gilrs) = &mut self.gilrs {
            while let Some(event) = gilrs.next_event() {
                match &event.event {
                    EventType::Connected => {
                        let _ = self.event_loop.send_event(RuffleEvent::GamepadChanged {
                            added: true,
                            handle: GamepadHandle(event.id.into()),
                        });
                    }
                    EventType::Disconnected => {
                        let _ = self.event_loop.send_event(RuffleEvent::GamepadChanged {
                            added: false,
                            handle: GamepadHandle(event.id.into()),
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

// TODO: Move link to https://ruffle.rs/faq or similar
const UNSUPPORTED_CONTENT_MESSAGE: &str = "\
The Ruffle emulator may not yet fully support all of ActionScript 3 used by this content.
Some parts of the content may not work as expected.

See the following link for more info:
https://github.com/ruffle-rs/ruffle/wiki/Frequently-Asked-Questions-For-Users";

const DOWNLOAD_FAILED_MESSAGE: &str = "Ruffle failed to open or download this file.";

impl UiBackend for DesktopUiBackend {
    fn mouse_visible(&self) -> bool {
        self.cursor_visible
    }

    fn set_mouse_visible(&mut self, visible: bool) {
        self.window.set_cursor_visible(visible);
        self.cursor_visible = visible;
    }

    fn set_mouse_cursor(&mut self, cursor: MouseCursor) {
        use winit::window::CursorIcon;
        let icon = match cursor {
            MouseCursor::Arrow => CursorIcon::Arrow,
            MouseCursor::Hand => CursorIcon::Hand,
            MouseCursor::IBeam => CursorIcon::Text,
            MouseCursor::Grab => CursorIcon::Grab,
        };
        self.window.set_cursor_icon(icon);
    }

    fn set_clipboard_content(&mut self, content: String) {
        if let Err(e) = self.clipboard.set_text(content) {
            error!("Couldn't set clipboard contents: {:?}", e);
        }
    }

    fn set_fullscreen(&mut self, is_full: bool) -> Result<(), FullscreenError> {
        self.window.set_fullscreen(if is_full {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        });
        Ok(())
    }

    fn display_unsupported_message(&self) {
        let dialog = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Ruffle - Unsupported content")
            .set_description(UNSUPPORTED_CONTENT_MESSAGE)
            .set_buttons(MessageButtons::Ok);
        dialog.show();
    }

    fn display_root_movie_download_failed_message(&self) {
        let dialog = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Ruffle - Load failed")
            .set_description(DOWNLOAD_FAILED_MESSAGE)
            .set_buttons(MessageButtons::Ok);
        dialog.show();
    }

    fn message(&self, message: &str) {
        let dialog = MessageDialog::new()
            .set_level(MessageLevel::Info)
            .set_title("Ruffle")
            .set_description(message)
            .set_buttons(MessageButtons::Ok);
        dialog.show();
    }

    // Unused on desktop
    fn open_virtual_keyboard(&self) {}

    fn supports_gamepads(&self) -> bool {
        self.gilrs.is_some()
    }
}
