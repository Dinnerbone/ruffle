//! AVM1 object type to represent XML documents

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::{Object, ScriptObject, TObject};
use crate::impl_custom_object;
use crate::xml::{XmlNode, ELEMENT_NODE, TEXT_NODE};
use gc_arena::{Collect, GcCell, MutationContext};
use quick_xml::{events::Event, Reader};
use ruffle_types::backend::Backend;
use ruffle_types::string::{AvmString, WStr, WString};
use std::fmt;

#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum XmlStatus {
    /// No error; parse was completed successfully.
    NoError = 0,

    /// A CDATA section was not properly terminated.
    #[allow(dead_code)]
    CdataNotTerminated = -2,

    /// The XML declaration was not properly terminated.
    DeclNotTerminated = -3,

    /// The DOCTYPE declaration was not properly terminated.
    #[allow(dead_code)]
    DoctypeNotTerminated = -4,

    /// A comment was not properly terminated.
    #[allow(dead_code)]
    CommentNotTerminated = -5,

    /// An XML element was malformed.
    ElementMalformed = -6,

    /// Out of memory.
    OutOfMemory = -7,

    /// An attribute value was not properly terminated.
    AttributeNotTerminated = -8,

    /// A start-tag was not matched with an end-tag.
    #[allow(dead_code)]
    MismatchedStart = -9,

    /// An end-tag was encountered without a matching start-tag.
    MismatchedEnd = -10,
}

/// A ScriptObject that is inherently tied to an XML document.
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct XmlObject<'gc, B: Backend>(GcCell<'gc, XmlObjectData<'gc, B>>);

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct XmlObjectData<'gc, B: Backend> {
    base: ScriptObject<'gc, B>,

    /// The root node of the XML document.
    root: XmlNode<'gc, B>,

    /// The XML declaration, if set.
    xml_decl: Option<AvmString<'gc>>,

    /// The XML doctype, if set.
    doctype: Option<AvmString<'gc>>,

    /// The document's ID map.
    ///
    /// When nodes are parsed into the document by way of `parseXML` or the
    /// document constructor, they get put into this object, which is accessible
    /// through the document's `idMap`.
    id_map: ScriptObject<'gc, B>,

    /// The last parse error encountered, if any.
    status: XmlStatus,
}

impl<'gc, B: Backend> XmlObject<'gc, B> {
    /// Construct a new XML document and object pair.
    pub fn empty(gc_context: MutationContext<'gc, '_>, proto: Option<Object<'gc, B>>) -> Self {
        let mut root = XmlNode::new(gc_context, ELEMENT_NODE, None);
        let object = Self(GcCell::allocate(
            gc_context,
            XmlObjectData {
                base: ScriptObject::object(gc_context, proto),
                root,
                xml_decl: None,
                doctype: None,
                id_map: ScriptObject::bare_object(gc_context),
                status: XmlStatus::NoError,
            },
        ));
        root.introduce_script_object(gc_context, object.into());
        object
    }

