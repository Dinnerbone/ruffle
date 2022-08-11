//! Object representation for sounds

use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::audio::SoundHandle;
use ruffle_types::backend::Backend;
use std::cell::{Ref, RefMut};

/// A class instance allocator that allocates Sound objects.
pub fn sound_allocator<'gc, B: Backend>(
    class: ClassObject<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Object<'gc, B>, Error> {
    let base = ScriptObjectData::new(class);

    Ok(SoundObject(GcCell::allocate(
        activation.context.gc_context,
        SoundObjectData { base, sound: None },
    ))
    .into())
}

#[derive(Clone, Collect, Debug, Copy)]
#[collect(no_drop)]
pub struct SoundObject<'gc, B: Backend>(GcCell<'gc, SoundObjectData<'gc, B>>);

#[derive(Clone, Collect, Debug)]
#[collect(no_drop)]
pub struct SoundObjectData<'gc, B: Backend> {
    /// Base script object
    base: ScriptObjectData<'gc, B>,

    /// The sound this object holds.
    #[collect(require_static)]
    sound: Option<SoundHandle>,
}

impl<'gc, B: Backend> SoundObject<'gc, B> {
    /// Convert a bare sound into it's object representation.
    ///
    /// In AS3, library sounds are accessed through subclasses of `Sound`. As a
    /// result, this needs to take the subclass so that the returned object is
    /// an instance of the correct class.
    pub fn from_sound(
        activation: &mut Activation<'_, 'gc, '_, B>,
        class: ClassObject<'gc, B>,
        sound: SoundHandle,
    ) -> Result<Object<'gc, B>, Error> {
        let base = ScriptObjectData::new(class);

        let mut sound_object: Object<'gc, B> = SoundObject(GcCell::allocate(
            activation.context.gc_context,
            SoundObjectData {
                base,
                sound: Some(sound),
            },
        ))
        .into();
        sound_object.install_instance_slots(activation);

        class.call_native_init(Some(sound_object), &[], activation)?;

        Ok(sound_object)
    }
}

impl<'gc, B: Backend> TObject<'gc> for SoundObject<'gc, B> {
    type B = B;

    fn base(&self) -> Ref<ScriptObjectData<'gc, B>> {
        Ref::map(self.0.read(), |read| &read.base)
    }

    fn base_mut(&self, mc: MutationContext<'gc, '_>) -> RefMut<ScriptObjectData<'gc, B>> {
        RefMut::map(self.0.write(mc), |write| &mut write.base)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc, B>, Error> {
        Ok(Object::from(*self).into())
    }

    fn as_sound(self) -> Option<SoundHandle> {
        self.0.read().sound
    }

    /// Associate the object with a particular sound handle.
    ///
    /// This does nothing if the object is not a sound.
    fn set_sound(self, mc: MutationContext<'gc, '_>, sound: SoundHandle) {
        self.0.write(mc).sound = Some(sound);
    }
}
