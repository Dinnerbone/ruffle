use crate::avm1::return_value::ReturnValue;
use crate::avm1::{Avm1, Error, Object, ScriptObject, TObject, UpdateContext, Value};

use gc_arena::{Collect, MutationContext};
use crate::backend::Backends;

#[derive(Clone, Collect, Debug, Copy)]
#[collect(no_drop)]
pub struct Listeners<'gc, B: Backends>(Object<'gc, B>);

macro_rules! register_listener {
    ( $gc_context: ident, $object:ident, $listener: ident, $fn_proto: ident, $system_listeners_key: ident ) => {{
        pub fn add_listener<'gc, B: Backends>(
            avm: &mut Avm1<'gc, B>,
            context: &mut UpdateContext<'_, 'gc, '_, B>,
            _this: Object<'gc, B>,
            args: &[Value<'gc, B>],
        ) -> Result<ReturnValue<'gc, B>, Error> {
            avm.system_listeners
                .$system_listeners_key
                .add_listener(context, args)
        }

        pub fn remove_listener<'gc, B: Backends>(
            avm: &mut Avm1<'gc, B>,
            context: &mut UpdateContext<'_, 'gc, '_, B>,
            _this: Object<'gc, B>,
            args: &[Value<'gc, B>],
        ) -> Result<ReturnValue<'gc, B>, Error> {
            let listener = avm.system_listeners.$system_listeners_key;
            listener.remove_listener(avm, context, args)
        }

        $object.define_value(
            $gc_context,
            "_listeners",
            $listener.object().into(),
            Attribute::DontEnum | Attribute::DontDelete | Attribute::ReadOnly,
        );

        $object.force_set_function(
            "addListener",
            add_listener,
            $gc_context,
            Attribute::DontEnum | Attribute::DontDelete | Attribute::ReadOnly,
            $fn_proto,
        );

        $object.force_set_function(
            "removeListener",
            remove_listener,
            $gc_context,
            Attribute::DontEnum | Attribute::DontDelete | Attribute::ReadOnly,
            $fn_proto,
        );
    }};
}

impl<'gc, B: Backends> Listeners<'gc, B> {
    pub fn new(gc_context: MutationContext<'gc, '_>, array_proto: Option<Object<'gc, B>>) -> Self {
        Self(ScriptObject::array(gc_context, array_proto).into())
    }

    pub fn add_listener(
        &self,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        args: &[Value<'gc, B>],
    ) -> Result<ReturnValue<'gc, B>, Error> {
        let listeners = self.0;
        let listener = args.get(0).unwrap_or(&Value::Undefined).to_owned();
        for i in 0..listeners.length() {
            if listeners.array_element(i) == listener {
                return Ok(true.into());
            }
        }

        listeners.set_array_element(listeners.length(), listener, context.gc_context);
        Ok(true.into())
    }

    pub fn remove_listener(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        args: &[Value<'gc, B>],
    ) -> Result<ReturnValue<'gc, B>, Error> {
        let listeners = self.0;
        let listener = args.get(0).unwrap_or(&Value::Undefined).to_owned();
        for index in 0..listeners.length() {
            if listeners.array_element(index) == listener {
                let new_length = listeners.length() - 1;

                for i in index..new_length {
                    listeners.set_array_element(
                        i,
                        listeners.array_element(i + 1),
                        context.gc_context,
                    );
                }

                listeners.delete_array_element(new_length, context.gc_context);
                listeners.delete(avm, context.gc_context, &new_length.to_string());
                listeners.set_length(context.gc_context, new_length);

                return Ok(true.into());
            }
        }

        Ok(false.into())
    }

    pub fn prepare_handlers(
        &self,
        avm: &mut Avm1<'gc, B>,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        method: &str,
    ) -> Vec<(Object<'gc, B>, Value<'gc, B>)> {
        let listeners = self.0;
        let mut handlers = Vec::with_capacity(listeners.length());

        for i in 0..listeners.length() {
            if let Ok(listener) = listeners.array_element(i).as_object() {
                if let Ok(handler) = listener
                    .get(method, avm, context)
                    .and_then(|v| v.resolve(avm, context))
                {
                    handlers.push((listener, handler));
                }
            }
        }

        handlers
    }

    pub fn object(&self) -> Object<'gc, B> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SystemListener {
    Mouse,
}

#[derive(Clone, Collect, Debug, Copy)]
#[collect(no_drop)]
pub struct SystemListeners<'gc, B: Backends> {
    pub mouse: Listeners<'gc, B>,
}

impl<'gc, B: Backends> SystemListeners<'gc, B> {
    pub fn new(gc_context: MutationContext<'gc, '_>, array_proto: Option<Object<'gc, B>>) -> Self {
        Self {
            mouse: Listeners::new(gc_context, array_proto),
        }
    }

    pub fn get(&self, listener: SystemListener) -> Listeners<'gc, B> {
        match listener {
            SystemListener::Mouse => self.mouse,
        }
    }
}
