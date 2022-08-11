//! AVM1 object type to represent objects on the stage.

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::property::Attribute;
use crate::avm1::property_map::PropertyMap;
use crate::avm1::{Object, ObjectPtr, ScriptObject, TDisplayObject, TObject, Value};
use crate::avm_warn;
use crate::context::UpdateContext;
use crate::display_object::{DisplayObject, EditText, MovieClip, TDisplayObjectContainer};
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use ruffle_types::numbers::Percent;
use ruffle_types::string::{AvmString, WStr};
use std::fmt;

/// A ScriptObject that is inherently tied to a display node.
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct StageObject<'gc, B: Backend>(GcCell<'gc, StageObjectData<'gc, B>>);

#[derive(Collect)]
#[collect(no_drop)]
pub struct StageObjectData<'gc, B: Backend> {
    /// The underlying script object.
    ///
    /// This is used to handle "expando properties" on AVM1 display nodes, as
    /// well as the underlying prototype chain.
    base: ScriptObject<'gc, B>,

    /// The display node this stage object
    display_object: DisplayObject<'gc, B>,

    text_field_bindings: Vec<TextFieldBinding<'gc, B>>,
}

impl<'gc, B: Backend> StageObject<'gc, B> {
    /// Create a stage object for a given display node.
    pub fn for_display_object(
        gc_context: MutationContext<'gc, '_>,
        display_object: DisplayObject<'gc, B>,
        proto: Option<Object<'gc, B>>,
    ) -> Self {
        Self(GcCell::allocate(
            gc_context,
            StageObjectData {
                base: ScriptObject::object(gc_context, proto),
                display_object,
                text_field_bindings: Vec::new(),
            },
        ))
    }

    /// Registers a text field variable binding for this stage object.
    /// Whenever a property with the given name is changed, we should change the text in the text field.
    pub fn register_text_field_binding(
        self,
        gc_context: MutationContext<'gc, '_>,
        text_field: EditText<'gc, B>,
        variable_name: AvmString<'gc>,
    ) {
        self.0
            .write(gc_context)
            .text_field_bindings
            .push(TextFieldBinding {
                text_field,
                variable_name,
            })
    }

    /// Removes a text field binding for the given text field.
    /// Does not place the text field on the unbound list.
    /// Caller is responsible for placing the text field on the unbound list, if necessary.
    pub fn clear_text_field_binding(
        self,
        gc_context: MutationContext<'gc, '_>,
        text_field: EditText<'gc, B>,
    ) {
        self.0
            .write(gc_context)
            .text_field_bindings
            .retain(|binding| !DisplayObject::ptr_eq(text_field.into(), binding.text_field.into()));
    }

    /// Clears all text field bindings from this stage object, and places the textfields on the unbound list.
    /// This is called when the object is removed from the stage.
    pub fn unregister_text_field_bindings(self, context: &mut UpdateContext<'_, 'gc, '_, B>) {
        for binding in self
            .0
            .write(context.gc_context)
            .text_field_bindings
            .drain(..)
        {
            binding.text_field.clear_bound_stage_object(context);
            context.unbound_text_fields.push(binding.text_field);
        }
    }

    fn resolve_path_property(
        self,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Option<Value<'gc, B>> {
        let case_sensitive = activation.is_case_sensitive();
        if name.eq_with_case(b"_root", case_sensitive) {
            return Some(activation.root_object());
        } else if name.eq_with_case(b"_parent", case_sensitive) {
            return Some(
                self.0
                    .read()
                    .display_object
                    .avm1_parent()
                    .map(|dn| dn.object().coerce_to_object(activation))
                    .map(Value::Object)
                    .unwrap_or(Value::Undefined),
            );
        } else if name.eq_with_case(b"_global", case_sensitive) {
            return Some(activation.context.avm1.global_object());
        }

        // Resolve level names `_levelN`.
        if let Some(prefix) = name.slice(..6) {
            // `_flash` is a synonym of `_level`, a relic from the earliest Flash versions.
            if prefix.eq_with_case(b"_level", case_sensitive)
                || prefix.eq_with_case(b"_flash", case_sensitive)
            {
                let level_id = Self::parse_level_id(&name[6..]);
                let level = activation
                    .context
                    .stage
                    .child_by_depth(level_id)
                    .map(|o| o.object())
                    .unwrap_or(Value::Undefined);
                return Some(level);
            }
        }

        None
    }

