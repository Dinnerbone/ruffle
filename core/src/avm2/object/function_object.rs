//! Function object impl

use crate::avm2::activation::Activation;
use crate::avm2::function::Executable;
use crate::avm2::method::Method;
use crate::avm2::object::script_object::{ScriptObject, ScriptObjectData};
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::scope::ScopeChain;
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::cell::{Ref, RefMut};

/// An Object which can be called to execute its function code.
#[derive(Collect, Debug, Clone, Copy)]
#[collect(no_drop)]
pub struct FunctionObject<'gc, B: Backend>(GcCell<'gc, FunctionObjectData<'gc, B>>);

#[derive(Collect, Debug, Clone)]
#[collect(no_drop)]
pub struct FunctionObjectData<'gc, B: Backend> {
    /// Base script object
    base: ScriptObjectData<'gc, B>,

    /// Executable code
    exec: Executable<'gc, B>,

    /// Attached prototype (note: not the same thing as base object's proto)
    prototype: Option<Object<'gc, B>>,
}

impl<'gc, B: Backend> FunctionObject<'gc, B> {
    /// Construct a function from an ABC method and the current closure scope.
    ///
    /// This associated constructor will also create and initialize an empty
    /// `Object` prototype for the function.
    pub fn from_function(
        activation: &mut Activation<'_, 'gc, '_, B>,
        method: Method<'gc, B>,
        scope: ScopeChain<'gc, B>,
    ) -> Result<FunctionObject<'gc, B>, Error> {
        let this = Self::from_method(activation, method, scope, None, None);
        let es3_proto = ScriptObject::custom_object(
            activation.context.gc_context,
            // TODO: is this really a class-less object?
            // (also: how much of "ES3 class-less object" is even true?)
            None,
            Some(activation.avm2().classes().object.prototype()),
        );

        this.0.write(activation.context.gc_context).prototype = Some(es3_proto);

        Ok(this)
    }

    /// Construct a method from an ABC method and the current closure scope.
    ///
    /// The given `reciever`, if supplied, will override any user-specified
    /// `this` parameter.
    pub fn from_method(
        activation: &mut Activation<'_, 'gc, '_, B>,
        method: Method<'gc, B>,
        scope: ScopeChain<'gc, B>,
        receiver: Option<Object<'gc, B>>,
        subclass_object: Option<ClassObject<'gc, B>>,
    ) -> FunctionObject<'gc, B> {
        let fn_class = activation.avm2().classes().function;
        let exec = Executable::from_method(method, scope, receiver, subclass_object);

        FunctionObject(GcCell::allocate(
            activation.context.gc_context,
            FunctionObjectData {
                base: ScriptObjectData::new(fn_class),
                exec,
                prototype: None,
            },
        ))
    }

    pub fn prototype(&self) -> Option<Object<'gc, B>> {
        self.0.read().prototype
    }

    pub fn set_prototype(&self, proto: Object<'gc, B>, mc: MutationContext<'gc, '_>) {
        self.0.write(mc).prototype = Some(proto);
    }
}

impl<'gc, B: Backend> TObject<'gc> for FunctionObject<'gc, B> {
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
        Ok("function Function() {}".into())
    }

    fn to_locale_string(&self, mc: MutationContext<'gc, '_>) -> Result<Value<'gc, B>, Error> {
        self.to_string(mc)
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc, B>, Error> {
        Ok(Value::Object(Object::from(*self)))
    }

    fn as_executable(&self) -> Option<Ref<Executable<'gc, B>>> {
        Some(Ref::map(self.0.read(), |r| &r.exec))
    }

    fn as_function_object(&self) -> Option<FunctionObject<'gc, B>> {
        Some(*self)
    }

    fn call(
        self,
        receiver: Option<Object<'gc, B>>,
        arguments: &[Value<'gc, B>],
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Result<Value<'gc, B>, Error> {
        self.0
            .read()
            .exec
            .exec(receiver, arguments, activation, self.into())
    }

    fn construct(
        self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        arguments: &[Value<'gc, B>],
    ) -> Result<Object<'gc, B>, Error> {
        let prototype = self.prototype().unwrap();

        let instance =
            ScriptObject::custom_object(activation.context.gc_context, None, Some(prototype));

        self.call(Some(instance), arguments, activation)?;

        Ok(instance)
    }
}
