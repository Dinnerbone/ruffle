#[macro_export]
macro_rules! impl_custom_object {
    ($B:ident, $field:ident) => {
        $crate::impl_custom_object!($B, $field {});
    };

    (@extra $B:ident, $field:ident bare_object($as_obj:ident -> $obj_type:ident :: $new:ident)) => {
        fn $as_obj(&self) -> Option<$obj_type<'gc, $B>> {
            Some(*self)
        }

        fn create_bare_object(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            this: $crate::avm1::Object<'gc, $B>,
        ) -> Result<$crate::avm1::Object<'gc, $B>, $crate::avm1::Error<'gc, $B>> {
            Ok($obj_type::$new(activation.context.gc_context, Some(this)).into())
        }
    };

    ($B:ident, $field:ident {
        $(
            $extra_name:ident($($extra:tt)*);
        )*
    }) => {
        $(
            $crate::impl_custom_object!(@extra $B, $field $extra_name($($extra)*));
        )*

        fn get_local_stored(
            &self,
            name: impl Into<$crate::avm1::AvmString<'gc>>,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
        ) -> Option<$crate::avm1::Value<'gc, $B>> {
            self.0.read().$field.get_local_stored(name, activation)
        }

        fn set_local(
            &self,
            name: $crate::avm1::AvmString<'gc>,
            value: $crate::avm1::Value<'gc, $B>,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            this: $crate::avm1::Object<'gc, $B>,
        ) -> Result<(), $crate::avm1::Error<'gc, $B>> {
            self.0.read().$field.set_local(name, value, activation, this)
        }

        fn call(
            &self,
            name: $crate::avm1::AvmString<'gc>,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            this: $crate::avm1::Value<'gc, $B>,
            args: &[$crate::avm1::Value<'gc, $B>],
        ) -> Result<$crate::avm1::Value<'gc, $B>, $crate::avm1::Error<'gc, $B>> {
            self.0
                .read()
                .$field
                .call(name, activation, this, args)
        }

        fn getter(
            &self,
            name: $crate::avm1::AvmString<'gc>,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
        ) -> Option<$crate::avm1::object::Object<'gc, $B>> {
            self.0.read().$field.getter(name, activation)
        }

        fn setter(
            &self,
            name: $crate::avm1::AvmString<'gc>,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
        ) -> Option<$crate::avm1::object::Object<'gc, $B>> {
            self.0.read().$field.setter(name, activation)
        }

        fn delete(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            name: $crate::avm1::AvmString<'gc>,
        ) -> bool {
            self.0.read().$field.delete(activation, name)
        }

        fn proto(&self, activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>) -> $crate::avm1::Value<'gc, $B> {
            self.0.read().$field.proto(activation)
        }

        fn define_value(
            &self,
            gc_context: gc_arena::MutationContext<'gc, '_>,
            name: impl Into<$crate::avm1::AvmString<'gc>>,
            value: $crate::avm1::Value<'gc, $B>,
            attributes: $crate::avm1::property::Attribute,
        ) {
            self.0
                .read()
                .$field
                .define_value(gc_context, name, value, attributes)
        }

        fn set_attributes(
            &self,
            gc_context: gc_arena::MutationContext<'gc, '_>,
            name: Option<$crate::avm1::AvmString<'gc>>,
            set_attributes: $crate::avm1::property::Attribute,
            clear_attributes: $crate::avm1::property::Attribute,
        ) {
            self.0.write(gc_context).$field.set_attributes(
                gc_context,
                name,
                set_attributes,
                clear_attributes,
            )
        }

        fn add_property(
            &self,
            gc_context: gc_arena::MutationContext<'gc, '_>,
            name: $crate::avm1::AvmString<'gc>,
            get: $crate::avm1::object::Object<'gc, $B>,
            set: Option<$crate::avm1::object::Object<'gc, $B>>,
            attributes: $crate::avm1::property::Attribute,
        ) {
            self.0
                .read()
                .$field
                .add_property(gc_context, name, get, set, attributes)
        }

        fn add_property_with_case(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            name: $crate::avm1::AvmString<'gc>,
            get: $crate::avm1::object::Object<'gc, $B>,
            set: Option<$crate::avm1::object::Object<'gc, $B>>,
            attributes: $crate::avm1::property::Attribute,
        ) {
            self.0
                .read()
                .$field
                .add_property_with_case(activation, name, get, set, attributes)
        }

        fn has_property(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            name: $crate::avm1::AvmString<'gc>,
        ) -> bool {
            self.0.read().$field.has_property(activation, name)
        }

        fn has_own_property(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            name: $crate::avm1::AvmString<'gc>,
        ) -> bool {
            self.0.read().$field.has_own_property(activation, name)
        }

        fn has_own_virtual(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            name: $crate::avm1::AvmString<'gc>,
        ) -> bool {
            self.0.read().$field.has_own_virtual(activation, name)
        }

        fn is_property_enumerable(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            name: $crate::avm1::AvmString<'gc>,
        ) -> bool {
            self.0
                .read()
                .$field
                .is_property_enumerable(activation, name)
        }

        fn get_keys(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
        ) -> Vec<$crate::avm1::AvmString<'gc>> {
            self.0.read().$field.get_keys(activation)
        }

        fn interfaces(&self) -> Vec<$crate::avm1::Object<'gc, $B>> {
            self.0.read().$field.interfaces()
        }

        fn set_interfaces(
            &self,
            gc_context: gc_arena::MutationContext<'gc, '_>,
            iface_list: Vec<$crate::avm1::Object<'gc, $B>>,
        ) {
            self.0
                .write(gc_context)
                .$field
                .set_interfaces(gc_context, iface_list)
        }

        fn as_script_object(&self) -> Option<ScriptObject<'gc, $B>> {
            Some(self.0.read().$field)
        }

        fn as_ptr(&self) -> *const $crate::avm1::ObjectPtr {
            self.0.as_ptr() as *const $crate::avm1::ObjectPtr
        }

        fn length(&self, activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>) -> Result<i32, $crate::avm1::Error<'gc, $B>> {
            self.0.read().$field.length(activation)
        }

        fn set_length(&self, activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>, length: i32) -> Result<(), $crate::avm1::Error<'gc, $B>> {
            self.0.read().$field.set_length(activation, length)
        }

        fn has_element(&self, activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>, index: i32) -> bool {
            self.0.read().$field.has_element(activation, index)
        }

        fn get_element(&self, activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>, index: i32) -> $crate::avm1::Value<'gc, $B> {
            self.0.read().$field.get_element(activation, index)
        }

        fn set_element(&self, activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>, index: i32, value: $crate::avm1::Value<'gc, $B>) -> Result<(), $crate::avm1::Error<'gc, $B>> {
            self.0.read().$field.set_element(activation, index, value)
        }

        fn delete_element(&self, activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>, index: i32) -> bool {
            self.0.read().$field.delete_element(activation, index)
        }

        fn call_watcher(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            name: $crate::avm1::AvmString<'gc>,
            value: &mut $crate::avm1::Value<'gc, $B>,
            this: $crate::avm1::object::Object<'gc, $B>,
        ) -> Result<(), $crate::avm1::Error<'gc, $B>> {
            self.0.read().$field.call_watcher(activation, name, value, this)
        }

        fn watch(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            name: $crate::avm1::AvmString<'gc>,
            callback: $crate::avm1::object::Object<'gc, $B>,
            user_data: $crate::avm1::Value<'gc, $B>,
        ) {
            self.0
                .read()
                .$field
                .watch(activation, name, callback, user_data);
        }

        fn unwatch(
            &self,
            activation: &mut $crate::avm1::Activation<'_, 'gc, '_, $B>,
            name: $crate::avm1::AvmString<'gc>,
        ) -> bool {
            self.0.read().$field.unwatch(activation, name)
        }
    };
}