    fn parse_level_id(digits: &WStr) -> i32 {
        // TODO: Use `split_first`?
        let (is_negative, digits) = match digits.get(0) {
            Some(45) => (true, &digits[1..]),
            _ => (false, digits),
        };
        let mut level_id: i32 = 0;
        for digit in digits
            .iter()
            .map_while(|c| char::from_u32(c.into()).and_then(|c| c.to_digit(10)))
        {
            level_id = level_id.wrapping_mul(10);
            level_id = level_id.wrapping_add(digit as i32);
        }
        if is_negative {
            level_id = level_id.wrapping_neg();
        }
        level_id
    }
}

/// A binding from a property of this StageObject to an EditText text field.
#[derive(Collect)]
#[collect(no_drop)]
struct TextFieldBinding<'gc, B: Backend> {
    text_field: EditText<'gc, B>,
    variable_name: AvmString<'gc>,
}

impl<B: Backend> fmt::Debug for StageObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let o = self.0.read();
        f.debug_struct("StageObject")
            .field("base", &o.base)
            .field("display_object", &o.display_object)
            .finish()
    }
}

impl<'gc, B: Backend> TObject<'gc> for StageObject<'gc, B> {
    type B = B;

    fn get_local_stored(
        &self,
        name: impl Into<AvmString<'gc>>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Option<Value<'gc, B>> {
        let name = name.into();
        let obj = self.0.read();
        let props = activation.context.avm1.display_properties;

        // Property search order for DisplayObjects:
        // 1) Actual properties on the underlying object
        if let Some(value) = obj.base.get_local_stored(name, activation) {
            return Some(value);
        }

        // 2) Path properties such as `_root`, `_parent`, `_levelN` (obeys case sensitivity)
        let magic_property = name.starts_with(b'_');
        if magic_property {
            if let Some(object) = self.resolve_path_property(name, activation) {
                return Some(object);
            }
        }

        // 3) Child display objects with the given instance name
        if let Some(child) = obj
            .display_object
            .as_container()
            .and_then(|o| o.child_by_name(&name, activation.is_case_sensitive()))
        {
            return Some(child.object());
        }

        // 4) Display object properties such as `_x`, `_y` (never case sensitive)
        if magic_property {
            if let Some(property) = props.read().get_by_name(name) {
                return Some(property.get(activation, obj.display_object));
            }
        }

        None
    }

    fn set_local(
        &self,
        name: AvmString<'gc>,
        value: Value<'gc, B>,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
    ) -> Result<(), Error<'gc, B>> {
        let obj = self.0.read();
        let props = activation.context.avm1.display_properties;

        // Check if a text field is bound to this property and update the text if so.
        let case_sensitive = activation.is_case_sensitive();
        for binding in obj.text_field_bindings.iter().filter(|binding| {
            if case_sensitive {
                binding.variable_name == name
            } else {
                binding.variable_name.eq_ignore_case(&name)
            }
        }) {
            let _ = binding.text_field.set_html_text(
                &value.coerce_to_string(activation)?,
                &mut activation.context,
            );
        }

        let base = obj.base;
        let display_object = obj.display_object;
        drop(obj);

        if base.has_own_property(activation, name) {
            // 1) Actual properties on the underlying object
            base.set_local(name, value, activation, this)
        } else if let Some(property) = props.read().get_by_name(name) {
            // 2) Display object properties such as _x, _y
            property.set(activation, display_object, value)?;
            Ok(())
        } else {
            // 3) TODO: Prototype
            base.set_local(name, value, activation, this)
        }
    }

    fn call(
        &self,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: Value<'gc, B>,
        args: &[Value<'gc, B>],
    ) -> Result<Value<'gc, B>, Error<'gc, B>> {
        self.0.read().base.call(name, activation, this, args)
    }

    fn getter(
        &self,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Option<Object<'gc, B>> {
        self.0.read().base.getter(name, activation)
    }

    fn setter(
        &self,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Option<Object<'gc, B>> {
        self.0.read().base.setter(name, activation)
    }

    fn create_bare_object(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
    ) -> Result<Object<'gc, B>, Error<'gc, B>> {
        //TODO: Create a StageObject of some kind
        self.0.read().base.create_bare_object(activation, this)
    }

    fn delete(&self, activation: &mut Activation<'_, 'gc, '_, B>, name: AvmString<'gc>) -> bool {
        self.0.read().base.delete(activation, name)
    }

    fn proto(&self, activation: &mut Activation<'_, 'gc, '_, B>) -> Value<'gc, B> {
        self.0.read().base.proto(activation)
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
            .base
            .define_value(gc_context, name, value, attributes)
    }

    fn set_attributes(
        &self,
        gc_context: MutationContext<'gc, '_>,
        name: Option<AvmString<'gc>>,
        set_attributes: Attribute,
        clear_attributes: Attribute,
    ) {
        self.0.write(gc_context).base.set_attributes(
            gc_context,
            name,
            set_attributes,
            clear_attributes,
        )
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
            .base
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
            .base
            .add_property_with_case(activation, name, get, set, attributes)
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
            .base
            .call_watcher(activation, name, value, this)
    }

    fn watch(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
        callback: Object<'gc, B>,
        user_data: Value<'gc, B>,
    ) {
        self.0
            .read()
            .base
            .watch(activation, name, callback, user_data);
    }

    fn unwatch(&self, activation: &mut Activation<'_, 'gc, '_, B>, name: AvmString<'gc>) -> bool {
        self.0.read().base.unwatch(activation, name)
    }

    fn has_property(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        let obj = self.0.read();
        if obj.base.has_property(activation, name) {
            return true;
        }

        let magic_property = name.starts_with(b'_');
        if magic_property
            && activation
                .context
                .avm1
                .display_properties
                .read()
                .get_by_name(name)
                .is_some()
        {
            return true;
        }

        let case_sensitive = activation.is_case_sensitive();
        if obj
            .display_object
            .as_container()
            .and_then(|o| o.child_by_name(&name, case_sensitive))
            .is_some()
        {
            return true;
        }

        if magic_property && self.resolve_path_property(name, activation).is_some() {
            return true;
        }

        false
    }

    fn has_own_property(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        // Note that `hasOwnProperty` does NOT return true for child display objects.
        self.0.read().base.has_own_property(activation, name)
    }

    fn has_own_virtual(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        self.0.read().base.has_own_virtual(activation, name)
    }

    fn is_property_enumerable(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        name: AvmString<'gc>,
    ) -> bool {
        self.0.read().base.is_property_enumerable(activation, name)
    }

    fn get_keys(&self, activation: &mut Activation<'_, 'gc, '_, B>) -> Vec<AvmString<'gc>> {
        // Keys from the underlying object are listed first, followed by
        // child display objects in order from highest depth to lowest depth.
        let obj = self.0.read();
        let mut keys = obj.base.get_keys(activation);

        if let Some(ctr) = obj.display_object.as_container() {
            keys.extend(ctr.iter_render_list().rev().map(|child| child.name()));
        }

        keys
    }

    fn length(&self, activation: &mut Activation<'_, 'gc, '_, B>) -> Result<i32, Error<'gc, B>> {
        self.0.read().base.length(activation)
    }

    fn set_length(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        length: i32,
    ) -> Result<(), Error<'gc, B>> {
        self.0.read().base.set_length(activation, length)
    }

    fn has_element(&self, activation: &mut Activation<'_, 'gc, '_, B>, index: i32) -> bool {
        self.0.read().base.has_element(activation, index)
    }

    fn get_element(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        index: i32,
    ) -> Value<'gc, B> {
        self.0.read().base.get_element(activation, index)
    }

    fn set_element(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        index: i32,
        value: Value<'gc, B>,
    ) -> Result<(), Error<'gc, B>> {
        self.0.read().base.set_element(activation, index, value)
    }

    fn delete_element(&self, activation: &mut Activation<'_, 'gc, '_, B>, index: i32) -> bool {
        self.0.read().base.delete_element(activation, index)
    }

    fn interfaces(&self) -> Vec<Object<'gc, B>> {
        self.0.read().base.interfaces()
    }

    fn set_interfaces(
        &self,
        gc_context: MutationContext<'gc, '_>,
        iface_list: Vec<Object<'gc, B>>,
    ) {
        self.0
            .write(gc_context)
            .base
            .set_interfaces(gc_context, iface_list)
    }

    fn as_script_object(&self) -> Option<ScriptObject<'gc, B>> {
        Some(self.0.read().base)
    }

    /// Get the underlying stage object, if it exists.
    fn as_stage_object(&self) -> Option<StageObject<'gc, B>> {
        Some(*self)
    }

    fn as_display_object(&self) -> Option<DisplayObject<'gc, B>> {
        Some(self.0.read().display_object)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.read().base.as_ptr() as *const ObjectPtr
    }
}

