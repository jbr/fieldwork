use std::borrow::Cow;

use crate::{
    DEFAULT_AUTO_COPY, DEFAULT_AUTO_DEREF, DEFAULT_CHAINABLE_SET, DEFAULT_OPTION_HANDLING,
    DEFAULT_RENAME_PREDICATES, Field, FieldAttributes, FieldMethodAttributes, Method, Resolved,
    StructAttributes, StructMethodAttributes,
    copy_detection::{enable_copy_for_type, is_type},
    deref_handling::auto_deref,
    option_handling::{extract_option_type, strip_ref},
};
use Method::{Get, GetMut, Set, With};
use syn::{Ident, Type, Visibility, token::Pub};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Query<'a> {
    method: &'a Method,
    field: &'a Field,
    struct_attributes: &'a StructAttributes,
}

impl<'a> Query<'a> {
    pub(crate) fn new(
        method: &'a Method,
        field: &'a Field,
        struct_attributes: &'a StructAttributes,
    ) -> Self {
        Self {
            method,
            field,
            struct_attributes,
        }
    }

    fn field_method_attribute(&self) -> Option<&'a FieldMethodAttributes> {
        self.field
            .attributes
            .method_attributes
            .iter()
            .find(|x| x.method == *self.method)
    }

    fn struct_method_attribute(&self) -> Option<&'a StructMethodAttributes> {
        self.struct_attributes
            .methods
            .iter()
            .find(|x| x.method == *self.method)
    }

    fn is_get_copy(&self) -> bool {
        if let Some(field_copy) = self.field_method_attribute().and_then(|fva| fva.get_copy) {
            return field_copy;
        }

        self.struct_method_attribute()
            .and_then(|x| x.auto_copy)
            .unwrap_or(DEFAULT_AUTO_COPY)
            && enable_copy_for_type(&self.field.ty)
    }

    fn chainable_set(&self) -> bool {
        self.method == &Set
            && self
                .field_method_attribute()
                .and_then(|fva| fva.chainable_set)
                .or(self
                    .struct_method_attribute()
                    .and_then(|sva| sva.chainable_set))
                .unwrap_or(DEFAULT_CHAINABLE_SET)
    }

    fn vis(&self) -> Cow<'a, Visibility> {
        if let Some(vis) = self.field_method_attribute().and_then(|x| x.vis.as_ref()) {
            return Cow::Borrowed(vis);
        }
        if let Some(vis) = self.struct_method_attribute().and_then(|x| x.vis.as_ref()) {
            return Cow::Borrowed(vis);
        }
        if let Some(vis) = &self.field.attributes.vis {
            return Cow::Borrowed(vis);
        }
        if let Some(vis) = &self.struct_attributes.vis {
            return Cow::Borrowed(vis);
        }

        Cow::Owned(Visibility::Public(Pub::default()))
    }

    fn rename_predicates(&self) -> bool {
        [
            self.field_method_attribute()
                .and_then(|x| x.rename_predicates),
            self.field.attributes.rename_predicates,
            self.struct_method_attribute()
                .and_then(|x| x.rename_predicates),
            self.struct_attributes.rename_predicates,
        ]
        .into_iter()
        .find_map(|x| x)
        .unwrap_or(DEFAULT_RENAME_PREDICATES)
    }

    fn fn_ident(&self) -> Cow<'a, Ident> {
        if let Some(fn_ident) = self
            .field_method_attribute()
            .and_then(|x| x.fn_ident.as_ref())
        {
            return Cow::Borrowed(fn_ident);
        }

        let ident = self
            .field
            .attributes
            .fn_ident
            .as_ref()
            .unwrap_or(&self.field.ident);

        if let Some(template) = self
            .struct_method_attribute()
            .and_then(|x| x.template.as_ref())
        {
            return Cow::Owned(Ident::new(
                &template.as_str().replacen("{}", &ident.to_string(), 1),
                self.field.ident.span(),
            ));
        }

        match self.method {
            Get if self.rename_predicates() && is_type(&self.field.ty, "bool") => {
                Cow::Owned(Ident::new(&format!("is_{ident}"), self.field.ident.span()))
            }
            Get => Cow::Borrowed(ident),
            Set => Cow::Owned(Ident::new(&format!("set_{ident}"), self.field.ident.span())),
            With => Cow::Owned(Ident::new(
                &format!("with_{ident}"),
                self.field.ident.span(),
            )),
            GetMut => Cow::Owned(Ident::new(&format!("{ident}_mut"), self.field.ident.span())),
        }
    }

    fn variable_ident(&self) -> Cow<'a, Ident> {
        Cow::Borrowed(&self.field.ident)
    }

    fn argument_ident(&self) -> Cow<'a, Ident> {
        if let Some(argument_ident) = self
            .field_method_attribute()
            .and_then(|x| x.argument_ident.as_ref())
        {
            return Cow::Borrowed(argument_ident);
        }

        if let Some(argument_ident) = self.field.attributes.argument_ident.as_ref() {
            return Cow::Borrowed(argument_ident);
        }

        if let Some(renamed) = self.field.attributes.fn_ident.as_ref() {
            return Cow::Borrowed(renamed);
        }

        Cow::Borrowed(&self.field.ident)
    }

    fn doc_template(&self) -> &str {
        match self.method {
            Get if self.is_get_copy() => "Returns a copy of {}",
            Get => "Borrows {}",
            Set if self.chainable_set() => "Sets {}, returning `&mut Self` for chaining",
            Set => "Sets {}",
            With => "Owned chainable setter for {}, returning `Self`",
            GetMut => "Mutably borrow {}",
        }
    }

    fn docs(&self) -> Option<Cow<'a, str>> {
        if let Some(explicit_method_doc) =
            self.field_method_attribute().and_then(|x| x.doc.as_ref())
        {
            return Some(Cow::Borrowed(explicit_method_doc));
        }

        let first_line = self.field.doc.first()?;

        let template = self
            .struct_method_attribute()
            .and_then(|x| x.doc_template.as_deref())
            .unwrap_or(self.doc_template());

        let mut doc = template.replacen("{}", first_line, 1);

        if self.field.doc.len() > 1 {
            doc.push('\n');
            doc.push_str(&self.field.doc[1..].join("\n"));
        }
        Some(Cow::Owned(doc))
    }

    fn enabled(&self) -> bool {
        let struct_method_attr = self.struct_method_attribute();
        let field_method_attr = self.field_method_attribute();
        let StructAttributes {
            include, opt_in, ..
        } = self.struct_attributes;
        let FieldAttributes {
            decorated,
            skip,
            opt_in: field_opt_in,
            ..
        } = self.field.attributes;

        if *opt_in || field_opt_in {
            decorated
                && ((self.field.attributes.method_attributes.is_empty()
                    && include.contains(self.method))
                    || field_method_attr.is_some_and(|x| !x.skip))
        } else if !include.contains(self.method) {
            field_method_attr.is_some_and(|x| !x.skip)
        } else {
            field_method_attr.is_none_or(|x| !x.skip)
                && struct_method_attr.is_none_or(|x| !x.skip)
                && !skip
        }
    }

    fn auto_deref(&self, ty: &'a Type) -> Option<Cow<'a, Type>> {
        let enabled = [
            self.field_method_attribute().and_then(|x| x.auto_deref),
            self.field.attributes.auto_deref,
            self.struct_method_attribute().and_then(|x| x.auto_deref),
            self.struct_attributes.auto_deref,
        ]
        .into_iter()
        .find_map(|x| x)
        .unwrap_or(DEFAULT_AUTO_DEREF);

        if enabled { auto_deref(ty) } else { None }
    }

    fn deref_type(&self) -> Option<Cow<'a, Type>> {
        self.field_method_attribute()
            .and_then(|x| x.deref.as_ref())
            .or(self.field.attributes.deref.as_ref())
            .map(strip_ref)
            .map(Cow::Borrowed)
            .or_else(|| self.auto_deref(&self.field.ty))
    }

    pub(crate) fn resolve(&self) -> Option<Resolved<'a>> {
        if !self.enabled() {
            return None;
        }
        let method = *self.method;
        let vis = self.vis();
        let fn_ident = self.fn_ident();
        let variable_ident = self.variable_ident();
        let argument_ident = self.argument_ident();
        let doc = self.docs();
        let deref_type = self.deref_type();
        let ty = &self.field.ty;
        let chainable_set = self.chainable_set();
        let get_copy = self.is_get_copy();
        let option_handling = self.option_handling();

        Some(Resolved {
            method,
            vis,
            fn_ident,
            variable_ident,
            argument_ident,
            ty,
            doc,
            get_copy,
            chainable_set,
            deref_type,
            option_handling,
        })
    }

    fn option_handling(&self) -> Option<OptionHandling<'a>> {
        self.field_method_attribute()
            .and_then(|x| x.option_handling)
            .or(self.field.attributes.option_handling)
            .or(self
                .struct_method_attribute()
                .and_then(|sma| sma.option_handling))
            .or(self.struct_attributes.option_handling)
            .unwrap_or(DEFAULT_OPTION_HANDLING)
            .then_some(())?;

        let ty = extract_option_type(&self.field.ty)?;

        if let Some(deref_ty) = self
            .field_method_attribute()
            .and_then(|x| x.deref.as_ref())
            .or(self.field.attributes.deref.as_ref())
        {
            let deref_ty = extract_option_type(deref_ty)?;
            Some(OptionHandling::Deref(Cow::Borrowed(deref_ty)))
        } else {
            self.auto_deref(ty)
                .map(OptionHandling::Deref)
                .or_else(|| Some(OptionHandling::Ref(Cow::Borrowed(strip_ref(ty)))))
        }
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) enum OptionHandling<'a> {
    Ref(Cow<'a, Type>),
    Deref(Cow<'a, Type>),
}
