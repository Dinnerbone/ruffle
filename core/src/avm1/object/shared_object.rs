use crate::impl_custom_object;
use gc_arena::{Collect, GcCell, MutationContext};

use crate::avm1::{Object, ScriptObject, TObject};
use ruffle_types::backend::Backend;
use std::fmt;

/// A SharedObject
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct SharedObject<'gc, B: Backend>(GcCell<'gc, SharedObjectData<'gc, B>>);

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct SharedObjectData<'gc, B: Backend> {
    /// The underlying script object.
    base: ScriptObject<'gc, B>,

    /// The local name of this shared object
    name: Option<String>,
    // In future this will also handle remote SharedObjects
}

impl<B: Backend> fmt::Debug for SharedObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("SharedObject")
            .field("name", &this.name)
            .finish()
    }
}

impl<'gc, B: Backend> SharedObject<'gc, B> {
    pub fn empty_shared_obj(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
    ) -> Self {
        SharedObject(GcCell::allocate(
            gc_context,
            SharedObjectData {
                base: ScriptObject::object(gc_context, proto),
                name: None,
            },
        ))
    }

    pub fn set_name(&self, gc_context: MutationContext<'gc, '_>, name: String) {
        self.0.write(gc_context).name = Some(name);
    }

    pub fn get_name(&self) -> String {
        self.0
            .read()
            .name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "".to_string())
    }
}

impl<'gc, B: Backend> TObject<'gc> for SharedObject<'gc, B> {
    type B = B;

    impl_custom_object!(B, base {
        bare_object(as_shared_object -> SharedObject::empty_shared_obj);
    });
}
