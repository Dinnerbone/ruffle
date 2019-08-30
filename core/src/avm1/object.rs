use crate::avm1::Value;
use crate::display_object::DisplayNode;
use core::fmt;
use gc_arena::{GcCell, MutationContext};
use std::collections::HashMap;


pub type NativeFunction<'gc> =
    fn(MutationContext<'gc, '_>, GcCell<'gc, Object<'gc>>, &[Value<'gc>]) -> Value<'gc>;

#[derive(Clone, Default)]
pub struct Object<'gc> {
    prototype: Option<GcCell<'gc, Object<'gc>>>,
    display_node: Option<DisplayNode<'gc>>,
    values: HashMap<String, Value<'gc>>,
    function: Option<NativeFunction<'gc>>,
}

unsafe impl<'gc> gc_arena::Collect for Object<'gc> {
    fn trace(&self, cc: gc_arena::CollectionContext) {
        self.prototype.trace(cc);
        self.display_node.trace(cc);
        self.values.trace(cc);
    }
}

impl fmt::Debug for Object<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Object")
            .field("prototype", &self.prototype)
            .field("display_node", &self.display_node)
            .field("values", &self.values)
            .field("function", &self.function.is_some())
            .finish()
    }
}

impl<'gc> Object<'gc> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn function(function: NativeFunction<'gc>) -> Self {
        Self {
            function: Some(function),
            ..Default::default()
        }
    }

    pub fn set_display_node(&mut self, display_node: DisplayNode<'gc>) {
        self.display_node = Some(display_node);
    }

    pub fn display_node(&self) -> Option<DisplayNode<'gc>> {
        self.display_node
    }

    pub fn set_prototype(&mut self, prototype: GcCell<'gc, Object<'gc>>) {
        self.prototype = Some(prototype);
    }

    pub fn prototype(&self) -> Option<&GcCell<'gc, Object<'gc>>> {
        self.prototype.as_ref()
    }

    pub fn set(&mut self, name: &str, value: Value<'gc>) {
        if name == "__proto__" {
            self.prototype = value.as_object().ok().map(ToOwned::to_owned);
        }
        self.values.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: &str) -> Value<'gc> {
        if name == "__proto__" {
            return self.prototype.map_or(Value::Undefined, |o| Value::Object(o));
        }
        if let Some(value) = self.values.get(name) {
            return value.to_owned();
        }
        self.prototype
            .as_ref()
            .map_or(Value::Undefined, |p| p.read().get(name))
    }

    pub fn has_property(&self, name: &str) -> bool {
        self.has_own_property(name)
            || self
                .prototype
                .as_ref()
                .map_or(false, |p| p.read().has_property(name))
    }

    pub fn has_own_property(&self, name: &str) -> bool {
        if name == "__proto__" {
            return true;
        }
        self.values.contains_key(name)
    }

    pub fn call(
        &self,
        gc_context: MutationContext<'gc, '_>,
        this: GcCell<'gc, Object<'gc>>,
        args: &[Value<'gc>],
    ) -> Value<'gc> {
        if let Some(function) = self.function {
            function(gc_context, this, args)
        } else {
            Value::Undefined
        }
    }
}
