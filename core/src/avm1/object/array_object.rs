use crate::avm1::property::Attribute;
use crate::avm1::{Activation, AvmString, Error, Object, ObjectPtr, ScriptObject, TObject, Value};
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use ruffle_types::ecma_conversions::f64_to_wrapping_i32;
use std::fmt;

#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct ArrayObject<'gc, B: Backend>(GcCell<'gc, ScriptObject<'gc, B>>);

impl<B: Backend> fmt::Debug for ArrayObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ArrayObject").finish()
    }
}

impl<'gc, B: Backend> ArrayObject<'gc, B> {
    pub fn empty(activation: &Activation<'_, 'gc, '_, B>) -> Self {
        Self::new(
            activation.context.gc_context,
            activation.context.avm1.prototypes().array,
            [],
        )
    }

    pub fn empty_with_proto(gc_context: MutationContext<'gc, '_>, proto: Object<'gc, B>) -> Self {
        Self::new_internal(gc_context, proto, [])
    }

    pub fn new(
        gc_context: MutationContext<'gc, '_>,
        array_proto: Object<'gc, B>,
        elements: impl IntoIterator<Item = Value<'gc, B>>,
    ) -> Self {
        Self::new_internal(gc_context, array_proto, elements)
    }

    fn new_internal(
        gc_context: MutationContext<'gc, '_>,
        proto: Object<'gc, B>,
        elements: impl IntoIterator<Item = Value<'gc, B>>,
    ) -> Self {
        let base = ScriptObject::object(gc_context, Some(proto));
        let mut length: i32 = 0;
        for value in elements.into_iter() {
            let length_str = AvmString::new_utf8(gc_context, length.to_string());
            base.define_value(gc_context, length_str, value, Attribute::empty());
            length += 1;
        }
        base.define_value(
            gc_context,
            "length",
            length.into(),
            Attribute::DONT_ENUM | Attribute::DONT_DELETE,
        );
        Self(GcCell::allocate(gc_context, base))
    }

    fn parse_index(name: AvmString<'gc>) -> Option<i32> {
        let name = name.trim_start_matches(|c| match u8::try_from(c) {
            Ok(c) => c.is_ascii_whitespace(),
            Err(_) => false,
        });

        name.parse::<std::num::Wrapping<i32>>().ok().map(|i| i.0)
    }
}

