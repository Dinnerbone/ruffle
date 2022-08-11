use crate::display_object::{
    Avm1Button, Avm2Button, Bitmap, EditText, Graphic, MorphShape, MovieClip, Text, Video,
};
use crate::font::Font;
use gc_arena::Collect;
use ruffle_types::backend::audio::SoundHandle;
use ruffle_types::backend::Backend;
use ruffle_types::binary_data::BinaryData;

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub enum Character<'gc, B: Backend> {
    EditText(EditText<'gc, B>),
    Graphic(Graphic<'gc, B>),
    MovieClip(MovieClip<'gc, B>),
    Bitmap(Bitmap<'gc, B>),
    Avm1Button(Avm1Button<'gc, B>),
    Avm2Button(Avm2Button<'gc, B>),
    Font(Font<'gc>),
    MorphShape(MorphShape<'gc, B>),
    Text(Text<'gc, B>),
    Sound(#[collect(require_static)] SoundHandle),
    Video(Video<'gc, B>),
    BinaryData(BinaryData),
}
