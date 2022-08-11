use crate::avm2::activation::Activation;
use crate::avm2::bytearray::ByteArrayStorage;
use crate::avm2::names::Multiname;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::cell::{Ref, RefMut};

/// A class instance allocator that allocates ByteArray objects.
pub fn bytearray_allocator<'gc, B: Backend>(
    class: ClassObject<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Object<'gc, B>, Error> {
    let base = ScriptObjectData::new(class);

    Ok(ByteArrayObject(GcCell::allocate(
        activation.context.gc_context,
        ByteArrayObjectData {
            base,
            storage: ByteArrayStorage::new(),
        },
    ))
    .into())
}

#[derive(Clone, Collect, Debug, Copy)]
#[collect(no_drop)]
pub struct ByteArrayObject<'gc, B: Backend>(GcCell<'gc, ByteArrayObjectData<'gc, B>>);

#[derive(Clone, Collect, Debug)]
#[collect(no_drop)]
pub struct ByteArrayObjectData<'gc, B: Backend> {
    /// Base script object
    base: ScriptObjectData<'gc, B>,

    storage: ByteArrayStorage,
}

impl<'gc, B: Backend> ByteArrayObject<'gc, B> {
    pub fn from_storage(
        activation: &mut Activation<'_, 'gc, '_, B>,
        bytes: ByteArrayStorage,
    ) -> Result<Object<'gc, B>, Error> {
        let class = activation.avm2().classes().bytearray;
        let base = ScriptObjectData::new(class);

        let mut instance: Object<'gc, B> = ByteArrayObject(GcCell::allocate(
            activation.context.gc_context,
            ByteArrayObjectData {
                base,
                storage: bytes,
            },
        ))
        .into();
        instance.install_instance_slots(activation);

        class.call_native_init(Some(instance), &[], activation)?;

        Ok(instance)
    }
}

impl<'gc, B: Backend> TObject<'gc> for ByteArrayObject<'gc, B> {
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

    fn get_property_local(
        self,
        name: &Multiname<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Result<Value<'gc, B>, Error> {
        let read = self.0.read();

        if name.contains_public_namespace() {
            if let Some(name) = name.local_name() {
                if let Ok(index) = name.parse::<usize>() {
                    return Ok(if let Some(val) = read.storage.get(index) {
                        Value::Unsigned(val as u32)
                    } else {
                        Value::Undefined
                    });
                }
            }
        }

        read.base.get_property_local(name, activation)
    }

    fn set_property_local(
        self,
        name: &Multiname<'gc>,
        value: Value<'gc, B>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Result<(), Error> {
        let mut write = self.0.write(activation.context.gc_context);

        if name.contains_public_namespace() {
            if let Some(name) = name.local_name() {
                if let Ok(index) = name.parse::<usize>() {
                    write
                        .storage
                        .set(index, value.coerce_to_u32(activation)? as u8);

                    return Ok(());
                }
            }
        }

        write.base.set_property_local(name, value, activation)
    }

    fn init_property_local(
        self,
        name: &Multiname<'gc>,
        value: Value<'gc, B>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Result<(), Error> {
        let mut write = self.0.write(activation.context.gc_context);

        if name.contains_public_namespace() {
            if let Some(name) = name.local_name() {
                if let Ok(index) = name.parse::<usize>() {
                    write
                        .storage
                        .set(index, value.coerce_to_u32(activation)? as u8);

                    return Ok(());
                }
            }
        }

        write.base.init_property_local(name, value, activation)
    }

    fn delete_property_local(
        self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: &Multiname<'gc>,
    ) -> Result<bool, Error> {
        if name.contains_public_namespace() {
            if let Some(name) = name.local_name() {
                if let Ok(index) = name.parse::<usize>() {
                    self.0
                        .write(activation.context.gc_context)
                        .storage
                        .delete(index);
                    return Ok(true);
                }
            }
        }

        Ok(self
            .0
            .write(activation.context.gc_context)
            .base
            .delete_property_local(name))
    }

    fn has_own_property(self, name: &Multiname<'gc>) -> bool {
        if name.contains_public_namespace() {
            if let Some(name) = name.local_name() {
                if let Ok(index) = name.parse::<usize>() {
                    return self.0.read().storage.get(index).is_some();
                }
            }
        }

        self.0.read().base.has_own_property(name)
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc, B>, Error> {
        Ok(Value::Object(Object::from(*self)))
    }

    fn as_bytearray(&self) -> Option<Ref<ByteArrayStorage>> {
        Some(Ref::map(self.0.read(), |d| &d.storage))
    }

    fn as_bytearray_mut(&self, mc: MutationContext<'gc, '_>) -> Option<RefMut<ByteArrayStorage>> {
        Some(RefMut::map(self.0.write(mc), |d| &mut d.storage))
    }

    fn as_bytearray_object(&self) -> Option<ByteArrayObject<'gc, B>> {
        Some(*self)
    }
}