/// Properties shared by display objects in AVM1, such as _x and _y.
/// These are only accessible for movie clips, buttons, and text fields (any others?)
/// These exist outside the global or prototype machinery. Instead, they are
/// "special" properties stored in a separate map that display objects look at in addition
/// to normal property lookup.
/// The map of property names to display object getts/setters.
#[derive(Copy, Clone)]
pub struct DisplayProperty<'gc, B: Backend> {
    get: DisplayGetter<'gc, B>,
    set: Option<DisplaySetter<'gc, B>>,
}

pub type DisplayGetter<'gc, B: Backend> =
    fn(&mut Activation<'_, 'gc, '_, B>, DisplayObject<'gc, B>) -> Value<'gc, B>;

pub type DisplaySetter<'gc, B: Backend> = fn(
    &mut Activation<'_, 'gc, '_, B>,
    DisplayObject<'gc, B>,
    Value<'gc, B>,
) -> Result<(), Error<'gc, B>>;

impl<'gc, B: Backend> DisplayProperty<'gc, B> {
    pub fn get(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: DisplayObject<'gc, B>,
    ) -> Value<'gc, B> {
        (self.get)(activation, this)
    }

    pub fn set(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: DisplayObject<'gc, B>,
        value: Value<'gc, B>,
    ) -> Result<(), Error<'gc, B>> {
        self.set
            .map(|f| f(activation, this, value))
            .unwrap_or(Ok(()))
    }
}

