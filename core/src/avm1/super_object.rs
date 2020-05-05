//! Special object that implements `super`

use crate::avm1::function::Executable;
use crate::avm1::object::search_prototype;
use crate::avm1::property::Attribute;
use crate::avm1::return_value::ReturnValue;
use crate::avm1::script_object::TYPE_OF_OBJECT;
use crate::avm1::{Avm1, Error, Object, ObjectPtr, ScriptObject, TObject, Value};
use crate::context::UpdateContext;
use crate::display_object::DisplayObject;
use enumset::EnumSet;
use gc_arena::{Collect, GcCell, MutationContext};
use crate::backend::Backends;

/// Implementation of the `super` object in AS2.
///
/// A `SuperObject` references all data from another object, but with one layer
/// of prototyping removed. It's as if the given object had been constructed
/// with it's parent class.
#[collect(no_drop)]
#[derive(Copy, Clone, Collect, Debug)]
pub struct SuperObject<'gc, B: Backends>(GcCell<'gc, SuperObjectData<'gc, B>>);

#[collect(no_drop)]
#[derive(Clone, Collect, Debug)]
pub struct SuperObjectData<'gc, B: Backends> {
    /// The object present as `this` throughout the superchain.
    child: Object<'gc, B>,

    /// The `proto` that the currently-executing method was pulled from.
    base_proto: Object<'gc, B>,
}

impl<'gc, B: Backends> SuperObject<'gc, B> {
    /// Construct a `super` for an incoming stack frame.
    ///
    /// `this` and `base_proto` must be the values provided to
    /// `Executable.exec`.
    ///
    /// NOTE: This function must not borrow any `GcCell` data as it is
    /// sometimes called while mutable borrows are held on cells. Specifically,
    /// `Object.call_setter` will panic if this function attempts to borrow
    /// *any* objects.
    pub fn from_this_and_base_proto(
        this: Object<'gc, B>,
        base_proto: Object<'gc, B>,
        _avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
    ) -> Result<Self, Error> {
        Ok(Self(GcCell::allocate(
            context.gc_context,
            SuperObjectData {
                child: this,
                base_proto,
            },
        )))
    }

    /// Retrieve the prototype that `super` should be pulling from.
    fn super_proto(self) -> Option<Object<'gc, B>> {
        self.0.read().base_proto.proto()
    }

    /// Retrieve the constructor associated with the super proto.
    fn super_constr(
        self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
    ) -> Result<Option<Object<'gc, B>>, Error> {
        if let Some(super_proto) = self.super_proto() {
            Ok(super_proto
                .get("__constructor__", avm, context)?
                .resolve(avm, context)?
                .as_object()
                .ok())
        } else {
            Ok(None)
        }
    }
}