#[macro_export]
macro_rules! add_field_accessors {
    ($([$set_ident: ident, $get_ident: ident, $($var: ident).+, $type_: ty],)*) => {
        add_field_accessors!(
            $([$($var).+, $type_, set => $set_ident, get => $get_ident],)*
        );
    };

    ($([$($var: ident).+, $type_: ty $(, set => $set_ident: ident)? $(, get => $get_ident: ident)?],)*) => {
        $(
            add_field_accessors!([single $($var).+, $type_ $(, set => $set_ident)? $(, get => $get_ident)?]);
        )*
    };


    // This intermediate stage is here because I couldn't figure out how to make the nested
    // repetitions of $var and the optional $set_ident and $get_ident all expand correctly.
    ([single $($var: ident).+, $type_: ty, set => $set_ident: ident]) => {
        add_field_accessors!([setter_only $set_ident, $($var).+, $type_],);
    };
    ([single $($var: ident).+, $type_: ty, get => $get_ident: ident]) => {
        add_field_accessors!([getter_only $get_ident, $($var).+, $type_],);
    };
    ([single $($var: ident).+, $type_: ty, set => $set_ident: ident, get => $get_ident: ident]) => {
        add_field_accessors!([getter_only $get_ident, $($var).+, $type_],);
        add_field_accessors!([setter_only $set_ident, $($var).+, $type_],);
    };
    ([single $($var: ident).+, $type_: ty]) => {
        // nothing
    };


    ($([getter_only $get_ident: ident, $($var: ident).+, $type_: ty],)*) => {
        $(
            pub fn $get_ident(&self) -> $type_ {
                self.0.read().$($var).+
            }
        )*
    };

    ($([setter_only $set_ident: ident, $($var: ident).+, $type_: ty],)*) => {
        $(
            pub fn $set_ident(&self, gc_context: MutationContext<'gc, '_>, v: $type_) {
                self.0.write(gc_context).$($var).+ = v;
            }
        )*
    };
}
