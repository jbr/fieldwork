use std::borrow::Cow;

/// Its methods are also emitted on each type that delegates to it
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, delegatable)]
struct Inner {
    /// the name
    name: String,
    /// the count
    count: usize,
    /// the label
    #[field(with, without, take, into)]
    label: Option<Cow<'static, str>>,
    /// the items
    #[field(get_mut, into_field)]
    items: Vec<u8>,
    /// whether it's enabled
    #[field(rename_predicates)]
    enabled: bool,
}

/// Delegates through a plain field
#[derive(fieldwork::Fieldwork)]
struct Outer {
    #[field(delegate)]
    inner: Inner,
}

/// Delegates through a box. The delegate field still gets the item-level methods.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct Boxed {
    /// the id
    id: u64,
    #[field(delegate)]
    inner: Box<Inner>,
}

/// Delegates through a newtype
#[derive(fieldwork::Fieldwork)]
struct Newtype(#[field(delegate)] Inner);

/// A type alias hides the delegate, so it's named explicitly
type BoxedInner = Box<Inner>;

#[derive(fieldwork::Fieldwork)]
struct Aliased {
    #[field(delegate = Inner)]
    inner: BoxedInner,
}

/// Delegation is additive: in opt-in mode, `delegate` opts the field into its own methods like any
/// other annotation, and `skip` leaves only the delegated ones
#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get)]
struct OptIn {
    #[field(delegate)]
    inner: Inner,
    #[field(delegate, skip)]
    later: Later,
}

/// Generic parameter names line up between the two structs
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, delegatable)]
struct GenericInner<T> {
    /// the value
    value: T,
}

/// The delegated impl uses this struct's generics and where clause, including `bounds`
#[derive(fieldwork::Fieldwork)]
#[fieldwork(bounds = "T: Clone")]
struct GenericOuter<'a, T: std::fmt::Debug>
where
    T: Default,
{
    #[field(delegate)]
    inner: GenericInner<T>,
    other: &'a (),
}

/// Declared before the struct it delegates to
#[derive(fieldwork::Fieldwork)]
struct Early {
    #[field(delegate)]
    later: Later,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, delegatable)]
struct Later {
    /// the flag
    flag: bool,
}

mod child {
    /// Delegates from another module. The delegated methods expand here, so `Later`'s fields
    /// (private, but visible to descendant modules) and field types must be reachable from here.
    #[derive(fieldwork::Fieldwork)]
    pub(super) struct CrossModule {
        #[field(delegate)]
        pub(super) later: super::Later,
    }
}
