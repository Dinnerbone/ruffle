//! Boxed QNames

use crate::avm2::activation::Activation;
use crate::avm2::names::QName;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::cell::{Ref, RefMut};

/// A class instance allocator that allocates QName objects.
pub fn qname_allocator<'gc, B: Backend>(
    class: ClassObject<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Object<'gc, B>, Error> {
    let base = ScriptObjectData::new(class);

    Ok(QNameObject(GcCell::allocate(
        activation.context.gc_context,
        QNameObjectData { base, qname: None },
    ))
    .into())
}

/// An Object which represents a boxed QName.
#[derive(Collect, Debug, Clone, Copy)]
#[collect(no_drop)]
pub struct QNameObject<'gc, B: Backend>(GcCell<'gc, QNameObjectData<'gc, B>>);

#[derive(Collect, Debug, Clone)]
#[collect(no_drop)]
pub struct QNameObjectData<'gc, B: Backend> {
    /// All normal script data.
    base: ScriptObjectData<'gc, B>,

    /// The QName name this object is associated with.
    qname: Option<QName<'gc>>,
}

impl<'gc, B: Backend> QNameObject<'gc, B> {
    /// Box a QName into an object.
    pub fn from_qname(
        activation: &mut Activation<'_, 'gc, '_, B>,
        qname: QName<'gc>,
    ) -> Result<Object<'gc, B>, Error> {
        let class = activation.avm2().classes().qname;
        let base = ScriptObjectData::new(class);

        let mut this: Object<'gc, B> = QNameObject(GcCell::allocate(
            activation.context.gc_context,
            QNameObjectData {
                base,
                qname: Some(qname),
            },
        ))
        .into();
        this.install_instance_slots(activation);

        class.call_native_init(Some(this), &[], activation)?;

        Ok(this)
    }

    pub fn qname(&self) -> Option<Ref<QName<'gc>>> {
        let read = self.0.read();
        read.qname.as_ref()?;

        Some(Ref::map(read, |r| r.qname.as_ref().unwrap()))
    }

    pub fn init_qname(self, mc: MutationContext<'gc, '_>, qname: QName<'gc>) {
        self.0.write(mc).qname = Some(qname);
    }
}

impl<'gc, B: Backend> TObject<'gc> for QNameObject<'gc, B> {
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
        Ok(Value::Object(Object::from(*self)))
    }

    fn as_qname_object(self) -> Option<QNameObject<'gc, B>> {
        Some(self)
    }
}
