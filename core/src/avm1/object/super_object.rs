//! Special object that implements `super`

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::function::ExecutionReason;
use crate::avm1::object::{search_prototype, ExecutionName};
use crate::avm1::property::Attribute;
use crate::avm1::{AvmString, Object, ObjectPtr, ScriptObject, TObject, Value};
use crate::display_object::DisplayObject;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implementation of the `super` object in AS2.
///
/// A `SuperObject` references all data from another object, but with one layer
/// of prototyping removed. It's as if the given object had been constructed
/// with its parent class.
#[derive(Copy, Clone, Collect, Debug)]
#[collect(no_drop)]
pub struct SuperObject<'gc, B: Backend>(GcCell<'gc, SuperObjectData<'gc, B>>);

#[derive(Clone, Collect, Debug)]
#[collect(no_drop)]
pub struct SuperObjectData<'gc, B: Backend> {
    /// The object present as `this` throughout the superchain.
    this: Object<'gc, B>,

    /// The prototype depth of the currently-executing method.
    depth: u8,
}

impl<'gc, B: Backend> SuperObject<'gc, B> {
    /// Construct a `super` for an incoming stack frame.
    pub fn new(
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
        depth: u8,
    ) -> Self {
        Self(GcCell::allocate(
            activation.context.gc_context,
            SuperObjectData { this, depth },
        ))
    }

    pub fn this(&self) -> Object<'gc, B> {
        self.0.read().this
    }

    fn base_proto(&self, activation: &mut Activation<'_, 'gc, '_, B>) -> Object<'gc, B> {
        let read = self.0.read();
        let depth = read.depth;
        let mut proto = read.this;
        for _ in 0..depth {
            proto = proto.proto(activation).coerce_to_object(activation);
        }
        proto
    }
}