impl<'gc, B: Backend> TObject<'gc> for ArrayObject<'gc, B> {
    type B = B;

    fn get_local_stored(
        &self,
        name: impl Into<AvmString<'gc>>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Option<Value<'gc, B>> {
        self.0.read().get_local_stored(name, activation)
    }

    fn set_local(
        &self,
        name: AvmString<'gc>,
        value: Value<'gc, B>,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
    ) -> Result<(), Error<'gc, B>> {
        if &name == b"length" {
            let new_length = value.coerce_to_i32(activation)?;
            self.set_length(activation, new_length)?;
        } else if let Some(index) = Self::parse_index(name) {
            let length = self.length(activation)?;
            if index >= length {
                self.set_length(activation, index.wrapping_add(1))?;
            }
        }

        self.0.read().set_local(name, value, activation, this)
    }

    fn call(
        &self,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: Value<'gc, B>,
        args: &[Value<'gc, B>],
    ) -> Result<Value<'gc, B>, Error<'gc, B>> {
        self.0.read().call(name, activation, this, args)
    }

    fn getter(
        &self,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Option<Object<'gc, B>> {
        self.0.read().getter(name, activation)
    }

    fn setter(
        &self,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Option<Object<'gc, B>> {
        self.0.read().setter(name, activation)
    }

    fn create_bare_object(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
    ) -> Result<Object<'gc, B>, Error<'gc, B>> {
        Ok(Self::empty_with_proto(activation.context.gc_context, this).into())
    }

    fn delete(&self, activation: &mut Activation<'_, 'gc, '_, B>, name: AvmString<'gc>) -> bool {
        self.0.read().delete(activation, name)
    }

    fn add_property(
        &self,
        gc_context: MutationContext<'gc, '_>,
        name: AvmString<'gc>,
        get: Object<'gc, B>,
        set: Option<Object<'gc, B>>,
        attributes: Attribute,
    ) {
        self.0
            .read()
            .add_property(gc_context, name, get, set, attributes)
    }

    fn add_property_with_case(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
        get: Object<'gc, B>,
        set: Option<Object<'gc, B>>,
        attributes: Attribute,
    ) {
        self.0
            .read()
            .add_property_with_case(activation, name, get, set, attributes)
    }

    fn call_watcher(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
        value: &mut Value<'gc, B>,
        this: Object<'gc, B>,
    ) -> Result<(), Error<'gc, B>> {
        self.0.read().call_watcher(activation, name, value, this)
    }

    fn watch(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
        callback: Object<'gc, B>,
        user_data: Value<'gc, B>,
    ) {
        self.0.read().watch(activation, name, callback, user_data);
    }

    fn unwatch(&self, activation: &mut Activation<'_, 'gc, '_, B>, name: AvmString<'gc>) -> bool {
        self.0.read().unwatch(activation, name)
    }

    fn define_value(
        &self,
        gc_context: MutationContext<'gc, '_>,
        name: impl Into<AvmString<'gc>>,
        value: Value<'gc, B>,
        attributes: Attribute,
    ) {
        self.0
            .read()
            .define_value(gc_context, name, value, attributes)
    }

    fn set_attributes(
        &self,
        gc_context: MutationContext<'gc, '_>,
        name: Option<AvmString<'gc>>,
        set_attributes: Attribute,
        clear_attributes: Attribute,
    ) {
        self.0
            .read()
            .set_attributes(gc_context, name, set_attributes, clear_attributes)
    }

    fn proto(&self, activation: &mut Activation<'_, 'gc, '_, B>) -> Value<'gc, B> {
        self.0.read().proto(activation)
    }

    fn has_property(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        self.0.read().has_property(activation, name)
    }

    fn has_own_property(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        self.0.read().has_own_property(activation, name)
    }

    fn has_own_virtual(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        self.0.read().has_own_virtual(activation, name)
    }

    fn is_property_enumerable(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        self.0.read().is_property_enumerable(activation, name)
    }

    fn get_keys(&self, activation: &mut Activation<'_, 'gc, '_, B>) -> Vec<AvmString<'gc>> {
        self.0.read().get_keys(activation)
    }

    fn interfaces(&self) -> Vec<Object<'gc, B>> {
        self.0.read().interfaces()
    }

    fn set_interfaces(
        &self,
        gc_context: MutationContext<'gc, '_>,
        iface_list: Vec<Object<'gc, B>>,
    ) {
        self.0.read().set_interfaces(gc_context, iface_list)
    }

    fn as_script_object(&self) -> Option<ScriptObject<'gc, B>> {
        Some(*self.0.read())
    }

    fn as_array_object(&self) -> Option<ArrayObject<'gc, B>> {
        Some(*self)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.read().as_ptr() as *const ObjectPtr
    }

    fn length(&self, activation: &mut Activation<'_, 'gc, '_, B>) -> Result<i32, Error<'gc, B>> {
        self.0.read().length(activation)
    }

    fn set_length(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        new_length: i32,
    ) -> Result<(), Error<'gc, B>> {
        if let Value::Number(old_length) = self.0.read().get_data("length".into(), activation) {
            for i in new_length.max(0)..f64_to_wrapping_i32(old_length) {
                self.delete_element(activation, i);
            }
        }
        self.0.read().set_length(activation, new_length)
    }

    fn has_element(&self, activation: &mut Activation<'_, 'gc, '_, B>, index: i32) -> bool {
        self.0.read().has_element(activation, index)
    }

    fn get_element(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        index: i32,
    ) -> Value<'gc, B> {
        self.0.read().get_element(activation, index)
    }

    fn set_element(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        index: i32,
        value: Value<'gc, B>,
    ) -> Result<(), Error<'gc, B>> {
        let length = self.length(activation)?;
        if index >= length {
            self.set_length(activation, index.wrapping_add(1))?;
        }
        self.0.read().set_element(activation, index, value)
    }

    fn delete_element(&self, activation: &mut Activation<'_, 'gc, '_, B>, index: i32) -> bool {
        self.0.read().delete_element(activation, index)
    }
}
