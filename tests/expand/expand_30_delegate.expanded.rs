use std::borrow::Cow;
/// Its methods are also emitted on each type that delegates to it
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
impl Inner {
    ///Borrows the name
    #[inline]
    pub fn name(&self) -> &str {
        &*self.name
    }
    ///Sets the name, returning `&mut Self` for chaining
    #[inline]
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    ///Returns a copy of the count
    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }
    ///Sets the count, returning `&mut Self` for chaining
    #[inline]
    pub fn set_count(&mut self, count: usize) -> &mut Self {
        self.count = count;
        self
    }
    ///Borrows the label
    #[inline]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
    ///Sets the label, returning `&mut Self` for chaining
    #[inline]
    pub fn set_label(
        &mut self,
        label: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.label = label.map(Into::into);
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.label = Some(label.into());
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn without_label(mut self) -> Self {
        self.label = None;
        self
    }
    ///Takes the label, leaving a None in its place
    #[inline]
    pub fn take_label(&mut self) -> Option<Cow<'static, str>> {
        self.label.take()
    }
    ///Borrows the items
    #[inline]
    pub fn items(&self) -> &[u8] {
        &*self.items
    }
    ///Mutably borrow the items
    #[inline]
    pub fn items_mut(&mut self) -> &mut [u8] {
        &mut *self.items
    }
    ///Sets the items, returning `&mut Self` for chaining
    #[inline]
    pub fn set_items(&mut self, items: Vec<u8>) -> &mut Self {
        self.items = items;
        self
    }
    ///Consumes self, returning the items
    #[inline]
    pub fn into_items(self) -> Vec<u8> {
        self.items
    }
    ///Returns a copy of whether it's enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    ///Sets whether it's enabled, returning `&mut Self` for chaining
    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
}
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use __fieldwork_delegate_Inner as Inner;
/// Delegates through a plain field
struct Outer {
    #[field(delegate)]
    inner: Inner,
}
impl Outer {}
impl Outer {
    ///Borrows the name
    #[inline]
    pub fn name(&self) -> &str {
        &*self.inner.name
    }
    ///Sets the name, returning `&mut Self` for chaining
    #[inline]
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.inner.name = name;
        self
    }
    ///Returns a copy of the count
    #[inline]
    pub fn count(&self) -> usize {
        self.inner.count
    }
    ///Sets the count, returning `&mut Self` for chaining
    #[inline]
    pub fn set_count(&mut self, count: usize) -> &mut Self {
        self.inner.count = count;
        self
    }
    ///Borrows the label
    #[inline]
    pub fn label(&self) -> Option<&str> {
        self.inner.label.as_deref()
    }
    ///Sets the label, returning `&mut Self` for chaining
    #[inline]
    pub fn set_label(
        &mut self,
        label: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.inner.label = label.map(Into::into);
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.inner.label = Some(label.into());
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn without_label(mut self) -> Self {
        self.inner.label = None;
        self
    }
    ///Takes the label, leaving a None in its place
    #[inline]
    pub fn take_label(&mut self) -> Option<Cow<'static, str>> {
        self.inner.label.take()
    }
    ///Borrows the items
    #[inline]
    pub fn items(&self) -> &[u8] {
        &*self.inner.items
    }
    ///Mutably borrow the items
    #[inline]
    pub fn items_mut(&mut self) -> &mut [u8] {
        &mut *self.inner.items
    }
    ///Sets the items, returning `&mut Self` for chaining
    #[inline]
    pub fn set_items(&mut self, items: Vec<u8>) -> &mut Self {
        self.inner.items = items;
        self
    }
    ///Consumes self, returning the items
    #[inline]
    pub fn into_items(self) -> Vec<u8> {
        self.inner.items
    }
    ///Returns a copy of whether it's enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.inner.enabled
    }
    ///Sets whether it's enabled, returning `&mut Self` for chaining
    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.inner.enabled = enabled;
        self
    }
}
/// Delegates through a box. The delegate field still gets the item-level methods.
#[fieldwork(get)]
struct Boxed {
    /// the id
    id: u64,
    #[field(delegate)]
    inner: Box<Inner>,
}
impl Boxed {
    ///Returns a copy of the id
    #[inline]
    pub fn id(&self) -> u64 {
        self.id
    }
    #[inline]
    pub fn inner(&self) -> &Inner {
        &*self.inner
    }
}
impl Boxed {
    ///Borrows the name
    #[inline]
    pub fn name(&self) -> &str {
        &*self.inner.name
    }
    ///Sets the name, returning `&mut Self` for chaining
    #[inline]
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.inner.name = name;
        self
    }
    ///Returns a copy of the count
    #[inline]
    pub fn count(&self) -> usize {
        self.inner.count
    }
    ///Sets the count, returning `&mut Self` for chaining
    #[inline]
    pub fn set_count(&mut self, count: usize) -> &mut Self {
        self.inner.count = count;
        self
    }
    ///Borrows the label
    #[inline]
    pub fn label(&self) -> Option<&str> {
        self.inner.label.as_deref()
    }
    ///Sets the label, returning `&mut Self` for chaining
    #[inline]
    pub fn set_label(
        &mut self,
        label: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.inner.label = label.map(Into::into);
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.inner.label = Some(label.into());
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn without_label(mut self) -> Self {
        self.inner.label = None;
        self
    }
    ///Takes the label, leaving a None in its place
    #[inline]
    pub fn take_label(&mut self) -> Option<Cow<'static, str>> {
        self.inner.label.take()
    }
    ///Borrows the items
    #[inline]
    pub fn items(&self) -> &[u8] {
        &*self.inner.items
    }
    ///Mutably borrow the items
    #[inline]
    pub fn items_mut(&mut self) -> &mut [u8] {
        &mut *self.inner.items
    }
    ///Sets the items, returning `&mut Self` for chaining
    #[inline]
    pub fn set_items(&mut self, items: Vec<u8>) -> &mut Self {
        self.inner.items = items;
        self
    }
    ///Consumes self, returning the items
    #[inline]
    pub fn into_items(self) -> Vec<u8> {
        self.inner.items
    }
    ///Returns a copy of whether it's enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.inner.enabled
    }
    ///Sets whether it's enabled, returning `&mut Self` for chaining
    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.inner.enabled = enabled;
        self
    }
}
/// Delegates through a newtype
struct Newtype(#[field(delegate)] Inner);
impl Newtype {}
impl Newtype {
    ///Borrows the name
    #[inline]
    pub fn name(&self) -> &str {
        &*self.0.name
    }
    ///Sets the name, returning `&mut Self` for chaining
    #[inline]
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.0.name = name;
        self
    }
    ///Returns a copy of the count
    #[inline]
    pub fn count(&self) -> usize {
        self.0.count
    }
    ///Sets the count, returning `&mut Self` for chaining
    #[inline]
    pub fn set_count(&mut self, count: usize) -> &mut Self {
        self.0.count = count;
        self
    }
    ///Borrows the label
    #[inline]
    pub fn label(&self) -> Option<&str> {
        self.0.label.as_deref()
    }
    ///Sets the label, returning `&mut Self` for chaining
    #[inline]
    pub fn set_label(
        &mut self,
        label: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.0.label = label.map(Into::into);
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.0.label = Some(label.into());
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn without_label(mut self) -> Self {
        self.0.label = None;
        self
    }
    ///Takes the label, leaving a None in its place
    #[inline]
    pub fn take_label(&mut self) -> Option<Cow<'static, str>> {
        self.0.label.take()
    }
    ///Borrows the items
    #[inline]
    pub fn items(&self) -> &[u8] {
        &*self.0.items
    }
    ///Mutably borrow the items
    #[inline]
    pub fn items_mut(&mut self) -> &mut [u8] {
        &mut *self.0.items
    }
    ///Sets the items, returning `&mut Self` for chaining
    #[inline]
    pub fn set_items(&mut self, items: Vec<u8>) -> &mut Self {
        self.0.items = items;
        self
    }
    ///Consumes self, returning the items
    #[inline]
    pub fn into_items(self) -> Vec<u8> {
        self.0.items
    }
    ///Returns a copy of whether it's enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.0.enabled
    }
    ///Sets whether it's enabled, returning `&mut Self` for chaining
    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.0.enabled = enabled;
        self
    }
}
/// A type alias hides the delegate, so it's named explicitly
type BoxedInner = Box<Inner>;
struct Aliased {
    #[field(delegate = Inner)]
    inner: BoxedInner,
}
impl Aliased {}
impl Aliased {
    ///Borrows the name
    #[inline]
    pub fn name(&self) -> &str {
        &*self.inner.name
    }
    ///Sets the name, returning `&mut Self` for chaining
    #[inline]
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.inner.name = name;
        self
    }
    ///Returns a copy of the count
    #[inline]
    pub fn count(&self) -> usize {
        self.inner.count
    }
    ///Sets the count, returning `&mut Self` for chaining
    #[inline]
    pub fn set_count(&mut self, count: usize) -> &mut Self {
        self.inner.count = count;
        self
    }
    ///Borrows the label
    #[inline]
    pub fn label(&self) -> Option<&str> {
        self.inner.label.as_deref()
    }
    ///Sets the label, returning `&mut Self` for chaining
    #[inline]
    pub fn set_label(
        &mut self,
        label: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.inner.label = label.map(Into::into);
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.inner.label = Some(label.into());
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn without_label(mut self) -> Self {
        self.inner.label = None;
        self
    }
    ///Takes the label, leaving a None in its place
    #[inline]
    pub fn take_label(&mut self) -> Option<Cow<'static, str>> {
        self.inner.label.take()
    }
    ///Borrows the items
    #[inline]
    pub fn items(&self) -> &[u8] {
        &*self.inner.items
    }
    ///Mutably borrow the items
    #[inline]
    pub fn items_mut(&mut self) -> &mut [u8] {
        &mut *self.inner.items
    }
    ///Sets the items, returning `&mut Self` for chaining
    #[inline]
    pub fn set_items(&mut self, items: Vec<u8>) -> &mut Self {
        self.inner.items = items;
        self
    }
    ///Consumes self, returning the items
    #[inline]
    pub fn into_items(self) -> Vec<u8> {
        self.inner.items
    }
    ///Returns a copy of whether it's enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.inner.enabled
    }
    ///Sets whether it's enabled, returning `&mut Self` for chaining
    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.inner.enabled = enabled;
        self
    }
}
/// Delegation is additive: in opt-in mode, `delegate` opts the field into its own methods like any
/// other annotation, and `skip` leaves only the delegated ones
#[fieldwork(opt_in, get)]
struct OptIn {
    #[field(delegate)]
    inner: Inner,
    #[field(delegate, skip)]
    later: Later,
}
impl OptIn {
    #[inline]
    pub fn inner(&self) -> &Inner {
        &self.inner
    }
}
impl OptIn {
    ///Borrows the name
    #[inline]
    pub fn name(&self) -> &str {
        &*self.inner.name
    }
    ///Sets the name, returning `&mut Self` for chaining
    #[inline]
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.inner.name = name;
        self
    }
    ///Returns a copy of the count
    #[inline]
    pub fn count(&self) -> usize {
        self.inner.count
    }
    ///Sets the count, returning `&mut Self` for chaining
    #[inline]
    pub fn set_count(&mut self, count: usize) -> &mut Self {
        self.inner.count = count;
        self
    }
    ///Borrows the label
    #[inline]
    pub fn label(&self) -> Option<&str> {
        self.inner.label.as_deref()
    }
    ///Sets the label, returning `&mut Self` for chaining
    #[inline]
    pub fn set_label(
        &mut self,
        label: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.inner.label = label.map(Into::into);
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.inner.label = Some(label.into());
        self
    }
    ///Owned chainable setter for the label, returning `Self`
    #[inline]
    #[must_use]
    pub fn without_label(mut self) -> Self {
        self.inner.label = None;
        self
    }
    ///Takes the label, leaving a None in its place
    #[inline]
    pub fn take_label(&mut self) -> Option<Cow<'static, str>> {
        self.inner.label.take()
    }
    ///Borrows the items
    #[inline]
    pub fn items(&self) -> &[u8] {
        &*self.inner.items
    }
    ///Mutably borrow the items
    #[inline]
    pub fn items_mut(&mut self) -> &mut [u8] {
        &mut *self.inner.items
    }
    ///Sets the items, returning `&mut Self` for chaining
    #[inline]
    pub fn set_items(&mut self, items: Vec<u8>) -> &mut Self {
        self.inner.items = items;
        self
    }
    ///Consumes self, returning the items
    #[inline]
    pub fn into_items(self) -> Vec<u8> {
        self.inner.items
    }
    ///Returns a copy of whether it's enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.inner.enabled
    }
    ///Sets whether it's enabled, returning `&mut Self` for chaining
    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.inner.enabled = enabled;
        self
    }
}
impl OptIn {
    ///Returns a copy of the flag
    #[inline]
    pub fn flag(&self) -> bool {
        self.later.flag
    }
}
/// Generic parameter names line up between the two structs
#[fieldwork(get, delegatable)]
struct GenericInner<T> {
    /// the value
    value: T,
}
impl<T> GenericInner<T> {
    ///Borrows the value
    #[inline]
    pub fn value(&self) -> &T {
        &self.value
    }
}
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use __fieldwork_delegate_GenericInner as GenericInner;
/// The delegated impl uses this struct's generics and where clause, including `bounds`
#[fieldwork(bounds = "T: Clone")]
struct GenericOuter<'a, T: std::fmt::Debug>
where
    T: Default,
{
    #[field(delegate)]
    inner: GenericInner<T>,
    other: &'a (),
}
impl<'a, T: std::fmt::Debug> GenericOuter<'a, T>
where
    T: Default,
    T: Clone,
{}
impl<'a, T: std::fmt::Debug> GenericOuter<'a, T>
where
    T: Default,
    T: Clone,
{
    ///Borrows the value
    #[inline]
    pub fn value(&self) -> &T {
        &self.inner.value
    }
}
/// Declared before the struct it delegates to
struct Early {
    #[field(delegate)]
    later: Later,
}
impl Early {}
impl Early {
    ///Returns a copy of the flag
    #[inline]
    pub fn flag(&self) -> bool {
        self.later.flag
    }
}
#[fieldwork(get, delegatable)]
struct Later {
    /// the flag
    flag: bool,
}
impl Later {
    ///Returns a copy of the flag
    #[inline]
    pub fn flag(&self) -> bool {
        self.flag
    }
}
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use __fieldwork_delegate_Later as Later;
mod child {
    /// Delegates from another module. The delegated methods expand here, so `Later`'s fields
    /// (private, but visible to descendant modules) and field types must be reachable from here.
    pub(super) struct CrossModule {
        #[field(delegate)]
        pub(super) later: super::Later,
    }
    impl CrossModule {}
    impl CrossModule {
        ///Returns a copy of the flag
        #[inline]
        pub fn flag(&self) -> bool {
            self.later.flag
        }
    }
}
