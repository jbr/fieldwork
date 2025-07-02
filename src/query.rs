use std::borrow::Cow;

use crate::{
    CommonSettings, Field, FieldAttributes, FieldMethodAttributes, Method, Resolved,
    StructAttributes, StructMethodAttributes,
    copy_detection::{enable_copy_for_type, is_type},
    deref_handling::auto_deref,
    option_handling::{extract_option_type, strip_ref},
};
use Method::{Get, GetMut, Set, With, Without};
use syn::{Expr, Ident, Member, Type, Visibility, parse_quote};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Query<'a> {
    method: &'a Method,
    field: &'a Field,
    struct_attributes: &'a StructAttributes,
    field_method_attributes: Option<&'a FieldMethodAttributes>,
    struct_method_attributes: Option<&'a StructMethodAttributes>,
}

impl<'a> Query<'a> {
    pub(crate) fn new(
        method: &'a Method,
        field: &'a Field,
        struct_attributes: &'a StructAttributes,
    ) -> Self {
        let field_method_attributes = field.attributes.method_attributes.retrieve(*method);
        let struct_method_attributes = struct_attributes.methods.retrieve(*method);

        Self {
            method,
            field,
            struct_attributes,
            field_method_attributes,
            struct_method_attributes,
        }
    }

    pub(crate) fn resolve(&self) -> Option<Resolved<'a>> {
        if !self.enabled() {
            return None;
        }
        let method = *self.method;
        let vis = self.vis();
        let fn_ident = self.fn_ident()?;
        let variable_ident = self.variable_ident();
        let argument_ident = self.argument_ident()?;
        let doc = self.docs();
        let deref_type = self.deref_type();
        let ty = &self.field.ty;
        let chainable_set = self.chainable_set();
        let get_copy = self.is_get_copy();
        let option_borrow_inner = self.option_borrow_inner();
        let (argument_ty, assigned_value) =
            self.determine_argument_ty_and_assigned_value(&argument_ident)?;

        let argument_ident_and_ty = argument_ty.map(|ty| (argument_ident, ty));

