//! Represents AVM2 scope chain resolution.

use crate::avm2::activation::Activation;
use crate::avm2::domain::Domain;
use crate::avm2::names::Multiname;
use crate::avm2::object::{Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{Collect, Gc, MutationContext};
use ruffle_types::backend::Backend;
use std::ops::Deref;

/// Represents a Scope that can be on either a ScopeChain or local ScopeStack.
#[derive(Debug, Collect, Clone, Copy)]
#[collect(no_drop)]
pub struct Scope<'gc, B: Backend> {
    /// The underlying object of this Scope
    values: Object<'gc, B>,

    /// Indicates whether or not this is a `with` scope.
    ///
    /// A `with` scope allows searching the dynamic properties of
    /// this scope.
    with: bool,
}

impl<'gc, B: Backend> Scope<'gc, B> {
    /// Creates a new regular Scope
    pub fn new(values: Object<'gc, B>) -> Self {
        Self {
            values,
            with: false,
        }
    }

    /// Creates a new `with` Scope
    pub fn new_with(values: Object<'gc, B>) -> Self {
        Self { values, with: true }
    }

    pub fn with(&self) -> bool {
        self.with
    }

    pub fn values(&self) -> Object<'gc, B> {
        self.values
    }
}

/// A ScopeChain "chains" scopes together.
///
/// A ScopeChain is used for "remembering" what a scope looked like. A ScopeChain also
/// contains an associated Domain that should be the domain that was in use during it's
/// initial creation.
///
/// A ScopeChain is either created by chaining new scopes on top of an already existing
/// ScopeChain, or if we havn't created one yet (like during script initialization), you can
/// create an empty ScopeChain with only a Domain. A ScopeChain should **always** have a Domain.
///
/// ScopeChain's are copy-on-write, meaning when we chain new scopes on top of a ScopeChain, we
/// actually create a completely brand new ScopeChain. The Domain of the ScopeChain we are chaining
/// on top of will be used for the new ScopeChain.
#[derive(Debug, Collect, Clone, Copy)]
#[collect(no_drop)]
pub struct ScopeChain<'gc, B: Backend> {
    scopes: Option<Gc<'gc, Vec<Scope<'gc, B>>>>,
    domain: Domain<'gc, B>,
}

impl<'gc, B: Backend> ScopeChain<'gc, B> {
    /// Creates a brand new ScopeChain with a domain. The domain should be the current domain in use.
    pub fn new(domain: Domain<'gc, B>) -> Self {
        Self {
            scopes: None,
            domain,
        }
    }

    /// Creates a new ScopeChain by chaining new scopes on top of this ScopeChain
    pub fn chain(&self, mc: MutationContext<'gc, '_>, new_scopes: &[Scope<'gc, B>]) -> Self {
        if new_scopes.is_empty() {
            // If we are not actually adding any new scopes, we don't need to do anything.
            return *self;
        }
        // TODO: This current implementation is a bit expensive, but it is exactly what avmplus does, so it's good enough for now.
        match self.scopes {
            Some(scopes) => {
                // The new ScopeChain is created by cloning the scopes of this ScopeChain,
                // and pushing the new scopes on top of that.
                let mut cloned = scopes.deref().clone();
                cloned.extend_from_slice(new_scopes);
                Self {
                    scopes: Some(Gc::allocate(mc, cloned)),
                    domain: self.domain,
                }
            }
            None => {
                // We are chaining on top of an empty ScopeChain, so we don't actually
                // need to chain anything.
                Self {
                    scopes: Some(Gc::allocate(mc, new_scopes.to_vec())),
                    domain: self.domain,
                }
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<Scope<'gc, B>> {
        self.scopes.and_then(|scopes| scopes.get(index).cloned())
    }

    pub fn is_empty(&self) -> bool {
        self.scopes.map(|scopes| scopes.is_empty()).unwrap_or(true)
    }

    /// Returns the domain associated with this ScopeChain.
    pub fn domain(&self) -> Domain<'gc, B> {
        self.domain
    }

    #[allow(clippy::collapsible_if)]
    pub fn find(
        &self,
        multiname: &Multiname<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Result<Option<Object<'gc, B>>, Error> {
        // First search our scopes
        if let Some(scopes) = self.scopes {
            for (depth, scope) in scopes.iter().enumerate().rev() {
                let values = scope.values();

                // We search the dynamic properties if either conditions are met:
                // 1. Scope is a `with` scope
                // 2. We are at depth 0 (global scope)
                //
                // But no matter what, we always search traits first.
                if values.has_trait(multiname) {
                    return Ok(Some(values));
                } else if scope.with() || depth == 0 {
                    if values.has_own_property(multiname) {
                        return Ok(Some(values));
                    }
                }
            }
        }
        // That didn't work... let's try searching the domain now.
        if let Some((_qname, mut script)) = self.domain.get_defining_script(multiname)? {
            return Ok(Some(script.globals(&mut activation.context)?));
        }
        Ok(None)
    }

    pub fn resolve(
        &self,
        name: &Multiname<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
    ) -> Result<Option<Value<'gc, B>>, Error> {
        if let Some(object) = self.find(name, activation)? {
            Ok(Some(object.get_property(name, activation)?))
        } else {
            Ok(None)
        }
    }
}

/// Represents a ScopeStack to be used in the AVM2 activation. A new ScopeStack should be created
/// per activation. A ScopeStack allows mutations, such as pushing new scopes, or popping scopes off.
/// A ScopeStack should only ever be accessed by the activation it was created in.
#[derive(Debug, Collect, Clone)]
#[collect(no_drop)]
pub struct ScopeStack<'gc, B: Backend> {
    scopes: Vec<Scope<'gc, B>>,
}

impl<'gc, B: Backend> ScopeStack<'gc, B> {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.scopes.clear();
    }

    pub fn push(&mut self, scope: Scope<'gc, B>) {
        self.scopes.push(scope);
    }

    pub fn pop(&mut self) -> Option<Scope<'gc, B>> {
        self.scopes.pop()
    }

    pub fn get(&self, index: usize) -> Option<Scope<'gc, B>> {
        self.scopes.get(index).cloned()
    }

    pub fn scopes(&self) -> &[Scope<'gc, B>] {
        &self.scopes
    }

    /// Searches for a scope in this ScopeStack by a multiname.
    ///
    /// The `global` parameter indicates whether we are on global$init (script initializer).
    /// When the `global` parameter is true, the scope at depth 0 is considered the global scope, and is
    /// searched for dynamic properties.
    #[allow(clippy::collapsible_if)]
    pub fn find(
        &self,
        multiname: &Multiname<'gc>,
        global: bool,
    ) -> Result<Option<Object<'gc, B>>, Error> {
        for (depth, scope) in self.scopes.iter().enumerate().rev() {
            let values = scope.values();

            if values.has_trait(multiname) {
                return Ok(Some(values));
            } else if scope.with() || (global && depth == 0) {
                // We search the dynamic properties if either conditions are met:
                // 1. Scope is a `with` scope
                // 2. We are at depth 0 AND we are at global$init (script initializer).
                if values.has_own_property(multiname) {
                    return Ok(Some(values));
                }
            }
        }
        Ok(None)
    }
}
