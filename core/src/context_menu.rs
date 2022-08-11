//! Structure holding the temporary state of an open context menu.
//!
//! The context menu items and callbacks set to `object.menu`
//! are stored aside when the menu is open. This way the context menu
//! items work even if the movie changed `object.menu` in the meantime.

use crate::avm1;
use gc_arena::Collect;
use ruffle_types::backend::Backend;
use serde::Serialize;

#[derive(Collect, Default)]
#[collect(no_drop)]
pub struct ContextMenuState<'gc, B: Backend> {
    info: Vec<ContextMenuItem>,
    callbacks: Vec<ContextMenuCallback<'gc, B>>,
}

impl<'gc, B: Backend> ContextMenuState<'gc, B> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, item: ContextMenuItem, callback: ContextMenuCallback<'gc, B>) {
        self.info.push(item);
        self.callbacks.push(callback);
    }
    pub fn info(&self) -> &Vec<ContextMenuItem> {
        &self.info
    }
    pub fn callback(&self, index: usize) -> &ContextMenuCallback<'gc, B> {
        &self.callbacks[index]
    }
}

#[derive(Collect, Clone, Serialize)]
#[collect(require_static)]
pub struct ContextMenuItem {
    pub enabled: bool,
    #[serde(rename = "separatorBefore")]
    pub separator_before: bool,
    pub checked: bool,
    pub caption: String,
}

#[derive(Collect)]
#[collect(no_drop)]
pub enum ContextMenuCallback<'gc, B: Backend> {
    Zoom,
    Quality,
    Play,
    Loop,
    Rewind,
    Forward,
    Back,
    Print,
    Avm1 {
        item: avm1::Object<'gc, B>,
        callback: avm1::Object<'gc, B>,
    },
}
