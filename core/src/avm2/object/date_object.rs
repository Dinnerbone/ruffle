use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::{Hint, Value};
use crate::avm2::Error;
use chrono::{DateTime, Utc};
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::cell::{Ref, RefMut};

/// A class instance allocator that allocates Date objects.
pub fn date_allocator<'gc, B: Backend>(
    class: ClassObject<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Object<'gc, B>, Error> {
    let base = ScriptObjectData::new(class);

    Ok(DateObject(GcCell::allocate(
        activation.context.gc_context,
        DateObjectData {
            base,
            date_time: None,
        },
    ))
    .into())
}
#[derive(Clone, Collect, Debug, Copy)]
#[collect(no_drop)]
pub struct DateObject<'gc, B: Backend>(GcCell<'gc, DateObjectData<'gc, B>>);

impl<'gc, B: Backend> DateObject<'gc, B> {
    pub fn date_time(self) -> Option<DateTime<Utc>> {
        self.0.read().date_time
    }

    pub fn set_date_time(
        self,
        gc_context: MutationContext<'gc, '_>,
        date_time: Option<DateTime<Utc>>,
    ) {
        self.0.write(gc_context).date_time = date_time;
    }
}

#[derive(Clone, Collect, Debug)]
#[collect(no_drop)]
pub struct DateObjectData<'gc, B: Backend> {
    /// Base script object
    base: ScriptObjectData<'gc, B>,

    #[collect(require_static)]
    date_time: Option<DateTime<Utc>>,
}

impl<'gc, B: Backend> TObject<'gc> for DateObject<'gc, B> {
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
        if let Some(date) = self.date_time() {
            Ok((date.timestamp_millis() as f64).into())
        } else {
            Ok(f64::NAN.into())
        }
    }

    fn default_hint(&self) -> Hint {
        Hint::String
    }

    fn as_date_object(&self) -> Option<DateObject<'gc, B>> {
        Some(*self)
    }
}
