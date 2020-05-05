//! Return value enum

use crate::avm1::activation::Activation;
use crate::avm1::{Avm1, Error, Object, Value};
use crate::context::UpdateContext;
use gc_arena::{Collect, GcCell};
use std::fmt;
use crate::backend::Backends;

/// Represents the return value of a function call.
///
/// Since function calls can result in invoking native code or adding a new
/// activation onto the AVM stack, you need another type to represent how the
/// return value will be delivered to you.
///
/// This function contains a handful of utility methods for deciding what to do
/// with a given value regardless of how it is delivered to the calling
/// function.
///
/// It is `must_use` - failing to use a return value is a compiler warning. We
/// provide a helper function specifically to indicate that you aren't
/// interested in the result of a call.
#[must_use = "Return values must be used"]
#[derive(Clone)]
pub enum ReturnValue<'gc, B> {
    /// Indicates that the return value is available immediately.
    Immediate(Value<'gc, B>),

    /// Indicates that the return value is the result of a given user-defined
    /// function call. The activation record returned is the frame that needs
    /// to return to get your value.
    ResultOf(GcCell<'gc, Activation<'gc, B>>),
}

unsafe impl<'gc, B: Backends> Collect for ReturnValue<'gc, B> {
    #[inline]
    fn trace(&self, cc: gc_arena::CollectionContext) {
        use ReturnValue::*;

        match self {
            Immediate(value) => value.trace(cc),
            ResultOf(frame) => frame.trace(cc),
        }
    }
}

impl<B: Backends> PartialEq for ReturnValue<'_, B> {
    fn eq(&self, other: &Self) -> bool {
        use ReturnValue::*;

        match (self, other) {
            (Immediate(val1), Immediate(val2)) => val1 == val2,
            (ResultOf(frame1), ResultOf(frame2)) => GcCell::ptr_eq(*frame1, *frame2),
            _ => false,
        }
    }
}

impl<B: Backends> fmt::Debug for ReturnValue<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ReturnValue::*;

        match self {
            Immediate(val) => write!(f, "Immediate({:?})", val),
            ResultOf(_frame) => write!(f, "ResultOf(<activation frame>)"),
        }
    }
}

impl<'gc, B: Backends> ReturnValue<'gc, B> {
    /// Mark a given return value as intended to be pushed onto the stack.
    ///
    /// The natural result of a stack frame retiring is to be pushed, so this
    /// only ensures that Immediate values are pushed.
    pub fn push(self, avm: &mut Avm1<'gc, B>) {
        use ReturnValue::*;

        match self {
            Immediate(val) => avm.push(val),
            ResultOf(_frame) => {}
        };
    }

    /// Force a return value to resolve on the Rust stack by recursing back
    /// into the AVM.
    pub fn resolve(
        self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
    ) -> Result<Value<'gc, B>, Error> {
        use ReturnValue::*;

        match self {
            Immediate(val) => Ok(val),
            ResultOf(frame) => {
                avm.run_current_frame(context, frame)?;

                Ok(avm.pop())
            }
        }
    }

    pub fn is_immediate(&self) -> bool {
        use ReturnValue::*;

        if let Immediate(_v) = self {
            true
        } else {
            false
        }
    }

    /// Panic if a value is not immediate.
    ///
    /// This should only be used in test assertions.
    #[cfg(test)]
    pub fn unwrap_immediate(self) -> Value<'gc, B> {
        use ReturnValue::*;

        match self {
            Immediate(val) => val,
            _ => panic!("Unwrapped a non-immediate return value"),
        }
    }
}

impl<'gc, B: Backends> From<Value<'gc, B>> for ReturnValue<'gc, B> {
    fn from(val: Value<'gc, B>) -> Self {
        ReturnValue::Immediate(val)
    }
}

impl<'gc, B: Backends> From<String> for ReturnValue<'gc, B> {
    fn from(string: String) -> Self {
        ReturnValue::Immediate(Value::String(string))
    }
}

impl<'gc, B: Backends> From<&str> for ReturnValue<'gc, B> {
    fn from(string: &str) -> Self {
        ReturnValue::Immediate(Value::String(string.to_owned()))
    }
}

impl<'gc, B: Backends> From<bool> for ReturnValue<'gc, B> {
    fn from(value: bool) -> Self {
        ReturnValue::Immediate(Value::Bool(value))
    }
}

impl<'gc, T, B: Backends> From<T> for ReturnValue<'gc, B>
where
    Object<'gc, B>: From<T>,
{
    fn from(value: T) -> Self {
        ReturnValue::Immediate(Value::Object(Object::from(value)))
    }
}

impl<'gc, B: Backends> From<f64> for ReturnValue<'gc, B> {
    fn from(value: f64) -> Self {
        ReturnValue::Immediate(Value::Number(value))
    }
}

impl<'gc, B: Backends> From<f32> for ReturnValue<'gc, B> {
    fn from(value: f32) -> Self {
        ReturnValue::Immediate(Value::Number(f64::from(value)))
    }
}

impl<'gc, B: Backends> From<u8> for ReturnValue<'gc, B> {
    fn from(value: u8) -> Self {
        ReturnValue::Immediate(Value::Number(f64::from(value)))
    }
}

impl<'gc, B: Backends> From<i32> for ReturnValue<'gc, B> {
    fn from(value: i32) -> Self {
        ReturnValue::Immediate(Value::Number(f64::from(value)))
    }
}

impl<'gc, B: Backends> From<u32> for ReturnValue<'gc, B> {
    fn from(value: u32) -> Self {
        ReturnValue::Immediate(Value::Number(f64::from(value)))
    }
}

impl<'gc, B: Backends> From<GcCell<'gc, Activation<'gc, B>>> for ReturnValue<'gc, B> {
    fn from(frame: GcCell<'gc, Activation<'gc, B>>) -> Self {
        ReturnValue::ResultOf(frame)
    }
}