unsafe impl<'gc, B: Backend> Collect for DisplayProperty<'gc, B> {
    fn needs_trace() -> bool {
        false
    }
}

/// The map from key/index to function pointers for special display object properties.
#[derive(Collect)]
#[collect(no_drop)]
pub struct DisplayPropertyMap<'gc, B: Backend>(PropertyMap<'gc, DisplayProperty<'gc, B>>);

impl<'gc, B: Backend> DisplayPropertyMap<'gc, B> {
    /// Creates the display property map.
    pub fn new(gc_context: MutationContext<'gc, '_>) -> GcCell<'gc, DisplayPropertyMap<'gc, B>> {
        let mut property_map = DisplayPropertyMap(PropertyMap::new());

        // Order is important:
        // should match the SWF specs for GetProperty/SetProperty.
        property_map.add_property("_x".into(), x, Some(set_x));
        property_map.add_property("_y".into(), y, Some(set_y));
        property_map.add_property("_xscale".into(), x_scale, Some(set_x_scale));
        property_map.add_property("_yscale".into(), y_scale, Some(set_y_scale));
        property_map.add_property("_currentframe".into(), current_frame, None);
        property_map.add_property("_totalframes".into(), total_frames, None);
        property_map.add_property("_alpha".into(), alpha, Some(set_alpha));
        property_map.add_property("_visible".into(), visible, Some(set_visible));
        property_map.add_property("_width".into(), width, Some(set_width));
        property_map.add_property("_height".into(), height, Some(set_height));
        property_map.add_property("_rotation".into(), rotation, Some(set_rotation));
        property_map.add_property("_target".into(), target, None);
        property_map.add_property("_framesloaded".into(), frames_loaded, None);
        property_map.add_property("_name".into(), name, Some(set_name));
        property_map.add_property("_droptarget".into(), drop_target, None);
        property_map.add_property("_url".into(), url, None);
        property_map.add_property("_highquality".into(), high_quality, Some(set_high_quality));
        property_map.add_property("_focusrect".into(), focus_rect, Some(set_focus_rect));
        property_map.add_property(
            "_soundbuftime".into(),
            sound_buf_time,
            Some(set_sound_buf_time),
        );
        property_map.add_property("_quality".into(), quality, Some(set_quality));
        property_map.add_property("_xmouse".into(), x_mouse, None);
        property_map.add_property("_ymouse".into(), y_mouse, None);

        GcCell::allocate(gc_context, property_map)
    }

