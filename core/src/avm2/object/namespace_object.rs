//! Boxed namespaces

use crate::avm2::activation::Activation;
use crate::avm2::names::Namespace;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::cell::{Ref, RefMut};

/// A class instance allocator that allocates namespace objects.
pub fn namespace_allocator<'gc, B: Backend>(
    class: ClassObject<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Object<'gc, B>, Error> {
    let base = ScriptObjectData::new(class);

    Ok(NamespaceObject(GcCell::allocate(
        activation.context.gc_context,
        NamespaceObjectData {
            base,
            namespace: Namespace::public(),
        },
    ))
    .into())
}

/// An Object which represents a boxed namespace name.
#[derive(Collect, Debug, Clone, Copy)]
#[collect(no_drop)]
pub struct NamespaceObject<'gc, B: Backend>(GcCell<'gc, NamespaceObjectData<'gc, B>>);

#[derive(Collect, Debug, Clone)]
#[collect(no_drop)]
pub struct NamespaceObjectData<'gc, B: Backend> {
    /// All normal script data.
    base: ScriptObjectData<'gc, B>,

    /// The namespace name this object is associated with.
    namespace: Namespace<'gc>,
}

impl<'gc, B: Backend> NamespaceObject<'gc, B> {
    /// Box a namespace into an object.
    pub fn from_namespace(
        activation: &mut Activation<'_, 'gc, '_, B>,
        namespace: Namespace<'gc>,
    ) -> Result<Object<'gc, B>, Error> {
        let class = activation.avm2().classes().namespace;
        let base = ScriptObjectData::new(class);

        let mut this: Object<'gc, B> = NamespaceObject(GcCell::allocate(
            activation.context.gc_context,
            NamespaceObjectData { base, namespace },
        ))
        .into();
        this.install_instance_slots(activation);

        class.call_native_init(Some(this), &[], activation)?;

        Ok(this)
    }
}

impl<'gc, B: Backend> TObject<'gc> for NamespaceObject<'gc, B> {
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

    fn to_string(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc, B>, Error> {
        Ok(self.0.read().namespace.as_uri().into())
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc, B>, Error> {
        Ok(self.0.read().namespace.as_uri().into())
    }

    fn as_namespace(&self) -> Option<Ref<Namespace<'gc>>> {
        Some(Ref::map(self.0.read(), |s| &s.namespace))
    }
}
