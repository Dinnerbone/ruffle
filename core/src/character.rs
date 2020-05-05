use crate::backend::audio::SoundHandle;
use crate::display_object::{Bitmap, Button, EditText, Graphic, MorphShape, MovieClip, Text};
use crate::font::Font;
use crate::backend::Backends;

#[derive(Clone)]
pub enum Character<'gc, B: Backends> {
    EditText(EditText<'gc, B>),
    Graphic(Graphic<'gc, B>),
    MovieClip(MovieClip<'gc, B>),
    Bitmap(Bitmap<'gc, B>),
    Button(Button<'gc, B>),
    Font(Font<'gc, B>),
    MorphShape(MorphShape<'gc, B>),
    Text(Text<'gc, B>),
    Sound(SoundHandle),
}

unsafe impl<'gc, B: Backends> gc_arena::Collect for Character<'gc, B> {
    #[inline]
    fn trace(&self, cc: gc_arena::CollectionContext) {
        match self {
            Character::EditText(c) => c.trace(cc),
            Character::Graphic(c) => c.trace(cc),
            Character::MovieClip(c) => c.trace(cc),
            Character::Bitmap(c) => c.trace(cc),
            Character::Button(c) => c.trace(cc),
            Character::Font(c) => c.trace(cc),
            Character::MorphShape(c) => c.trace(cc),
            Character::Text(c) => c.trace(cc),
            Character::Sound(c) => c.trace(cc),
        }
    }
}