    /// Yield the document in node form.
    pub fn as_node(self) -> XmlNode<'gc, B> {
        self.0.read().root
    }

    /// Retrieve the XML declaration of this document.
    pub fn xml_decl(self) -> Option<AvmString<'gc>> {
        self.0.read().xml_decl
    }

    /// Retrieve the first DocType node in the document.
    pub fn doctype(self) -> Option<AvmString<'gc>> {
        self.0.read().doctype
    }

    /// Replace the contents of this document with the result of parsing a string.
    ///
    /// This method does not yet actually remove existing node contents.
    pub fn replace_with_str(
        &mut self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        data: &WStr,
        ignore_white: bool,
    ) -> Result<(), quick_xml::Error> {
        let data_utf8 = data.to_utf8_lossy();
        let mut parser = Reader::from_str(&data_utf8);
        let mut buf = Vec::new();
        let mut open_tags = vec![self.as_node()];

        self.0.write(activation.context.gc_context).status = XmlStatus::NoError;

        loop {
            let event = parser.read_event(&mut buf).map_err(|error| {
                self.0.write(activation.context.gc_context).status = match error {
                    quick_xml::Error::UnexpectedEof(_)
                    | quick_xml::Error::NameWithQuote(_)
                    | quick_xml::Error::NoEqAfterName(_)
                    | quick_xml::Error::DuplicatedAttribute(_, _) => XmlStatus::ElementMalformed,
                    quick_xml::Error::EndEventMismatch { .. } => XmlStatus::MismatchedEnd,
                    quick_xml::Error::XmlDeclWithoutVersion(_) => XmlStatus::DeclNotTerminated,
                    quick_xml::Error::UnquotedValue(_) => XmlStatus::AttributeNotTerminated,
                    _ => XmlStatus::OutOfMemory,
                    // Not accounted for:
                    // quick_xml::Error::UnexpectedToken(_)
                    // quick_xml::Error::UnexpectedBang
                    // quick_xml::Error::TextNotFound
                    // quick_xml::Error::EscapeError(_)
                };
                error
            })?;

            match event {
                Event::Start(bs) => {
                    let child = XmlNode::from_start_event(activation, bs, self.id_map())?;
                    open_tags
                        .last_mut()
                        .unwrap()
                        .append_child(activation.context.gc_context, child);
                    open_tags.push(child);
                }
                Event::Empty(bs) => {
                    let child = XmlNode::from_start_event(activation, bs, self.id_map())?;
                    open_tags
                        .last_mut()
                        .unwrap()
                        .append_child(activation.context.gc_context, child);
                }
                Event::End(_) => {
                    open_tags.pop();
                }
                Event::Text(bt) | Event::CData(bt) => {
                    let text = bt.unescaped()?;
                    let is_whitespace_char = |c: &u8| matches!(*c, b'\t' | b'\n' | b'\r' | b' ');
                    let is_whitespace_text = text.iter().all(is_whitespace_char);
                    if !(text.is_empty() || ignore_white && is_whitespace_text) {
                        let text = AvmString::new_utf8_bytes(activation.context.gc_context, &text);
                        let child =
                            XmlNode::new(activation.context.gc_context, TEXT_NODE, Some(text));
                        open_tags
                            .last_mut()
                            .unwrap()
                            .append_child(activation.context.gc_context, child);
                    }
                }
                Event::Decl(bd) => {
                    let mut xml_decl = WString::from_buf(b"<?".to_vec());
                    xml_decl.push_str(WStr::from_units(bd.as_ref()));
                    xml_decl.push_str(WStr::from_units(b"?>"));
                    self.0.write(activation.context.gc_context).xml_decl =
                        Some(AvmString::new(activation.context.gc_context, xml_decl));
                }
                Event::DocType(bt) => {
                    // TODO: `quick-xml` is case-insensitive for DOCTYPE declarations,
                    // but it doesn't expose the whole tag, only the inner portion of it.
                    // Flash is also case-insensitive for DOCTYPE declarations. However,
                    // the `.docTypeDecl` property preserves the original case.
                    let mut doctype = WString::from_buf(b"<!DOCTYPE".to_vec());
                    doctype.push_str(WStr::from_units(bt.escaped()));
                    doctype.push_byte(b'>');
                    self.0.write(activation.context.gc_context).doctype =
                        Some(AvmString::new(activation.context.gc_context, doctype));
                }
                Event::Eof => break,
                _ => {}
            }
        }

        Ok(())
    }

    /// Obtain the script object for the document's `idMap` property.
    pub fn id_map(self) -> ScriptObject<'gc, B> {
        self.0.read().id_map
    }

    pub fn status(self) -> XmlStatus {
        self.0.read().status
    }
}

impl<B: Backend> fmt::Debug for XmlObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("XmlObject")
            .field("base", &this.base)
            .field("root", &self.0.read().root)
            .finish()
    }
}

impl<'gc, B: Backend> TObject<'gc> for XmlObject<'gc, B> {
    type B = B;

    impl_custom_object!(B, base);

    fn create_bare_object(
        &self,
        activation: &mut Activation<'_, 'gc, '_, B>,
        this: Object<'gc, B>,
    ) -> Result<Object<'gc, B>, Error<'gc, B>> {
        Ok(Self::empty(activation.context.gc_context, Some(this)).into())
    }

    fn as_xml(&self) -> Option<XmlObject<'gc, B>> {
        Some(*self)
    }

    fn as_xml_node(&self) -> Option<XmlNode<'gc, B>> {
        Some(self.as_node())
    }
}