impl<'gc, B: Backend> TObject<'gc> for SuperObject<'gc, B> {
    type B = B;

    fn get_local_stored(
        &self,
        _name: impl Into<AvmString<'gc>>,
        _activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Option<Value<'gc, B>> {
        None
    }

    fn set_local(
        &self,
        _name: AvmString<'gc>,
        _value: Value<'gc, B>,
        _activation: &mut Activation<'_, 'gc, '_, B>,
        _this: Object<'gc, B>,
    ) -> Result<(), Error<'gc, B>> {
        //TODO: What happens if you set `super.__proto__`?
        Ok(())
    }

    fn call(
        &self,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
        _this: Value<'gc, B>,
        args: &[Value<'gc, B>],
    ) -> Result<Value<'gc, B>, Error<'gc, B>> {
        let constructor = self
            .base_proto(activation)
            .get("__constructor__", activation)?
            .coerce_to_object(activation);
        match constructor.as_executable() {
            Some(exec) => exec.exec(
                ExecutionName::Dynamic(name),
                activation,
                self.0.read().this.into(),
                self.0.read().depth + 1,
                args,
                ExecutionReason::FunctionCall,
                constructor,
            ),
            None => Ok(Value::Undefined),
        }
    }

    fn call_method(
        &self,
        name: AvmString<'gc>,
        args: &[Value<'gc, B>],
        activation: &mut Activation<'_, 'gc, '_, B>,
        reason: ExecutionReason,
    ) -> Result<Value<'gc, B>, Error<'gc, B>> {
        let this = self.0.read().this;
        let (method, depth) =
            match search_prototype(self.proto(activation), name, activation, this)? {
                Some((Value::Object(method), depth)) => (method, depth),
                _ => return Ok(Value::Undefined),
            };

        match method.as_executable() {
            Some(exec) => exec.exec(
                ExecutionName::Dynamic(name),
                activation,
                this.into(),
                self.0.read().depth + depth + 1,
                args,
                reason,
                method,
            ),
            None => method.call(name, activation, this.into(), args),
        }
    }

    fn getter(
        &self,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Option<Object<'gc, B>> {
        self.0.read().this.getter(name, activation)
    }

    fn setter(
        &self,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Option<Object<'gc, B>> {
        self.0.read().this.setter(name, activation)
    }

    fn create_bare_object(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
    ) -> Result<Object<'gc, B>, Error<'gc, B>> {
        if let Value::Object(proto) = self.proto(activation) {
            proto.create_bare_object(activation, this)
        } else {
            // TODO: What happens when you `new super` but there's no
            // super? Is this code even reachable?!
            self.0.read().this.create_bare_object(activation, this)
        }
    }

    fn delete(&self, _activation: &mut Activation<'_, 'gc, '_, B>, _name: AvmString<'gc>) -> bool {
        //`super` cannot have properties deleted from it
        false
    }

    fn proto(&self, activation: &mut Activation<'_, 'gc, '_, B>) -> Value<'gc, B> {
        self.base_proto(activation).proto(activation)
    }

    fn define_value(
        &self,
        _gc_context: MutationContext<'gc, '_>,
        _name: impl Into<AvmString<'gc>>,
        _value: Value<'gc, B>,
        _attributes: Attribute,
    ) {
        //`super` cannot have values defined on it
    }

    fn set_attributes(
        &self,
        _gc_context: MutationContext<'gc, '_>,
        _name: Option<AvmString<'gc>>,
        _set_attributes: Attribute,
        _clear_attributes: Attribute,
    ) {
        //TODO: Does ASSetPropFlags work on `super`? What would it even work on?
    }

    fn add_property(
        &self,
        _gc_context: MutationContext<'gc, '_>,
        _name: AvmString<'gc>,
        _get: Object<'gc, B>,
        _set: Option<Object<'gc, B>>,
        _attributes: Attribute,
    ) {
        //`super` cannot have properties defined on it
    }

    fn add_property_with_case(
        &self,
        _activation: &mut Activation<'_, 'gc, '_, B>,
        _name: AvmString<'gc>,
        _get: Object<'gc, B>,
        _set: Option<Object<'gc, B>>,
        _attributes: Attribute,
    ) {
        //`super` cannot have properties defined on it
    }

    fn call_watcher(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
        value: &mut Value<'gc, B>,
        this: Object<'gc, B>,
    ) -> Result<(), Error<'gc, B>> {
        self.0
            .read()
            .this
            .call_watcher(activation, name, value, this)
    }

    fn watch(
        &self,
        _activation: &mut Activation<'_, 'gc, '_, B>,
        _name: AvmString<'gc>,
        _callback: Object<'gc, B>,
        _user_data: Value<'gc, B>,
    ) {
        //`super` cannot have properties defined on it
    }

    fn unwatch(&self, _activation: &mut Activation<'_, 'gc, '_, B>, _name: AvmString<'gc>) -> bool {
        //`super` cannot have properties defined on it
        false
    }

    fn has_property(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        self.0.read().this.has_property(activation, name)
    }

    fn has_own_property(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        self.0.read().this.has_own_property(activation, name)
    }

    fn has_own_virtual(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        self.0.read().this.has_own_virtual(activation, name)
    }

    fn is_property_enumerable(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        self.0.read().this.is_property_enumerable(activation, name)
    }

    fn get_keys(&self, _activation: &mut Activation<'_, 'gc, '_, B>) -> Vec<AvmString<'gc>> {
        vec![]
    }

    fn length(&self, _activation: &mut Activation<'_, 'gc, '_, B>) -> Result<i32, Error<'gc, B>> {
        Ok(0)
    }

    fn set_length(
        &self,
        _activation: &mut Activation<'_, 'gc, '_, B>,
        _length: i32,
    ) -> Result<(), Error<'gc, B>> {
        Ok(())
    }

    fn has_element(&self, _activation: &mut Activation<'_, 'gc, '_, B>, _index: i32) -> bool {
        false
    }

    fn get_element(
        &self,
        _activation: &mut Activation<'_, 'gc, '_, B>,
        _index: i32,
    ) -> Value<'gc, B> {
        Value::Undefined
    }

    fn set_element(
        &self,
        _activation: &mut Activation<'_, 'gc, '_, B>,
        _index: i32,
        _value: Value<'gc, B>,
    ) -> Result<(), Error<'gc, B>> {
        Ok(())
    }

    fn delete_element(&self, _activation: &mut Activation<'_, 'gc, '_, B>, _index: i32) -> bool {
        false
    }

    fn interfaces(&self) -> Vec<Object<'gc, B>> {
        //`super` does not implement interfaces
        vec![]
    }

    fn set_interfaces(
        &self,
        _gc_context: MutationContext<'gc, '_>,
        _iface_list: Vec<Object<'gc, B>>,
    ) {
        //`super` probably cannot have interfaces set on it
    }

    fn as_script_object(&self) -> Option<ScriptObject<'gc, B>> {
        None
    }

    fn as_super_object(&self) -> Option<SuperObject<'gc, B>> {
        Some(*self)
    }

    fn as_display_object(&self) -> Option<DisplayObject<'gc, B>> {
        //`super` actually can be used to invoke MovieClip methods
        self.0.read().this.as_display_object()
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }
}
