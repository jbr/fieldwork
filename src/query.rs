use std::borrow::Cow;

use crate::{
    CommonSettings, Field, FieldAttributes, FieldMethodAttributes, Method, Resolved,
    StructAttributes, StructMethodAttributes,
    copy_detection::{enable_copy_for_type, is_type},
    deref_handling::auto_deref,
    option_handling::{extract_option_type, ref_inner, strip_ref},
};
use Method::{Get, GetMut, Set, With, Without};
use proc_macro2::Span;
use syn::{Expr, Ident, Member, Type, Visibility, parse_quote_spanned};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Query<'a> {
    field: &'a Field,
    field_method_attributes: Option<&'a FieldMethodAttributes>,
    method: &'a Method,
    span: &'a Span,
    struct_attributes: &'a StructAttributes,
    struct_method_attributes: Option<&'a StructMethodAttributes>,
}

impl<'a> Query<'a> {
    pub(crate) fn method(&self) -> Method {
        *self.method
    }

    pub(crate) fn span(&self) -> Span {
        *self.span
    }

    pub(crate) fn new(
        method: &'a Method,
        field: &'a Field,
        struct_attributes: &'a StructAttributes,
    ) -> Self {
        let (span, field_method_attributes) = field
            .attributes
            .method_attributes
            .retrieve(*method)
            .map_or((None, None), |(s, fma)| (Some(s), Some(fma)));
        let struct_method_attributes = struct_attributes.methods.retrieve(*method);
        let span = span.unwrap_or(&field.span);

        Self {
            field,
            field_method_attributes,
            method,
            span,
            struct_attributes,
            struct_method_attributes,
        }
    }

