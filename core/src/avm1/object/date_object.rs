use crate::avm1::{Object, ScriptObject, TObject};
use crate::impl_custom_object;
use chrono::{DateTime, Utc};
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::fmt;

#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct DateObject<'gc, B: Backend>(GcCell<'gc, DateObjectData<'gc, B>>);

#[derive(Collect)]
#[collect(no_drop)]
pub struct DateObjectData<'gc, B: Backend> {
    /// The underlying script object.
    base: ScriptObject<'gc, B>,

    /// The DateTime represented by this object
    #[collect(require_static)]
    date_time: Option<DateTime<Utc>>,
}

impl<B: Backend> fmt::Debug for DateObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("DateObject")
            .field("date_time", &this.date_time)
            .finish()
    }
}

impl<'gc, B: Backend> DateObject<'gc, B> {
    pub fn empty(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
    ) -> DateObject<'gc, B> {
        Self::with_date_time(gc_context, proto, None)
    }

    pub fn with_date_time(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
        date_time: Option<DateTime<Utc>>,
    ) -> DateObject<'gc, B> {
        DateObject(GcCell::allocate(
            gc_context,
            DateObjectData {
                base: ScriptObject::object(gc_context, proto),
                date_time,
            },
        ))
    }

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

impl<'gc, B: Backend> TObject<'gc> for DateObject<'gc, B> {
    type B = B;

    impl_custom_object!(B, base {
        bare_object(as_date_object -> DateObject::empty);
    });
}