impl<'gc, B: Backends> TObject<'gc, B> for SuperObject<'gc, B> {
    fn get_local(
        &self,
        _name: &str,
        _avm: &mut Avm1<'gc, B>,
        _context: &mut UpdateContext<'_, 'gc, '_, B>,
        _this: Object<'gc, B>,
    ) -> Result<ReturnValue<'gc, B>, Error> {
        Ok(Value::Undefined.into())
    }

    fn set(
        &self,
        _name: &str,
        _value: Value<'gc, B>,
        _avm: &mut Avm1<'gc, B>,
        _context: &mut UpdateContext<'_, 'gc, '_, B>,
    ) -> Result<(), Error> {
        //TODO: What happens if you set `super.__proto__`?
        Ok(())
    }

    fn call(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        _this: Object<'gc, B>,
        _base_proto: Option<Object<'gc, B>>,
        args: &[Value<'gc, B>],
    ) -> Result<ReturnValue<'gc, B>, Error> {
        if let Some(constr) = self.super_constr(avm, context)? {
            constr.call(avm, context, self.0.read().child, self.super_proto(), args)
        } else {
            Ok(Value::Undefined.into())
        }
    }

    fn call_method(
        &self,
        name: &str,
        args: &[Value<'gc, B>],
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
    ) -> Result<ReturnValue<'gc, B>, Error> {
        let child = self.0.read().child;
        let super_proto = self.super_proto();
        let (method, base_proto) = search_prototype(super_proto, name, avm, context, child)?;
        let method = method.resolve(avm, context)?;

        if let Value::Object(_) = method {
        } else {
            log::warn!("Super method {} is not callable", name);
        }

        method.call(avm, context, child, base_proto, args)
    }

    fn call_setter(
        &self,
        name: &str,
        value: Value<'gc, B>,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
    ) -> Result<ReturnValue<'gc, B>, Error> {
        self.0
            .read()
            .child
            .call_setter(name, value, avm, context, this)
    }

    #[allow(clippy::new_ret_no_self)]
    fn new(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
        args: &[Value<'gc, B>],
    ) -> Result<Object<'gc, B>, Error> {
        if let Some(proto) = self.proto() {
            proto.new(avm, context, this, args)
        } else {
            // TODO: What happens when you `new super` but there's no
            // super? Is this code even reachable?!
            self.0.read().child.new(avm, context, this, args)
        }
    }

    fn delete(
        &self,
        _avm: &mut Avm1<'gc, B>,
        _gc_context: MutationContext<'gc, '_>,
        _name: &str,
    ) -> bool {
        //`super` cannot have properties deleted from it
        false
    }

    fn proto(&self) -> Option<Object<'gc, B>> {
        self.super_proto()
    }

    fn set_proto(&self, gc_context: MutationContext<'gc, '_>, prototype: Option<Object<'gc, B>>) {
        if let Some(prototype) = prototype {
            self.0.write(gc_context).base_proto = prototype;
        }
    }

    fn define_value(
        &self,
        _gc_context: MutationContext<'gc, '_>,
        _name: &str,
        _value: Value<'gc, B>,
        _attributes: EnumSet<Attribute>,
    ) {
        //`super` cannot have values defined on it
    }

    fn set_attributes(
        &mut self,
        _gc_context: MutationContext<'gc, '_>,
        _name: Option<&str>,
        _set_attributes: EnumSet<Attribute>,
        _clear_attributes: EnumSet<Attribute>,
    ) {
        //TODO: Does ASSetPropFlags work on `super`? What would it even work on?
    }

    fn add_property(
        &self,
        _gc_context: MutationContext<'gc, '_>,
        _name: &str,
        _get: Executable<'gc, B>,
        _set: Option<Executable<'gc, B>>,
        _attributes: EnumSet<Attribute>,
    ) {
        //`super` cannot have properties defined on it
    }

    fn add_property_with_case(
        &self,
        _avm: &mut Avm1<'gc, B>,
        _gc_context: MutationContext<'gc, '_>,
        _name: &str,
        _get: Executable<'gc, B>,
        _set: Option<Executable<'gc, B>>,
        _attributes: EnumSet<Attribute>,
    ) {
        //`super` cannot have properties defined on it
    }

    fn has_property(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        name: &str,
    ) -> bool {
        self.0.read().child.has_property(avm, context, name)
    }

    fn has_own_property(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        name: &str,
    ) -> bool {
        self.0.read().child.has_own_property(avm, context, name)
    }

    fn has_own_virtual(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        name: &str,
    ) -> bool {
        self.0.read().child.has_own_virtual(avm, context, name)
    }

    fn is_property_enumerable(&self, avm: &mut Avm1<'gc, B>, name: &str) -> bool {
        self.0.read().child.is_property_enumerable(avm, name)
    }

    fn is_property_overwritable(&self, avm: &mut Avm1<'gc, B>, name: &str) -> bool {
        self.0.read().child.is_property_overwritable(avm, name)
    }

    fn get_keys(&self, _avm: &mut Avm1<'gc, B>) -> Vec<String> {
        vec![]
    }

    fn as_string(&self) -> String {
        self.0.read().child.as_string()
    }

    fn type_of(&self) -> &'static str {
        TYPE_OF_OBJECT
    }

    fn length(&self) -> usize {
        0
    }

    fn set_length(&self, _gc_context: MutationContext<'gc, '_>, _new_length: usize) {}

    fn array(&self) -> Vec<Value<'gc, B>> {
        vec![]
    }

    fn array_element(&self, _index: usize) -> Value<'gc, B> {
        Value::Undefined
    }

    fn set_array_element(
        &self,
        _index: usize,
        _value: Value<'gc, B>,
        _gc_context: MutationContext<'gc, '_>,
    ) -> usize {
        0
    }

    fn delete_array_element(&self, _index: usize, _gc_context: MutationContext<'gc, '_>) {}

    fn interfaces(&self) -> Vec<Object<'gc, B>> {
        //`super` does not implement interfaces
        vec![]
    }

    fn set_interfaces(
        &mut self,
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
        self.0.read().child.as_display_object()
    }

    fn as_executable(&self) -> Option<Executable<'gc, B>> {
        //well, `super` *can* be called...
        //...but `super_constr` needs an avm and context in order to get called.
        //ergo, we can't downcast.
        None
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }
}