    /// Gets a property slot by name.
    /// Used by `GetMember`, `GetVariable`, `SetMember`, and `SetVariable`.
    pub fn get_by_name(&self, name: AvmString<'gc>) -> Option<&DisplayProperty<'gc, B>> {
        // Display object properties are case insensitive, regardless of SWF version!?
        // TODO: Another string alloc; optimize this eventually.
        self.0.get(name, false)
    }

    /// Gets a property slot by SWF4 index.
    /// The order is defined by the SWF specs.
    /// Used by `GetProperty`/`SetProperty`.
    /// SWF19 pp. 85-86
    pub fn get_by_index(&self, index: usize) -> Option<&DisplayProperty<'gc, B>> {
        self.0.get_index(index)
    }

    fn add_property(
        &mut self,
        name: AvmString<'gc>,
        get: DisplayGetter<'gc, B>,
        set: Option<DisplaySetter<'gc, B>>,
    ) {
        let prop = DisplayProperty { get, set };
        self.0.insert(name, prop, false);
    }
}

fn x<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.x().into()
}

fn set_x<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    if let Some(val) = property_coerce_to_number(activation, val)? {
        this.set_x(activation.context.gc_context, val);
    }
    Ok(())
}

fn y<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.y().into()
}

fn set_y<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    if let Some(val) = property_coerce_to_number(activation, val)? {
        this.set_y(activation.context.gc_context, val);
    }
    Ok(())
}

fn x_scale<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.scale_x(activation.context.gc_context)
        .into_fraction()
        .into()
}

fn set_x_scale<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    if let Some(val) = property_coerce_to_number(activation, val)? {
        this.set_scale_x(activation.context.gc_context, Percent::from_fraction(val));
    }
    Ok(())
}

fn y_scale<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.scale_y(activation.context.gc_context)
        .into_fraction()
        .into()
}

fn set_y_scale<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    if let Some(val) = property_coerce_to_number(activation, val)? {
        this.set_scale_y(activation.context.gc_context, Percent::from_fraction(val));
    }
    Ok(())
}

fn current_frame<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.as_movie_clip()
        .map(MovieClip::current_frame)
        .map_or(Value::Undefined, Value::from)
}

fn total_frames<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.as_movie_clip()
        .map(MovieClip::total_frames)
        .map_or(Value::Undefined, Value::from)
}

fn alpha<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    (this.alpha() * 100.0).into()
}

fn set_alpha<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    if let Some(val) = property_coerce_to_number(activation, val)? {
        this.set_alpha(activation.context.gc_context, val / 100.0);
    }
    Ok(())
}

fn visible<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.visible().into()
}

fn set_visible<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    // Because this property dates to the era of Flash 4, this is actually coerced to an integer.
    // `_visible = "false";` coerces to NaN and has no effect.
    if let Some(n) = property_coerce_to_number(activation, val)? {
        this.set_visible(activation.context.gc_context, n != 0.0);
    }
    Ok(())
}

fn width<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.width().into()
}

fn set_width<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    if let Some(val) = property_coerce_to_number(activation, val)? {
        this.set_width(activation.context.gc_context, val);
    }
    Ok(())
}

fn height<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.height().into()
}

fn set_height<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    if let Some(val) = property_coerce_to_number(activation, val)? {
        this.set_height(activation.context.gc_context, val);
    }
    Ok(())
}

fn rotation<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    let degrees: f64 = this.rotation(activation.context.gc_context).into();
    degrees.into()
}

fn set_rotation<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
    degrees: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    if let Some(mut degrees) = property_coerce_to_number(activation, degrees)? {
        // Normalize into the range of [-180, 180].
        degrees %= 360.0;
        if degrees < -180.0 {
            degrees += 360.0
        } else if degrees > 180.0 {
            degrees -= 360.0
        }
        this.set_rotation(activation.context.gc_context, degrees.into());
    }
    Ok(())
}

