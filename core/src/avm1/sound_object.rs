//! AVM1 object type to represent Sound objects.

use crate::avm1::function::Executable;
use crate::avm1::property::Attribute;
use crate::avm1::return_value::ReturnValue;
use crate::avm1::{Avm1, Error, Object, ObjectPtr, ScriptObject, TObject, Value};
use crate::backend::audio::{SoundHandle, SoundInstanceHandle};
use crate::context::UpdateContext;
use crate::display_object::DisplayObject;
use enumset::EnumSet;
use gc_arena::{Collect, GcCell, MutationContext};
use std::fmt;
use crate::backend::Backends;

/// A SounObject that is tied to a sound from the AudioBackend.
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct SoundObject<'gc, B: Backends>(GcCell<'gc, SoundObjectData<'gc, B>>);

pub struct SoundObjectData<'gc, B: Backends> {
    /// The underlying script object.
    ///
    /// This is used to handle "expando properties" on AVM1 display nodes, as
    /// well as the underlying prototype chain.
    base: ScriptObject<'gc, B>,

    /// The sound that is attached to this object.
    sound: Option<SoundHandle>,

    /// The instance of the last played sound on this object.
    sound_instance: Option<SoundInstanceHandle>,

    /// Sounds in AVM1 are tied to a speicifc movie clip.
    owner: Option<DisplayObject<'gc, B>>,

    /// Position of the last playing sound in milliseconds.
    position: u32,

    /// Duration of the currently attached sound in milliseconds.
    duration: u32,
}

unsafe impl<'gc, B: Backends> Collect for SoundObjectData<'gc, B> {
    fn trace(&self, cc: gc_arena::CollectionContext) {
        self.base.trace(cc);
        self.owner.trace(cc);
    }
}

impl<B: Backends> fmt::Debug for SoundObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("SoundObject")
            .field("sound", &this.sound)
            .field("sound_instance", &this.sound_instance)
            .field("owner", &this.owner)
            .finish()
    }
}

impl<'gc, B: Backends> SoundObject<'gc, B> {
    pub fn empty_sound(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
    ) -> SoundObject<'gc, B> {
        SoundObject(GcCell::allocate(
            gc_context,
            SoundObjectData {
                base: ScriptObject::object(gc_context, proto),
                sound: None,
                sound_instance: None,
                owner: None,
                position: 0,
                duration: 0,
            },
        ))
    }

    pub fn duration(self) -> u32 {
        self.0.read().duration
    }

    pub fn set_duration(self, gc_context: MutationContext<'gc, '_>, duration: u32) {
        self.0.write(gc_context).duration = duration;
    }

    pub fn sound(self) -> Option<SoundHandle> {
        self.0.read().sound
    }

    pub fn set_sound(self, gc_context: MutationContext<'gc, '_>, sound: Option<SoundHandle>) {
        self.0.write(gc_context).sound = sound;
    }

    pub fn sound_instance(self) -> Option<SoundInstanceHandle> {
        self.0.read().sound_instance
    }

    pub fn set_sound_instance(
        self,
        gc_context: MutationContext<'gc, '_>,
        sound_instance: Option<SoundInstanceHandle>,
    ) {
        self.0.write(gc_context).sound_instance = sound_instance;
    }

    pub fn owner(self) -> Option<DisplayObject<'gc, B>> {
        self.0.read().owner
    }

    pub fn set_owner(
        self,
        gc_context: MutationContext<'gc, '_>,
        owner: Option<DisplayObject<'gc, B>>,
    ) {
        self.0.write(gc_context).owner = owner;
    }

    pub fn position(self) -> u32 {
        self.0.read().position
    }

    pub fn set_position(self, gc_context: MutationContext<'gc, '_>, position: u32) {
        self.0.write(gc_context).position = position;
    }

    fn base(self) -> ScriptObject<'gc, B> {
        self.0.read().base
    }
}

