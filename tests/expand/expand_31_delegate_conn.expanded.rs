//! Every `#[field(...)]` combination on trillium-http's `Conn`, moved behind a box. The public
//! methods on `Conn` should be unchanged.
use std::{borrow::Cow, net::IpAddr, sync::Arc, time::Instant};
pub struct HttpContext;
pub struct Headers;
pub struct Method;
pub struct Status;
pub struct Version;
pub struct TypeSet;
pub struct Body;
impl Clone for Method {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for Method {}
impl Clone for Status {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for Status {}
impl Clone for Version {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for Version {}
/// An HTTP connection.
pub struct Conn<Transport> {
    #[field(delegate)]
    inner: Box<ConnInner>,
    /// the transport
    #[field(get, get_mut)]
    transport: Transport,
}
impl<Transport> Conn<Transport> {
    ///Borrows the transport
    #[inline]
    pub fn transport(&self) -> &Transport {
        &self.transport
    }
    ///Mutably borrow the transport
    #[inline]
    pub fn transport_mut(&mut self) -> &mut Transport {
        &mut self.transport
    }
}
impl<Transport> Conn<Transport> {
    ///Borrows the shared [`HttpContext`]
    #[inline]
    pub fn context(&self) -> &HttpContext {
        &*self.inner.context
    }
    ///Borrows request [headers](Headers)
    #[inline]
    pub fn request_headers(&self) -> &Headers {
        &self.inner.request_headers
    }
    ///Mutably borrow request [headers](Headers)
    #[inline]
    pub fn request_headers_mut(&mut self) -> &mut Headers {
        &mut self.inner.request_headers
    }
    ///Borrows response [headers](Headers)
    #[inline]
    pub fn response_headers(&self) -> &Headers {
        &self.inner.response_headers
    }
    ///Mutably borrow response [headers](Headers)
    #[inline]
    pub fn response_headers_mut(&mut self) -> &mut Headers {
        &mut self.inner.response_headers
    }
    ///Returns a copy of the http method for this conn's request
    #[inline]
    pub fn method(&self) -> Method {
        self.inner.method
    }
    ///Sets the http method for this conn's request, returning `&mut Self` for chaining
    #[inline]
    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.inner.method = method;
        self
    }
    ///Returns a copy of the http status for this conn, if set
    #[inline]
    pub fn status(&self) -> Option<Status> {
        self.inner.status
    }
    ///Returns a copy of The HTTP protocol version in use on this connection.
    #[inline]
    pub fn http_version(&self) -> Version {
        self.inner.version
    }
    ///Borrows the [state typemap](TypeSet) for this conn
    #[inline]
    pub fn state(&self) -> &TypeSet {
        &self.inner.state
    }
    ///Mutably borrow the [state typemap](TypeSet) for this conn
    #[inline]
    pub fn state_mut(&mut self) -> &mut TypeSet {
        &mut self.inner.state
    }
    ///Borrows the response [body](Body)
    #[inline]
    pub fn response_body(&self) -> Option<&Body> {
        self.inner.response_body.as_ref()
    }
    ///Sets the response [body](Body), returning `&mut Self` for chaining
    #[inline]
    pub fn set_response_body(&mut self, response_body: impl Into<Body>) -> &mut Self {
        self.inner.response_body = Some(response_body.into());
        self
    }
    ///Owned chainable setter for the response [body](Body), returning `Self`
    #[inline]
    #[must_use]
    pub fn with_response_body(mut self, response_body: impl Into<Body>) -> Self {
        self.inner.response_body = Some(response_body.into());
        self
    }
    ///Takes the response [body](Body), leaving a None in its place
    #[inline]
    pub fn take_response_body(&mut self) -> Option<Body> {
        self.inner.response_body.take()
    }
    ///Returns a copy of whether the connection is secure
    #[inline]
    pub fn is_secure(&self) -> bool {
        self.inner.secure
    }
    ///Sets whether the connection is secure, returning `&mut Self` for chaining
    #[inline]
    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.inner.secure = secure;
        self
    }
    /**Returns a copy of The [`Instant`] that the first header bytes for this conn were
received, before any processing or parsing has been performed.*/
    #[inline]
    pub fn start_time(&self) -> Instant {
        self.inner.start_time
    }
    ///Returns a copy of The IP Address for the connection, if available
    #[inline]
    pub fn peer_ip(&self) -> Option<IpAddr> {
        self.inner.peer_ip
    }
    ///Sets The IP Address for the connection, if available, returning `&mut Self` for chaining
    #[inline]
    pub fn set_peer_ip(&mut self, peer_ip: Option<impl Into<IpAddr>>) -> &mut Self {
        self.inner.peer_ip = peer_ip.map(Into::into);
        self
    }
    ///Borrows the `:authority` pseudo-header
    #[inline]
    pub fn authority(&self) -> Option<&str> {
        self.inner.authority.as_deref()
    }
    ///Sets the `:authority` pseudo-header, returning `&mut Self` for chaining
    #[inline]
    pub fn set_authority(
        &mut self,
        authority: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.inner.authority = authority.map(Into::into);
        self
    }
    ///Borrows the `:scheme` pseudo-header
    #[inline]
    pub fn scheme(&self) -> Option<&str> {
        self.inner.scheme.as_deref()
    }
    ///Sets the `:scheme` pseudo-header, returning `&mut Self` for chaining
    #[inline]
    pub fn set_scheme(
        &mut self,
        scheme: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.inner.scheme = scheme.map(Into::into);
        self
    }
    ///Borrows the `:protocol` pseudo-header (extended CONNECT)
    #[inline]
    pub fn protocol(&self) -> Option<&str> {
        self.inner.protocol.as_deref()
    }
    ///Sets the `:protocol` pseudo-header (extended CONNECT), returning `&mut Self` for chaining
    #[inline]
    pub fn set_protocol(
        &mut self,
        protocol: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.inner.protocol = protocol.map(Into::into);
        self
    }
    ///Borrows request trailers, populated after the request body has been fully read
    #[inline]
    pub fn request_trailers(&self) -> Option<&Headers> {
        self.inner.request_trailers.as_ref()
    }
    ///Mutably borrow request trailers, populated after the request body has been fully read
    #[inline]
    pub fn request_trailers_mut(&mut self) -> Option<&mut Headers> {
        self.inner.request_trailers.as_mut()
    }
}
#[fieldwork(delegatable)]
pub(crate) struct ConnInner {
    #[field(get)]
    /// the shared [`HttpContext`]
    pub(crate) context: Arc<HttpContext>,
    /// request [headers](Headers)
    #[field(get, get_mut)]
    pub(crate) request_headers: Headers,
    pub(crate) spare_request_headers: Option<Headers>,
    /// response [headers](Headers)
    #[field(get, get_mut)]
    pub(crate) response_headers: Headers,
    pub(crate) path: Cow<'static, str>,
    /// the http method for this conn's request
    #[field(get, set, copy)]
    pub(crate) method: Method,
    /// the http status for this conn, if set
    #[field(get, copy)]
    pub(crate) status: Option<Status>,
    /// The HTTP protocol version in use on this connection.
    #[field(get = http_version, copy)]
    pub(crate) version: Version,
    /// the [state typemap](TypeSet) for this conn
    #[field(get, get_mut)]
    pub(crate) state: TypeSet,
    /// the response [body](Body)
    #[field(get, set, into, option_set_some, take, with)]
    pub(crate) response_body: Option<Body>,
    /// whether the connection is secure
    #[field(get, set, rename_predicates)]
    pub(crate) secure: bool,
    /// The [`Instant`] that the first header bytes for this conn were
    /// received, before any processing or parsing has been performed.
    #[field(get, copy)]
    pub(crate) start_time: Instant,
    /// The IP Address for the connection, if available
    #[field(set, get, copy, into)]
    pub(crate) peer_ip: Option<IpAddr>,
    /// the `:authority` pseudo-header
    #[field(set, get, into)]
    pub(crate) authority: Option<Cow<'static, str>>,
    /// the `:scheme` pseudo-header
    #[field(set, get, into)]
    pub(crate) scheme: Option<Cow<'static, str>>,
    /// the `:protocol` pseudo-header (extended CONNECT)
    #[field(set, get, into)]
    pub(crate) protocol: Option<Cow<'static, str>>,
    /// request trailers, populated after the request body has been fully read
    #[field(get, get_mut)]
    pub(crate) request_trailers: Option<Headers>,
    pub(crate) upgrade: bool,
    #[field = false]
    pub(crate) peer_gone: Option<()>,
}
impl ConnInner {
    ///Borrows the shared [`HttpContext`]
    #[inline]
    pub fn context(&self) -> &HttpContext {
        &*self.context
    }
    ///Borrows request [headers](Headers)
    #[inline]
    pub fn request_headers(&self) -> &Headers {
        &self.request_headers
    }
    ///Mutably borrow request [headers](Headers)
    #[inline]
    pub fn request_headers_mut(&mut self) -> &mut Headers {
        &mut self.request_headers
    }
    ///Borrows response [headers](Headers)
    #[inline]
    pub fn response_headers(&self) -> &Headers {
        &self.response_headers
    }
    ///Mutably borrow response [headers](Headers)
    #[inline]
    pub fn response_headers_mut(&mut self) -> &mut Headers {
        &mut self.response_headers
    }
    ///Returns a copy of the http method for this conn's request
    #[inline]
    pub fn method(&self) -> Method {
        self.method
    }
    ///Sets the http method for this conn's request, returning `&mut Self` for chaining
    #[inline]
    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }
    ///Returns a copy of the http status for this conn, if set
    #[inline]
    pub fn status(&self) -> Option<Status> {
        self.status
    }
    ///Returns a copy of The HTTP protocol version in use on this connection.
    #[inline]
    pub fn http_version(&self) -> Version {
        self.version
    }
    ///Borrows the [state typemap](TypeSet) for this conn
    #[inline]
    pub fn state(&self) -> &TypeSet {
        &self.state
    }
    ///Mutably borrow the [state typemap](TypeSet) for this conn
    #[inline]
    pub fn state_mut(&mut self) -> &mut TypeSet {
        &mut self.state
    }
    ///Borrows the response [body](Body)
    #[inline]
    pub fn response_body(&self) -> Option<&Body> {
        self.response_body.as_ref()
    }
    ///Sets the response [body](Body), returning `&mut Self` for chaining
    #[inline]
    pub fn set_response_body(&mut self, response_body: impl Into<Body>) -> &mut Self {
        self.response_body = Some(response_body.into());
        self
    }
    ///Owned chainable setter for the response [body](Body), returning `Self`
    #[inline]
    #[must_use]
    pub fn with_response_body(mut self, response_body: impl Into<Body>) -> Self {
        self.response_body = Some(response_body.into());
        self
    }
    ///Takes the response [body](Body), leaving a None in its place
    #[inline]
    pub fn take_response_body(&mut self) -> Option<Body> {
        self.response_body.take()
    }
    ///Returns a copy of whether the connection is secure
    #[inline]
    pub fn is_secure(&self) -> bool {
        self.secure
    }
    ///Sets whether the connection is secure, returning `&mut Self` for chaining
    #[inline]
    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.secure = secure;
        self
    }
    /**Returns a copy of The [`Instant`] that the first header bytes for this conn were
received, before any processing or parsing has been performed.*/
    #[inline]
    pub fn start_time(&self) -> Instant {
        self.start_time
    }
    ///Returns a copy of The IP Address for the connection, if available
    #[inline]
    pub fn peer_ip(&self) -> Option<IpAddr> {
        self.peer_ip
    }
    ///Sets The IP Address for the connection, if available, returning `&mut Self` for chaining
    #[inline]
    pub fn set_peer_ip(&mut self, peer_ip: Option<impl Into<IpAddr>>) -> &mut Self {
        self.peer_ip = peer_ip.map(Into::into);
        self
    }
    ///Borrows the `:authority` pseudo-header
    #[inline]
    pub fn authority(&self) -> Option<&str> {
        self.authority.as_deref()
    }
    ///Sets the `:authority` pseudo-header, returning `&mut Self` for chaining
    #[inline]
    pub fn set_authority(
        &mut self,
        authority: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.authority = authority.map(Into::into);
        self
    }
    ///Borrows the `:scheme` pseudo-header
    #[inline]
    pub fn scheme(&self) -> Option<&str> {
        self.scheme.as_deref()
    }
    ///Sets the `:scheme` pseudo-header, returning `&mut Self` for chaining
    #[inline]
    pub fn set_scheme(
        &mut self,
        scheme: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.scheme = scheme.map(Into::into);
        self
    }
    ///Borrows the `:protocol` pseudo-header (extended CONNECT)
    #[inline]
    pub fn protocol(&self) -> Option<&str> {
        self.protocol.as_deref()
    }
    ///Sets the `:protocol` pseudo-header (extended CONNECT), returning `&mut Self` for chaining
    #[inline]
    pub fn set_protocol(
        &mut self,
        protocol: Option<impl Into<Cow<'static, str>>>,
    ) -> &mut Self {
        self.protocol = protocol.map(Into::into);
        self
    }
    ///Borrows request trailers, populated after the request body has been fully read
    #[inline]
    pub fn request_trailers(&self) -> Option<&Headers> {
        self.request_trailers.as_ref()
    }
    ///Mutably borrow request trailers, populated after the request body has been fully read
    #[inline]
    pub fn request_trailers_mut(&mut self) -> Option<&mut Headers> {
        self.request_trailers.as_mut()
    }
}
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use __fieldwork_delegate_ConnInner as ConnInner;
