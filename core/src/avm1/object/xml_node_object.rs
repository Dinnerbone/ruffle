//! AVM1 object type to represent XML nodes

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::object::TObject;
use crate::avm1::{Object, ScriptObject};
use crate::impl_custom_object;
use crate::xml::{XmlNode, TEXT_NODE};
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::fmt;

/// A ScriptObject that is inherently tied to an XML node.
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct XmlNodeObject<'gc, B: Backend>(GcCell<'gc, XmlNodeObjectData<'gc, B>>);

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct XmlNodeObjectData<'gc, B: Backend> {
    base: ScriptObject<'gc, B>,
    node: XmlNode<'gc, B>,
}

impl<'gc, B: Backend> XmlNodeObject<'gc, B> {
    /// Construct an XmlNodeObject for an already existing node.
    pub fn from_xml_node(
        gc_context: MutationContext<'gc, '_>,
        mut node: XmlNode<'gc, B>,
        proto: Option<Object<'gc, B>>,
    ) -> Self {
        let object = Self(GcCell::allocate(
            gc_context,
            XmlNodeObjectData {
                base: ScriptObject::object(gc_context, proto),
                node,
            },
        ));
        node.introduce_script_object(gc_context, object.into());
        object
    }
}

impl<B: Backend> fmt::Debug for XmlNodeObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("XmlNodeObject")
            .field("base", &this.base)
            .field("node", &this.node)
            .finish()
    }
}

impl<'gc, B: Backend> TObject<'gc> for XmlNodeObject<'gc, B> {
    type B = B;

    impl_custom_object!(B, base);

    fn create_bare_object(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
    ) -> Result<Object<'gc, B>, Error<'gc, B>> {
        Ok(Self::from_xml_node(
            activation.context.gc_context,
            XmlNode::new(activation.context.gc_context, TEXT_NODE, Some("".into())),
            Some(this),
        )
        .into())
    }

    fn as_xml_node(&self) -> Option<XmlNode<'gc, B>> {
        Some(self.0.read().node)
    }
}