impl<'gc, B: Backends> TObject<'gc, B> for SoundObject<'gc, B> {
    fn get_local(
        &self,
        name: &str,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
    ) -> Result<ReturnValue<'gc, B>, Error> {
        self.base().get_local(name, avm, context, this)
    }

    fn set(
        &self,
        name: &str,
        value: Value<'gc, B>,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
    ) -> Result<(), Error> {
        self.base().set(name, value, avm, context)
    }

    fn call(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
        base_proto: Option<Object<'gc, B>>,
        args: &[Value<'gc, B>],
    ) -> Result<ReturnValue<'gc, B>, Error> {
        self.base().call(avm, context, this, base_proto, args)
    }

    fn call_setter(
        &self,
        name: &str,
        value: Value<'gc, B>,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
    ) -> Result<ReturnValue<'gc, B>, Error> {
        self.base().call_setter(name, value, avm, context, this)
    }

    #[allow(clippy::new_ret_no_self)]
    fn new(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        _this: Object<'gc, B>,
        _args: &[Value<'gc, B>],
    ) -> Result<Object<'gc, B>, Error> {
        Ok(SoundObject::empty_sound(context.gc_context, Some(avm.prototypes.sound)).into())
    }

    fn delete(
        &self,
        avm: &mut Avm1<'gc, B>,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
    ) -> bool {
        self.base().delete(avm, gc_context, name)
    }

    fn proto(&self) -> Option<Object<'gc, B>> {
        self.base().proto()
    }

    fn set_proto(&self, gc_context: MutationContext<'gc, '_>, prototype: Option<Object<'gc, B>>) {
        self.base().set_proto(gc_context, prototype);
    }

    fn define_value(
        &self,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
        value: Value<'gc, B>,
        attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .define_value(gc_context, name, value, attributes)
    }

    fn set_attributes(
        &mut self,
        gc_context: MutationContext<'gc, '_>,
        name: Option<&str>,
        set_attributes: EnumSet<Attribute>,
        clear_attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .set_attributes(gc_context, name, set_attributes, clear_attributes)
    }

    fn add_property(
        &self,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
        get: Executable<'gc, B>,
        set: Option<Executable<'gc, B>>,
        attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .add_property(gc_context, name, get, set, attributes)
    }

    fn add_property_with_case(
        &self,
        avm: &mut Avm1<'gc, B>,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
        get: Executable<'gc, B>,
        set: Option<Executable<'gc, B>>,
        attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .add_property_with_case(avm, gc_context, name, get, set, attributes)
    }

    fn has_property(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        name: &str,
    ) -> bool {
        self.base().has_property(avm, context, name)
    }

    fn has_own_property(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        name: &str,
    ) -> bool {
        self.base().has_own_property(avm, context, name)
    }

    fn has_own_virtual(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        name: &str,
    ) -> bool {
        self.base().has_own_virtual(avm, context, name)
    }

    fn is_property_overwritable(&self, avm: &mut Avm1<'gc, B>, name: &str) -> bool {
        self.base().is_property_overwritable(avm, name)
    }

    fn is_property_enumerable(&self, avm: &mut Avm1<'gc, B>, name: &str) -> bool {
        self.base().is_property_enumerable(avm, name)
    }

    fn get_keys(&self, avm: &mut Avm1<'gc, B>) -> Vec<String> {
        self.base().get_keys(avm)
    }

    fn as_string(&self) -> String {
        self.base().as_string()
    }

    fn type_of(&self) -> &'static str {
        self.base().type_of()
    }

    fn interfaces(&self) -> Vec<Object<'gc, B>> {
        self.base().interfaces()
    }

    fn set_interfaces(
        &mut self,
        gc_context: MutationContext<'gc, '_>,
        iface_list: Vec<Object<'gc, B>>,
    ) {
        self.base().set_interfaces(gc_context, iface_list)
    }

    fn as_script_object(&self) -> Option<ScriptObject<'gc, B>> {
        Some(self.base())
    }

    fn as_display_object(&self) -> Option<DisplayObject<'gc, B>> {
        None
    }

    fn as_executable(&self) -> Option<Executable<'gc, B>> {
        None
    }

    fn as_sound_object(&self) -> Option<SoundObject<'gc, B>> {
        Some(*self)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }

    fn length(&self) -> usize {
        self.base().length()
    }

    fn array(&self) -> Vec<Value<'gc, B>> {
        self.base().array()
    }

    fn set_length(&self, gc_context: MutationContext<'gc, '_>, length: usize) {
        self.base().set_length(gc_context, length)
    }

    fn array_element(&self, index: usize) -> Value<'gc, B> {
        self.base().array_element(index)
    }

    fn set_array_element(
        &self,
        index: usize,
        value: Value<'gc, B>,
        gc_context: MutationContext<'gc, '_>,
    ) -> usize {
        self.base().set_array_element(index, value, gc_context)
    }

    fn delete_array_element(&self, index: usize, gc_context: MutationContext<'gc, '_>) {
        self.base().delete_array_element(index, gc_context)
    }
}
