//! User-defined properties

use crate::avm1::{Object, Value};
use bitflags::bitflags;
use core::fmt;
use gc_arena::Collect;
use ruffle_types::backend::Backend;

bitflags! {
    /// Attributes of properties in the AVM runtime.
    /// The values are significant and should match the order used by `object::as_set_prop_flags`.
    #[derive(Collect)]
    #[collect(require_static)]
    pub struct Attribute: u16 {
        const DONT_ENUM     = 1 << 0;
        const DONT_DELETE   = 1 << 1;
        const READ_ONLY     = 1 << 2;
        const VERSION_MASK  = 0x1FFF << 3;
    }
}

/// To check if a property is available in a specific SWF version, mask the property attributes
/// against the entry in this array. If the result is non-zero, the property should be hidden.
const VERSION_MASKS: [u16; 10] = [
    // SWFv4 and earlier: always hide
    // Shouldn't really be used because SWFv4 did not have much AS support.
    0b0111_1111_1111_1000,
    0b0111_1111_1111_1000,
    0b0111_1111_1111_1000,
    0b0111_1111_1111_1000,
    0b0111_1111_1111_1000,
    // SWFv5 and above
    0b0111_0100_1000_0000, // v5
    0b0111_0101_0000_0000, // v6
    0b0111_0000_0000_0000, // v7
    0b0110_0000_0000_0000, // v8
    0b0100_0000_0000_0000, // v9
];

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct Property<'gc, B: Backend> {
    data: Value<'gc, B>,
    getter: Option<Object<'gc, B>>,
    setter: Option<Object<'gc, B>>,
    attributes: Attribute,
}

impl<'gc, B: Backend> Property<'gc, B> {
    pub fn new_stored(data: Value<'gc, B>, attributes: Attribute) -> Self {
        Self {
            data,
            getter: None,
            setter: None,
            attributes,
        }
    }

    pub fn new_virtual(
        getter: Object<'gc, B>,
        setter: Option<Object<'gc, B>>,
        attributes: Attribute,
    ) -> Self {
        Self {
            data: Value::Undefined,
            getter: Some(getter),
            setter,
            attributes,
        }
    }

    pub fn data(&self) -> Value<'gc, B> {
        self.data
    }

    pub fn getter(&self) -> Option<Object<'gc, B>> {
        self.getter
    }

    pub fn setter(&self) -> Option<Object<'gc, B>> {
        self.setter
    }

    /// Store data on this property, ignoring virtual setters.
    ///
    /// Read-only properties are not affected.
    pub fn set_data(&mut self, data: Value<'gc, B>) {
        if self.is_overwritable() {
            self.data = data;
            // Overwriting a property also clears SWF version requirements.
            self.attributes.remove(Attribute::VERSION_MASK);
        }
    }

    /// Make this property virtual by attaching a getter/setter to it.
    pub fn set_virtual(&mut self, getter: Object<'gc, B>, setter: Option<Object<'gc, B>>) {
        self.getter = Some(getter);
        self.setter = setter;
    }

    /// List this property's attributes.
    pub fn attributes(&self) -> Attribute {
        self.attributes
    }

    /// Re-define this property's attributes.
    pub fn set_attributes(&mut self, attributes: Attribute) {
        self.attributes = attributes;
    }

    pub fn is_enumerable(&self) -> bool {
        !self.attributes.contains(Attribute::DONT_ENUM)
    }

    pub fn can_delete(&self) -> bool {
        !self.attributes.contains(Attribute::DONT_DELETE)
    }

    pub fn is_overwritable(&self) -> bool {
        !self.attributes.contains(Attribute::READ_ONLY)
    }

    pub fn is_virtual(&self) -> bool {
        self.getter.is_some()
    }

    /// Checks if this property is accessible in the given SWF version.
    /// If `false`, the property should be returned as `undefined`.
    pub fn allow_swf_version(&self, swf_version: u8) -> bool {
        let mask = VERSION_MASKS
            .get(usize::from(swf_version))
            .copied()
            .unwrap_or_default();
        (self.attributes.bits() & mask) == 0
    }
}

impl<B: Backend> fmt::Debug for Property<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Property")
            .field("data", &self.data)
            .field("getter", &self.getter)
            .field("setter", &self.setter)
            .field("attributes", &self.attributes)
            .finish()
    }
}
