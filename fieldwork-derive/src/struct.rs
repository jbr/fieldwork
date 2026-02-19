use syn::{
    Data, DeriveInput, Error, Generics, Ident,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use crate::{Field, StructAttributes};

// this represents the struct that we called derive on
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Struct {
    pub(crate) ident: Ident,
    pub(crate) fields: Vec<Field>,
    pub(crate) attributes: StructAttributes,
    pub(crate) generics: Generics,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input = DeriveInput::parse(input)?;
        let Data::Struct(ds) = &input.data else {
            return Err(Error::new(
                input.span(),
                "fieldwork currently only works on named structs",
            ));
        };
        let ident = input.ident;
        let mut attributes = StructAttributes::build(&input.attrs)?;
        let fields = ds
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| Field::build(field, index))
            .collect::<syn::Result<Vec<_>>>()?;

        let mut generics = input.generics.clone();
        generics.where_clause = attributes.where_clause.take();

        Ok(Self {
            ident,
            fields,
            attributes,
            generics,
        })
    }
}
