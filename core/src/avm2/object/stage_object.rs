//! AVM2 object impl for the display hierarchy.

use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::display_object::DisplayObject;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::cell::{Ref, RefMut};

/// A class instance allocator that allocates Stage objects.
pub fn stage_allocator<'gc, B: Backend>(
    class: ClassObject<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Object<'gc, B>, Error> {
    let base = ScriptObjectData::new(class);

    Ok(StageObject(GcCell::allocate(
        activation.context.gc_context,
        StageObjectData {
            base,
            display_object: None,
        },
    ))
    .into())
}

#[derive(Clone, Collect, Debug, Copy)]
#[collect(no_drop)]
pub struct StageObject<'gc, B: Backend>(GcCell<'gc, StageObjectData<'gc, B>>);

#[derive(Clone, Collect, Debug)]
#[collect(no_drop)]
pub struct StageObjectData<'gc, B: Backend> {
    /// The base data common to all AVM2 objects.
    base: ScriptObjectData<'gc, B>,

    /// The associated display object, if one exists.
    display_object: Option<DisplayObject<'gc, B>>,
}

impl<'gc, B: Backend> StageObject<'gc, B> {
    /// Allocate the AVM2 side of a display object intended to be of a given
    /// class's type.
    ///
    /// This function makes no attempt to construct the returned object. You
    /// are responsible for calling the native initializer of the given
    /// class at a later time. Typically, a display object that can contain
    /// movie-constructed children must first allocate itself (using this
    /// function), construct it's children, and then finally initialize itself.
    /// Display objects that do not need to use this flow should use
    /// `for_display_object_childless`.
    pub fn for_display_object(
        activation: &mut Activation<'_, 'gc, '_, B>,
        display_object: DisplayObject<'gc, B>,
        class: ClassObject<'gc, B>,
    ) -> Result<Self, Error> {
        let mut instance = Self(GcCell::allocate(
            activation.context.gc_context,
            StageObjectData {
                base: ScriptObjectData::new(class),
                display_object: Some(display_object),
            },
        ));
        instance.install_instance_slots(activation);

        Ok(instance)
    }

    /// Allocate and construct the AVM2 side of a display object intended to be
    /// of a given class's type.
    ///
    /// This function is intended for display objects that do not have children
    /// and thus do not need to be allocated and initialized in separate phases.
    pub fn for_display_object_childless(
        activation: &mut Activation<'_, 'gc, '_, B>,
        display_object: DisplayObject<'gc, B>,
        class: ClassObject<'gc, B>,
    ) -> Result<Self, Error> {
        let this = Self::for_display_object(activation, display_object, class)?;

        class.call_native_init(Some(this.into()), &[], activation)?;

        Ok(this)
    }

    /// Create a `graphics` object for a given display object.
    pub fn graphics(
        activation: &mut Activation<'_, 'gc, '_, B>,
        display_object: DisplayObject<'gc, B>,
    ) -> Result<Self, Error> {
        let class = activation.avm2().classes().graphics;
        let mut this = Self(GcCell::allocate(
            activation.context.gc_context,
            StageObjectData {
                base: ScriptObjectData::new(class),
                display_object: Some(display_object),
            },
        ));
        this.install_instance_slots(activation);

        class.call_native_init(Some(this.into()), &[], activation)?;

        Ok(this)
    }
}

impl<'gc, B: Backend> TObject<'gc> for StageObject<'gc, B> {
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

    fn as_display_object(&self) -> Option<DisplayObject<'gc, B>> {
        self.0.read().display_object
    }

    fn init_display_object(&self, mc: MutationContext<'gc, '_>, obj: DisplayObject<'gc, B>) {
        self.0.write(mc).display_object = Some(obj);
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc, B>, Error> {
        Ok(Value::Object(Object::from(*self)))
    }
}
