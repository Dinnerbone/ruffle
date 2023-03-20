//! Gamepad devices

use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::backend::ui::GamepadHandle;
use core::fmt;
use gc_arena::{Collect, GcCell, MutationContext};
use std::cell::{Ref, RefMut};

/// An Object which represents a boxed namespace name.
#[derive(Collect, Clone, Copy)]
#[collect(no_drop)]
pub struct GamepadObject<'gc>(GcCell<'gc, GamepadObjectData<'gc>>);

impl fmt::Debug for GamepadObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GamepadObject")
            .field("ptr", &self.0.as_ptr())
            .finish()
    }
}

// Note that many objects can exist for the same handle
// Nothing more useful than the handle should really be stored here

#[derive(Collect, Clone)]
#[collect(no_drop)]
pub struct GamepadObjectData<'gc> {
    /// All normal script data.
    base: ScriptObjectData<'gc>,

    /// The handle of the gamepad this object represents
    #[collect(require_static)]
    handle: GamepadHandle,
}

impl<'gc> GamepadObject<'gc> {
    /// Box a namespace into an object.
    pub fn from_handle(activation: &mut Activation<'_, 'gc>, handle: GamepadHandle) -> Object<'gc> {
        let class = activation.avm2().classes().gameinputdevice;
        let base = ScriptObjectData::new(class);

        let mut this: Object<'gc> = GamepadObject(GcCell::allocate(
            activation.context.gc_context,
            GamepadObjectData { base, handle },
        ))
        .into();
        this.install_instance_slots(activation);

        this
    }

    pub fn handle(self) -> GamepadHandle {
        return self.0.read().handle;
    }
}

impl<'gc> TObject<'gc> for GamepadObject<'gc> {
    fn base(&self) -> Ref<ScriptObjectData<'gc>> {
        Ref::map(self.0.read(), |read| &read.base)
    }

    fn base_mut(&self, mc: MutationContext<'gc, '_>) -> RefMut<ScriptObjectData<'gc>> {
        RefMut::map(self.0.write(mc), |write| &mut write.base)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc>, Error<'gc>> {
        Ok(Value::Object(Object::from(*self)))
    }

    fn as_gamepad_object(&self) -> Option<Self> {
        Some(*self)
    }
}