fn target<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    AvmString::new(activation.context.gc_context, this.slash_path()).into()
}

fn frames_loaded<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.as_movie_clip()
        .map(MovieClip::frames_loaded)
        .map_or(Value::Undefined, Value::from)
}

fn name<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.name().into()
}

fn set_name<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    let name = val.coerce_to_string(activation)?;
    this.set_name(activation.context.gc_context, name);
    Ok(())
}

fn drop_target<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.as_movie_clip()
        .and_then(|mc| mc.drop_target())
        .map_or_else(
            || "".into(),
            |drop_target| {
                AvmString::new(activation.context.gc_context, drop_target.slash_path()).into()
            },
        )
}

fn url<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    this.as_movie_clip()
        .and_then(|mc| mc.movie())
        .and_then(|mov| mov.url().map(|url| url.to_string()))
        .map_or_else(
            || "".into(),
            |s| AvmString::new_utf8(activation.context.gc_context, s).into(),
        )
}

fn high_quality<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    use ruffle_types::display_object::stage::StageQuality;
    let quality = match activation.context.stage.quality() {
        StageQuality::Best => 2,
        StageQuality::High => 1,
        _ => 0,
    };
    quality.into()
}

fn set_high_quality<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    use ruffle_types::display_object::stage::StageQuality;
    let val = val.coerce_to_f64(activation)?;
    if !val.is_nan() {
        // 0 -> Low, 1 -> High, 2 -> Best, but with some odd rules for non-integers.
        let quality = if val > 1.5 {
            StageQuality::Best
        } else if val == 0.0 {
            StageQuality::Low
        } else {
            StageQuality::High
        };
        activation
            .context
            .stage
            .set_quality(activation.context.gc_context, quality);
    }
    Ok(())
}

fn focus_rect<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    avm_warn!(activation, "Unimplemented property _focusrect");
    Value::Null
}

fn set_focus_rect<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: DisplayObject<'gc, B>,
    _val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    avm_warn!(activation, "Unimplemented property _focusrect");
    Ok(())
}

fn sound_buf_time<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    activation.context.audio_manager.stream_buffer_time().into()
}

fn set_sound_buf_time<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    avm_warn!(activation, "_soundbuftime is currently ignored by Ruffle");
    if let Some(val) = property_coerce_to_i32(activation, val)? {
        activation
            .context
            .audio_manager
            .set_stream_buffer_time(val as i32);
    }
    Ok(())
}

fn quality<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    let quality = activation.context.stage.quality().into_avm_str();
    quality.into()
}

fn set_quality<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: DisplayObject<'gc, B>,
    val: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    if let Ok(quality) = val.coerce_to_string(activation)?.parse() {
        activation
            .context
            .stage
            .set_quality(activation.context.gc_context, quality);
    }
    Ok(())
}

fn x_mouse<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    let (local_x, _) = this.global_to_local(*activation.context.mouse_position);
    local_x.to_pixels().into()
}

fn y_mouse<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: DisplayObject<'gc, B>,
) -> Value<'gc, B> {
    let (_, local_y) = this.global_to_local(*activation.context.mouse_position);
    local_y.to_pixels().into()
}

fn property_coerce_to_number<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    value: Value<'gc, B>,
) -> Result<Option<f64>, Error<'gc, B>> {
    if value != Value::Undefined && value != Value::Null {
        let n = value.coerce_to_f64(activation)?;
        if n.is_finite() {
            return Ok(Some(n));
        }
    }

    // Invalid value; do not set.
    Ok(None)
}

/// Coerces `value` to `i32` for use by a stage object property.
///
/// Values out of range of `i32` will be clamped to `i32::MIN`. Returns `None` if the value is
/// invalid (NaN, null, or undefined).
fn property_coerce_to_i32<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    value: Value<'gc, B>,
) -> Result<Option<i32>, Error<'gc, B>> {
    let n = value.coerce_to_f64(activation)?;
    let ret = if n.is_nan() {
        // NaN/undefined/null are invalid values; do not set.
        None
    } else if n >= i32::MIN as f64 && n <= i32::MAX as f64 {
        Some(n as i32)
    } else {
        // Out of range of i32; snaps to `i32::MIN`.
        Some(i32::MIN)
    };

    Ok(ret)
}