    pub(crate) fn ty(&self) -> &'a Type {
        &self.field.ty
    }

    pub(crate) fn resolve(&self) -> Option<Resolved<'a>> {
        Resolved::from_query(self)
    }

    pub(crate) fn field_method_attribute(&self) -> Option<&'a FieldMethodAttributes> {
        self.field_method_attributes
    }

    pub(crate) fn struct_method_attribute(&self) -> Option<&'a StructMethodAttributes> {
        self.struct_method_attributes
    }

    pub(crate) fn is_get_copy(&self, ty: &Type) -> bool {
        if let Some(field_copy) = self
            .field_method_attributes
            .and_then(|fma| fma.common_settings.get_copy)
            .or(self.field.attributes.common_settings.get_copy)
        {
            return field_copy;
        }

        self.common_setting(|x| x.get_copy)
            && (enable_copy_for_type(ty, *self.method)
                || self
                    .borrow_inner(ty)
                    .is_some_and(|ty| enable_copy_for_type(ty, *self.method)))
    }

    pub(crate) fn chainable_set(&self) -> bool {
        self.method == &Set && self.common_setting(|x| x.chainable_set)
    }

    pub(crate) fn vis(&self) -> Cow<'a, Visibility> {
        self.common_setting(|x| x.vis.as_ref()).as_visibility()
    }

    pub(crate) fn rename_predicates(&self) -> bool {
        self.common_setting(|x| x.rename_predicates)
    }

    pub(crate) fn field_name(&self) -> Option<&'a Ident> {
        self.field_method_attributes
            .and_then(|x| x.fn_ident.as_ref())
            .or(self.field.attributes.fn_ident.as_ref())
            .or(match &self.field.member {
                Member::Named(ident) => Some(ident),
                Member::Unnamed(_) => None,
            })
    }

    pub(crate) fn fn_ident(&self) -> Option<Cow<'a, Ident>> {
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

    pub(crate) fn variable_ident(&self) -> &'a Member {
        &self.field.member
    }

    pub(crate) fn argument_ident(&self) -> Option<Cow<'a, Ident>> {
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

    pub(crate) fn doc_template(&self, is_get_copy: bool) -> &str {
        match self.method {
            Get if is_get_copy => "Returns a copy of {}",
            Get => "Borrows {}",
            Set if self.chainable_set() => "Sets {}, returning `&mut Self` for chaining",
            Set => "Sets {}",
            With | Without => "Owned chainable setter for {}, returning `Self`",
            GetMut => "Mutably borrow {}",
        }
    }

    pub(crate) fn docs(&self, is_get_copy: bool) -> Option<Cow<'a, str>> {
        if let Some(explicit_method_doc) =
            self.field_method_attribute().and_then(|x| x.doc.as_ref())
        {
            return Some(Cow::Borrowed(explicit_method_doc));
        }

        let first_line = self.field.doc.first()?;

        let template = self
            .struct_method_attribute()
            .and_then(|x| x.doc_template.as_deref())
            .unwrap_or(self.doc_template(is_get_copy));

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

    pub(crate) fn auto_deref(&self, ty: &'a Type) -> Option<(Cow<'a, Type>, usize)> {
        if self.common_setting(|x| x.auto_deref) {
            auto_deref(ty, *self.method, self.span()).map(|(ty, count)| (Cow::Owned(ty), count))
        } else {
            None
        }
    }

    pub(crate) fn explicit_deref_type(&self) -> Option<&'a Type> {
        self.field_method_attribute()
            .and_then(|x| x.deref.as_ref())
            .or(self.field.attributes.deref.as_ref())
            .map(|specified| strip_ref(extract_option_type(specified).unwrap_or(specified)))
    }

    pub(crate) fn deref_and_count(&self, ty: &'a Type) -> Option<(Cow<'a, Type>, usize)> {
        let (mut deref, mut count) = self
            .auto_deref(ty)
            .map_or((None, 0), |(deref, count)| (Some(deref), count));

        if let Some(specified) = self.explicit_deref_type() {
            if deref.as_deref().is_none_or(|deref| deref != specified) {
                deref = Some(Cow::Borrowed(specified));
                count += 1;
            }
        }

        deref.map(|deref| (deref, count))
    }

    pub(crate) fn common_setting<T: 'a>(&self, fun: impl Fn(&'a CommonSettings) -> Option<T>) -> T {
        self.common_setting_without_default(&fun)
            .unwrap_or_else(|| fun(CommonSettings::DEFAULTS).unwrap())
    }

    pub(crate) fn common_setting_without_default<T: 'a>(
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

    pub(crate) fn borrow_inner(&self, ty: &'a Type) -> Option<&'a Type> {
        if self.common_setting(|x| x.option_borrow_inner) {
            extract_option_type(ty)
        } else {
            None
        }
    }

    pub(crate) fn mut_access_expr_and_type(&self) -> (Expr, Type) {
        let member = self.variable_ident();
        let span = self.span();
        let mut access_expr: Expr = parse_quote_spanned!(span => self.#member);
        let mut current_type: Type = self.ty().clone();

        if let Some(inner_type) = self.borrow_inner(&current_type) {
            if let Some((deref_type, deref_count)) = self.deref_and_count(inner_type) {
                access_expr = if deref_count == 1 {
                    parse_quote_spanned!(span => #access_expr.as_deref_mut())
                } else {
                    let ident = self.field_name();
                    let mut deref_expr: Expr = parse_quote_spanned!(span => #ident);
                    for _ in 0..deref_count {
                        deref_expr = parse_quote_spanned!(span => *#deref_expr);
                    }
                    parse_quote_spanned!(span => #access_expr.as_mut().map(|#ident| &mut #deref_expr))
                };

                current_type = parse_quote_spanned!(span => Option<&mut #deref_type>);
            } else if ref_inner(inner_type).is_none() {
                access_expr = parse_quote_spanned!(span => #access_expr.as_mut());
                current_type = parse_quote_spanned!(span => Option<&mut #inner_type>);
            }
            return (access_expr, current_type);
        }

        if let Some((deref_type, deref_count)) = self.deref_and_count(&current_type) {
            for _ in 0..deref_count {
                access_expr = parse_quote_spanned!(span => *#access_expr);
            }

            current_type = deref_type.into_owned();
        }

        (
            parse_quote_spanned!(span => &mut #access_expr),
            parse_quote_spanned!(span => &mut #current_type),
        )
    }

    pub(crate) fn get_access_expr_type_and_copy(&self) -> (Expr, Type, bool) {
        let span = self.span();
        let member = self.variable_ident();
        let mut access_expr: Expr = parse_quote_spanned!(span => self.#member);
        let mut current_type: Type = self.ty().clone();

        if let Some(result) = self.check_copy(&access_expr, &current_type) {
            return result;
        }

        if let Some(inner_type) = self.borrow_inner(&current_type) {
            if let Some((deref_type, deref_count)) = self.deref_and_count(inner_type) {
                access_expr = if deref_count == 1 {
                    parse_quote_spanned!(span => #access_expr.as_deref())
                } else {
                    let ident = self.field_name();
                    let mut deref_expr: Expr = parse_quote_spanned!(span => #ident);
                    for _ in 0..deref_count {
                        deref_expr = parse_quote_spanned!(span => *#deref_expr);
                    }
                    parse_quote_spanned!(span => #access_expr.as_ref().map(|#ident| &#deref_expr))
                };

                current_type = parse_quote_spanned!(span => Option<&#deref_type>);
            } else if ref_inner(inner_type).is_none() {
                access_expr = parse_quote_spanned!(span => #access_expr.as_ref());
                current_type = parse_quote_spanned!(span => Option<&#inner_type>);
            }
        } else if let Some((deref_type, deref_count)) = self.deref_and_count(&current_type) {
            for _ in 0..deref_count {
                access_expr = parse_quote_spanned!(span => *#access_expr);
            }

            current_type = deref_type.into_owned();
        }

        self.check_copy(&access_expr, &current_type)
            .unwrap_or_else(|| {
                (
                    parse_quote_spanned!(span => &#access_expr),
                    parse_quote_spanned!(span => &#current_type),
                    false,
                )
            })
    }

    fn check_copy(&self, expr: &Expr, ty: &Type) -> Option<(Expr, Type, bool)> {
        if self.is_get_copy(ty) {
            let is_not_a_reference = ref_inner(ty).is_none()
                && extract_option_type(ty).is_none_or(|t| ref_inner(t).is_none());
            Some((expr.clone(), ty.clone(), is_not_a_reference))
        } else {
            None
        }
    }

    pub(crate) fn determine_argument_ty_and_assigned_value(
        &self,
        argument_ident: &Ident,
    ) -> Option<(Option<Cow<'a, Type>>, Expr)> {
        let span = self.span();
        if self.method == &Without {
            if is_type(&self.field.ty, "bool") {
                return Some((None, parse_quote_spanned!(span => false)));
            }

            if extract_option_type(&self.field.ty).is_some() {
                return Some((None, parse_quote_spanned!(span => None)));
            }

            return None;
        }

        let with_without_pair = self.method == &With && {
            Query::new(&Without, self.field, self.struct_attributes).enabled()
        };

        let mut option_set_some = self
            .common_setting_without_default(|x| x.option_set_some)
            .unwrap_or(with_without_pair);

        let into = self.common_setting(|x| x.into);

        if with_without_pair && option_set_some && is_type(&self.field.ty, "bool") {
            return Some((None, parse_quote_spanned!(span => true)));
        }

        let mut argument_ty = Cow::Borrowed(&self.field.ty);

        if option_set_some {
            if let Some(ty) = extract_option_type(&self.field.ty) {
                argument_ty = Cow::Borrowed(strip_ref(ty));
            } else {
                option_set_some = false;
            }
        }

        let mut assigned_value = parse_quote_spanned!(span => #argument_ident);

        if into {
            argument_ty = Cow::Owned(parse_quote_spanned!(span => impl Into<#argument_ty>));
            assigned_value = parse_quote_spanned!(span => #assigned_value.into());
        }

        if option_set_some {
            assigned_value = parse_quote_spanned!(span => Some(#assigned_value));
        }

        Some((Some(argument_ty), assigned_value))
    }
}