        Some(Resolved {
            method,
            vis,
            fn_ident,
            variable_ident,
            argument_ident_and_ty,
            ty,
            doc,
            get_copy,
            chainable_set,
            deref_type,
            option_borrow_inner,
            assigned_value,
        })
    }

    fn field_method_attribute(&self) -> Option<&'a FieldMethodAttributes> {
        self.field_method_attributes
    }

    fn struct_method_attribute(&self) -> Option<&'a StructMethodAttributes> {
        self.struct_method_attributes
    }

    fn is_get_copy(&self) -> bool {
        if let Some(field_copy) = self
            .field_method_attributes
            .and_then(|fma| fma.common_settings.get_copy)
        {
            return field_copy;
        }

        self.common_setting(|x| x.get_copy) && enable_copy_for_type(&self.field.ty)
    }

    fn chainable_set(&self) -> bool {
        self.method == &Set && self.common_setting(|x| x.chainable_set)
    }

    fn vis(&self) -> Cow<'a, Visibility> {
        self.common_setting(|x| x.vis.as_ref()).as_visibility()
    }

    fn rename_predicates(&self) -> bool {
        self.common_setting(|x| x.rename_predicates)
    }

    fn fn_ident(&self) -> Option<Cow<'a, Ident>> {
        if let Some(fn_ident) = self
            .field_method_attribute()
            .and_then(|x| x.fn_ident.as_ref())
        {
            return Some(Cow::Borrowed(fn_ident));
        }

        let ident = self
            .field
            .attributes
            .fn_ident
            .as_ref()
            .or(match &self.field.member {
                Member::Named(ident) => Some(ident),
                Member::Unnamed(_) => None,
            })?;

        if let Some(template) = self
            .struct_method_attribute()
            .and_then(|x| x.template.as_ref())
        {
            return Some(Cow::Owned(Ident::new(
                &template.as_str().replacen("{}", &ident.to_string(), 1),
                self.field.span,
            )));
        }

        Some(match self.method {
            Get if self.rename_predicates() && is_type(&self.field.ty, "bool") => {
                Cow::Owned(Ident::new(&format!("is_{ident}"), self.field.span))
            }
            Get => Cow::Borrowed(ident),
            Set => Cow::Owned(Ident::new(&format!("set_{ident}"), self.field.span)),
            With => Cow::Owned(Ident::new(&format!("with_{ident}"), self.field.span)),
            GetMut => Cow::Owned(Ident::new(&format!("{ident}_mut"), self.field.span)),
            Without => Cow::Owned(Ident::new(&format!("without_{ident}"), self.field.span)),
        })
    }

    fn variable_ident(&self) -> &'a Member {
        &self.field.member
    }

    fn argument_ident(&self) -> Option<Cow<'a, Ident>> {
        if let Some(argument_ident) = self
            .field_method_attributes
            .and_then(|x| x.argument_ident.as_ref())
        {
            return Some(Cow::Borrowed(argument_ident));
        }

        if let Some(argument_ident) = self.field.attributes.argument_ident.as_ref() {
            return Some(Cow::Borrowed(argument_ident));
        }

        if let Some(renamed) = self.field.attributes.fn_ident.as_ref() {
            return Some(Cow::Borrowed(renamed));
        }

        if let Member::Named(ident) = &self.field.member {
            return Some(Cow::Borrowed(ident));
        }

        if let Some(argument_ident) = self
            .field_method_attributes
            .and_then(|x| x.fn_ident.as_ref())
        {
            return Some(Cow::Borrowed(argument_ident));
        }

        None
    }

    fn doc_template(&self) -> &str {
        match self.method {
            Get if self.is_get_copy() => "Returns a copy of {}",
            Get => "Borrows {}",
            Set if self.chainable_set() => "Sets {}, returning `&mut Self` for chaining",
            Set => "Sets {}",
            With | Without => "Owned chainable setter for {}, returning `Self`",
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
            include,
            common_settings: CommonSettings { opt_in, .. },
            ..
        } = self.struct_attributes;
        let FieldAttributes {
            decorated,
            common_settings:
                CommonSettings {
                    opt_in: field_opt_in,
                    skip,
                    ..
                },
            ..
        } = self.field.attributes;

        if *opt_in || field_opt_in {
            decorated
                && ((self.field.attributes.method_attributes.is_empty()
                    && include.contains(*self.method))
                    || field_method_attr.is_some_and(|x| !x.common_settings.skip))
        } else if !include.contains(*self.method) {
            field_method_attr.is_some_and(|x| !x.common_settings.skip)
        } else {
            field_method_attr.is_none_or(|x| !x.common_settings.skip)
                && struct_method_attr.is_none_or(|x| !x.common_settings.skip)
                && !skip
        }
    }

    fn auto_deref(&self, ty: &'a Type) -> Option<Cow<'a, Type>> {
        if self.common_setting(|x| x.auto_deref) {
            auto_deref(ty)
        } else {
            None
        }
    }

    fn deref_type(&self) -> Option<Cow<'a, Type>> {
        self.field_method_attribute()
            .and_then(|x| x.deref.as_ref())
            .or(self.field.attributes.deref.as_ref())
            .map(strip_ref)
            .map(Cow::Borrowed)
            .or_else(|| self.auto_deref(&self.field.ty))
    }

    fn common_setting<T: 'a>(&self, fun: impl Fn(&'a CommonSettings) -> Option<T>) -> T {
        self.common_setting_without_default(&fun)
            .unwrap_or_else(|| fun(CommonSettings::DEFAULTS).unwrap())
    }

    fn common_setting_without_default<T: 'a>(
        &self,
        fun: impl Fn(&'a CommonSettings) -> Option<T>,
    ) -> Option<T> {
        [
            self.field_method_attributes.map(|x| &x.common_settings),
            Some(&self.field.attributes.common_settings),
            self.struct_method_attributes.map(|x| &x.common_settings),
            Some(&self.struct_attributes.common_settings),
        ]
        .into_iter()
        .flatten()
        .find_map(fun)
    }

    fn option_borrow_inner(&self) -> Option<OptionHandling<'a>> {
        if !self.common_setting(|x| x.option_borrow_inner) {
            return None;
        }

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

    fn determine_argument_ty_and_assigned_value(
        &self,
        argument_ident: &Ident,
    ) -> Option<(Option<Cow<'a, Type>>, Expr)> {
        if self.method == &Without {
            if is_type(&self.field.ty, "bool") {
                return Some((None, parse_quote!(false)));
            }

            if extract_option_type(&self.field.ty).is_some() {
                return Some((None, parse_quote!(None)));
            }

            return None;
        }

        let with_without_pair = self.method == &With && {
            Query::new(&Without, &self.field, &self.struct_attributes).enabled()
        };

        let mut option_set_some = self
            .common_setting_without_default(|x| x.option_set_some)
            .unwrap_or(with_without_pair);

        let into = self.common_setting(|x| x.into);

        if with_without_pair && option_set_some && is_type(&self.field.ty, "bool") {
            return Some((None, parse_quote!(true)));
        }

        let mut argument_ty = Cow::Borrowed(&self.field.ty);

        if option_set_some {
            if let Some(ty) = extract_option_type(&self.field.ty) {
                argument_ty = Cow::Borrowed(ty);
            } else {
                option_set_some = false;
            }
        }

        let mut assigned_value = parse_quote!(#argument_ident);

        if into {
            argument_ty = Cow::Owned(parse_quote!(impl Into<#argument_ty>));
            assigned_value = parse_quote!(#assigned_value.into());
        }

        if option_set_some {
            assigned_value = parse_quote!(Some(#assigned_value));
        }

        Some((Some(argument_ty), assigned_value))
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) enum OptionHandling<'a> {
    Ref(Cow<'a, Type>),
    Deref(Cow<'a, Type>),
}
