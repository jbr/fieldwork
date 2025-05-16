use std::borrow::Cow;

use crate::{
    DEFAULT_CHAINABLE_SET, Field, FieldAttributes, FieldMethodAttributes, Method, StructAttributes,
    StructMethodAttributes,
};
use Method::{Get, GetMut, Set, With};
use syn::{Ident, Type, Visibility, token::Pub};

#[derive(Debug)]
pub(crate) struct Query<'a> {
    method: &'a Method,
    field: &'a Field,
    struct_attributes: &'a StructAttributes,
}

impl<'a> Query<'a> {
    pub(crate) fn field_method_attribute(&self) -> Option<&'a FieldMethodAttributes> {
        self.field
            .attributes
            .method_attributes
            .iter()
            .find(|x| x.method == *self.method)
    }

    pub(crate) fn struct_method_attribute(&self) -> Option<&'a StructMethodAttributes> {
        self.struct_attributes
            .methods
            .iter()
            .find(|x| x.method == *self.method)
    }
    pub(crate) fn is_get_copy(&self) -> bool {
        self.field_method_attribute()
            .and_then(|fva| fva.get_copy)
            .unwrap_or_default()
    }

    pub(crate) fn chainable_set(&self) -> bool {
        self.method == &Set
            && self
                .field_method_attribute()
                .and_then(|fva| fva.chainable_set)
                .or(self
                    .struct_method_attribute()
                    .and_then(|sva| sva.chainable_set))
                .unwrap_or(DEFAULT_CHAINABLE_SET)
    }

    pub(crate) fn vis(&self) -> Cow<'a, Visibility> {
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

    pub(crate) fn fn_ident(&self) -> Cow<'a, Ident> {
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
            Get => Cow::Borrowed(ident),
            Set => Cow::Owned(Ident::new(&format!("set_{ident}"), self.field.ident.span())),
            With => Cow::Owned(Ident::new(
                &format!("with_{ident}"),
                self.field.ident.span(),
            )),
            GetMut => Cow::Owned(Ident::new(&format!("{ident}_mut"), self.field.ident.span())),
        }
    }

    pub(crate) fn variable_ident(&self) -> Cow<'a, Ident> {
        Cow::Borrowed(&self.field.ident)
    }

    pub(crate) fn argument_ident(&self) -> Cow<'a, Ident> {
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

    pub(crate) fn doc_template(&self) -> &str {
        match self.method {
            Get if self.is_get_copy() => "Returns a copy of {}",
            Get => "Borrows {}",
            Set if self.chainable_set() => "Sets {}, returning `&mut Self` for chaining",
            Set => "Sets {}",
            With => "Owned chainable setter for {}, returning `Self`",
            GetMut => "Mutably borrow {}",
        }
    }

    pub(crate) fn docs(&self) -> Option<Cow<'a, str>> {
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

    pub(crate) fn enabled(&self) -> bool {
        let struct_method_attr = self.struct_method_attribute();
        let field_method_attr = self.field_method_attribute();
        let StructAttributes {
            include, opt_in, ..
        } = self.struct_attributes;
        let FieldAttributes {
            decorated, skip, ..
        } = self.field.attributes;

        if *opt_in {
            decorated
                && ((self.field.attributes.method_attributes.is_empty()
                    && include.as_ref().is_some_and(|x| x.contains(self.method)))
                    || field_method_attr.is_some_and(|x| !x.skip))
        } else if include.as_ref().is_some_and(|x| !x.contains(self.method)) {
            field_method_attr.is_some_and(|x| !x.skip)
        } else {
            field_method_attr.is_none_or(|x| !x.skip)
                && struct_method_attr.is_none_or(|x| !x.skip)
                && !skip
        }
    }

    pub(crate) fn deref_type(&self) -> &'a Type {
        self.field_method_attribute()
            .and_then(|x| x.deref.as_ref())
            .unwrap_or(
                self.field
                    .attributes
                    .deref
                    .as_ref()
                    .unwrap_or(&self.field.ty),
            )
    }
}

pub(crate) fn resolve<'a>(
    method: &'a Method,
    field: &'a Field,
    struct_attributes: &'a StructAttributes,
) -> Option<Resolved<'a>> {
    let query = Query {
        method,
        field,
        struct_attributes,
    };
    if !query.enabled() {
        return None;
    }
    let vis = query.vis();
    let fn_ident = query.fn_ident();
    let variable_ident = query.variable_ident();
    let argument_ident = query.argument_ident();
    let doc = query.docs();
    let deref_type = query.deref_type();
    let ty = &field.ty;
    let chainable_set = query.chainable_set();
    let get_copy = query.is_get_copy();

    Some(Resolved {
        vis,
        fn_ident,
        variable_ident,
        argument_ident,
        ty,
        doc,
        get_copy,
        chainable_set,
        deref_type,
    })
}

#[derive(Debug)]
pub(crate) struct Resolved<'a> {
    pub(crate) vis: Cow<'a, Visibility>,
    pub(crate) fn_ident: Cow<'a, Ident>,
    pub(crate) variable_ident: Cow<'a, Ident>,
    pub(crate) argument_ident: Cow<'a, Ident>,
    pub(crate) ty: &'a Type,
    pub(crate) doc: Option<Cow<'a, str>>,
    pub(crate) get_copy: bool,
    pub(crate) chainable_set: bool,
    pub(crate) deref_type: &'a Type,
}
