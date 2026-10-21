//! trillium-http's `Upgrade`, moved behind a box: item-level methods, deprecated renames, explicit
//! deref types, and method-level opt-outs. The public methods on `Upgrade` should be unchanged.
use std::{borrow::Cow, net::IpAddr, ops::Deref, sync::Arc, time::Instant};
pub struct Headers;
pub struct Method;
pub struct TypeSet;
pub struct HttpContext;
pub struct Status;
pub struct Version;
pub struct Buffer(Vec<u8>);
impl Deref for Buffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}
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
/// An HTTP upgrade
#[fieldwork(get, get_mut, set, with, take, into_field, rename_predicates)]
pub struct Upgrade<Transport> {
    #[field(delegate, skip)]
    inner: Box<UpgradeInner>,
    /// The underlying io (often a `TcpStream` or similar)
    transport: Transport,
}
impl<Transport> Upgrade<Transport> {
    ///Borrows The underlying io (often a `TcpStream` or similar)
    #[inline]
    pub fn transport(&self) -> &Transport {
        &self.transport
    }
    ///Mutably borrow The underlying io (often a `TcpStream` or similar)
    #[inline]
    pub fn transport_mut(&mut self) -> &mut Transport {
        &mut self.transport
    }
    ///Sets The underlying io (often a `TcpStream` or similar), returning `&mut Self` for chaining
    #[inline]
    pub fn set_transport(&mut self, transport: Transport) -> &mut Self {
        self.transport = transport;
        self
    }
    ///Owned chainable setter for The underlying io (often a `TcpStream` or similar), returning `Self`
    #[inline]
    #[must_use]
    pub fn with_transport(mut self, transport: Transport) -> Self {
        self.transport = transport;
        self
    }
    ///Consumes self, returning The underlying io (often a `TcpStream` or similar)
    #[inline]
    pub fn into_transport(self) -> Transport {
        self.transport
    }
}
impl<Transport> Upgrade<Transport> {
    ///Borrows The http headers the peer sent to us
    #[inline]
    pub fn received_headers(&self) -> &Headers {
        &self.inner.received_headers
    }
    ///Borrows The http headers the peer sent to us
    #[deprecated(since = "1.3.0", note = "use `received_headers` instead")]
    #[inline]
    pub fn request_headers(&self) -> &Headers {
        &self.inner.received_headers
    }
    ///Mutably borrow The http headers the peer sent to us
    #[inline]
    pub fn received_headers_mut(&mut self) -> &mut Headers {
        &mut self.inner.received_headers
    }
    ///Mutably borrow The http headers the peer sent to us
    #[deprecated(since = "1.3.0", note = "use `received_headers_mut` instead")]
    #[inline]
    pub fn request_headers_mut(&mut self) -> &mut Headers {
        &mut self.inner.received_headers
    }
    ///Sets The http headers the peer sent to us, returning `&mut Self` for chaining
    #[inline]
    pub fn set_received_headers(&mut self, received_headers: Headers) -> &mut Self {
        self.inner.received_headers = received_headers;
        self
    }
    ///Sets The http headers the peer sent to us, returning `&mut Self` for chaining
    #[deprecated(since = "1.3.0", note = "use `set_received_headers` instead")]
    #[inline]
    pub fn set_request_headers(&mut self, request_headers: Headers) -> &mut Self {
        self.inner.received_headers = request_headers;
        self
    }
    ///Owned chainable setter for The http headers the peer sent to us, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_received_headers(mut self, received_headers: Headers) -> Self {
        self.inner.received_headers = received_headers;
        self
    }
    ///Owned chainable setter for The http headers the peer sent to us, returning `Self`
    #[deprecated(since = "1.3.0", note = "use `with_received_headers` instead")]
    #[inline]
    #[must_use]
    pub fn with_request_headers(mut self, request_headers: Headers) -> Self {
        self.inner.received_headers = request_headers;
        self
    }
    ///Consumes self, returning The http headers the peer sent to us
    #[inline]
    pub fn into_received_headers(self) -> Headers {
        self.inner.received_headers
    }
    ///Consumes self, returning The http headers the peer sent to us
    #[deprecated(since = "1.3.0", note = "use `into_received_headers` instead")]
    #[inline]
    pub fn into_request_headers(self) -> Headers {
        self.inner.received_headers
    }
    /**Borrows The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[inline]
    pub fn sent_headers(&self) -> &Headers {
        &self.inner.sent_headers
    }
    /**Borrows The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[deprecated(since = "1.3.0", note = "use `sent_headers` instead")]
    #[inline]
    pub fn response_headers(&self) -> &Headers {
        &self.inner.sent_headers
    }
    /**Mutably borrow The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[inline]
    pub fn sent_headers_mut(&mut self) -> &mut Headers {
        &mut self.inner.sent_headers
    }
    /**Mutably borrow The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[deprecated(since = "1.3.0", note = "use `sent_headers_mut` instead")]
    #[inline]
    pub fn response_headers_mut(&mut self) -> &mut Headers {
        &mut self.inner.sent_headers
    }
    /**Sets The http headers as set before the upgrade was negotiated and sent, returning `&mut Self` for chaining
to the peer.*/
    #[inline]
    pub fn set_sent_headers(&mut self, sent_headers: Headers) -> &mut Self {
        self.inner.sent_headers = sent_headers;
        self
    }
    /**Sets The http headers as set before the upgrade was negotiated and sent, returning `&mut Self` for chaining
to the peer.*/
    #[deprecated(since = "1.3.0", note = "use `set_sent_headers` instead")]
    #[inline]
    pub fn set_response_headers(&mut self, response_headers: Headers) -> &mut Self {
        self.inner.sent_headers = response_headers;
        self
    }
    /**Owned chainable setter for The http headers as set before the upgrade was negotiated and sent, returning `Self`
to the peer.*/
    #[inline]
    #[must_use]
    pub fn with_sent_headers(mut self, sent_headers: Headers) -> Self {
        self.inner.sent_headers = sent_headers;
        self
    }
    /**Owned chainable setter for The http headers as set before the upgrade was negotiated and sent, returning `Self`
to the peer.*/
    #[deprecated(since = "1.3.0", note = "use `with_sent_headers` instead")]
    #[inline]
    #[must_use]
    pub fn with_response_headers(mut self, response_headers: Headers) -> Self {
        self.inner.sent_headers = response_headers;
        self
    }
    /**Consumes self, returning The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[inline]
    pub fn into_sent_headers(self) -> Headers {
        self.inner.sent_headers
    }
    /**Consumes self, returning The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[deprecated(since = "1.3.0", note = "use `into_sent_headers` instead")]
    #[inline]
    pub fn into_response_headers(self) -> Headers {
        self.inner.sent_headers
    }
    ///Mutably borrow The request path
    #[inline]
    pub fn path_mut(&mut self) -> &mut Cow<'static, str> {
        &mut self.inner.path
    }
    ///Sets The request path, returning `&mut Self` for chaining
    #[inline]
    pub fn set_path(&mut self, path: Cow<'static, str>) -> &mut Self {
        self.inner.path = path;
        self
    }
    ///Owned chainable setter for The request path, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_path(mut self, path: Cow<'static, str>) -> Self {
        self.inner.path = path;
        self
    }
    ///Consumes self, returning The request path
    #[inline]
    pub fn into_path(self) -> Cow<'static, str> {
        self.inner.path
    }
    ///Returns a copy of The http request method
    #[inline]
    pub fn method(&self) -> Method {
        self.inner.method
    }
    ///Mutably borrow The http request method
    #[inline]
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.inner.method
    }
    ///Sets The http request method, returning `&mut Self` for chaining
    #[inline]
    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.inner.method = method;
        self
    }
    ///Owned chainable setter for The http request method, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_method(mut self, method: Method) -> Self {
        self.inner.method = method;
        self
    }
    ///Borrows Any state that has been accumulated on the Conn before negotiating the upgrade
    #[inline]
    pub fn state(&self) -> &TypeSet {
        &self.inner.state
    }
    ///Mutably borrow Any state that has been accumulated on the Conn before negotiating the upgrade
    #[inline]
    pub fn state_mut(&mut self) -> &mut TypeSet {
        &mut self.inner.state
    }
    ///Sets Any state that has been accumulated on the Conn before negotiating the upgrade, returning `&mut Self` for chaining
    #[inline]
    pub fn set_state(&mut self, state: TypeSet) -> &mut Self {
        self.inner.state = state;
        self
    }
    ///Owned chainable setter for Any state that has been accumulated on the Conn before negotiating the upgrade, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_state(mut self, state: TypeSet) -> Self {
        self.inner.state = state;
        self
    }
    ///Consumes self, returning Any state that has been accumulated on the Conn before negotiating the upgrade
    #[inline]
    pub fn into_state(self) -> TypeSet {
        self.inner.state
    }
    ///Borrows Any bytes that have been read from the underlying transport already.
    #[inline]
    pub fn buffer(&self) -> &[u8] {
        &*self.inner.buffer
    }
    ///Borrows The [`HttpContext`] shared for this server
    #[inline]
    pub fn context(&self) -> &Arc<HttpContext> {
        &self.inner.context
    }
    ///Mutably borrow The [`HttpContext`] shared for this server
    #[inline]
    pub fn context_mut(&mut self) -> &mut Arc<HttpContext> {
        &mut self.inner.context
    }
    ///Sets The [`HttpContext`] shared for this server, returning `&mut Self` for chaining
    #[inline]
    pub fn set_context(&mut self, context: Arc<HttpContext>) -> &mut Self {
        self.inner.context = context;
        self
    }
    ///Owned chainable setter for The [`HttpContext`] shared for this server, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_context(mut self, context: Arc<HttpContext>) -> Self {
        self.inner.context = context;
        self
    }
    ///Consumes self, returning The [`HttpContext`] shared for this server
    #[inline]
    pub fn into_context(self) -> Arc<HttpContext> {
        self.inner.context
    }
    ///Returns a copy of the ip address of the connection, if available
    #[inline]
    pub fn peer_ip(&self) -> Option<IpAddr> {
        self.inner.peer_ip
    }
    ///Mutably borrow the ip address of the connection, if available
    #[inline]
    pub fn peer_ip_mut(&mut self) -> Option<&mut IpAddr> {
        self.inner.peer_ip.as_mut()
    }
    ///Sets the ip address of the connection, if available, returning `&mut Self` for chaining
    #[inline]
    pub fn set_peer_ip(&mut self, peer_ip: Option<IpAddr>) -> &mut Self {
        self.inner.peer_ip = peer_ip;
        self
    }
    ///Owned chainable setter for the ip address of the connection, if available, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_peer_ip(mut self, peer_ip: Option<IpAddr>) -> Self {
        self.inner.peer_ip = peer_ip;
        self
    }
    ///Takes the ip address of the connection, if available, leaving a None in its place
    #[inline]
    pub fn take_peer_ip(&mut self) -> Option<IpAddr> {
        self.inner.peer_ip.take()
    }
    ///Returns a copy of the wall-clock time at which the underlying `Conn` was constructed
    #[inline]
    pub fn start_time(&self) -> Instant {
        self.inner.start_time
    }
    ///Mutably borrow the wall-clock time at which the underlying `Conn` was constructed
    #[inline]
    pub fn start_time_mut(&mut self) -> &mut Instant {
        &mut self.inner.start_time
    }
    ///Sets the wall-clock time at which the underlying `Conn` was constructed, returning `&mut Self` for chaining
    #[inline]
    pub fn set_start_time(&mut self, start_time: Instant) -> &mut Self {
        self.inner.start_time = start_time;
        self
    }
    ///Owned chainable setter for the wall-clock time at which the underlying `Conn` was constructed, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_start_time(mut self, start_time: Instant) -> Self {
        self.inner.start_time = start_time;
        self
    }
    ///Borrows the :authority http/3 pseudo-header
    #[inline]
    pub fn authority(&self) -> Option<&str> {
        self.inner.authority.as_deref()
    }
    ///Mutably borrow the :authority http/3 pseudo-header
    #[inline]
    pub fn authority_mut(&mut self) -> Option<&mut Cow<'static, str>> {
        self.inner.authority.as_mut()
    }
    ///Sets the :authority http/3 pseudo-header, returning `&mut Self` for chaining
    #[inline]
    pub fn set_authority(&mut self, authority: Option<Cow<'static, str>>) -> &mut Self {
        self.inner.authority = authority;
        self
    }
    ///Owned chainable setter for the :authority http/3 pseudo-header, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_authority(mut self, authority: Option<Cow<'static, str>>) -> Self {
        self.inner.authority = authority;
        self
    }
    ///Takes the :authority http/3 pseudo-header, leaving a None in its place
    #[inline]
    pub fn take_authority(&mut self) -> Option<Cow<'static, str>> {
        self.inner.authority.take()
    }
    ///Consumes self, returning the :authority http/3 pseudo-header
    #[inline]
    pub fn into_authority(self) -> Option<Cow<'static, str>> {
        self.inner.authority
    }
    ///Borrows the :scheme http/3 pseudo-header
    #[inline]
    pub fn scheme(&self) -> Option<&str> {
        self.inner.scheme.as_deref()
    }
    ///Mutably borrow the :scheme http/3 pseudo-header
    #[inline]
    pub fn scheme_mut(&mut self) -> Option<&mut Cow<'static, str>> {
        self.inner.scheme.as_mut()
    }
    ///Sets the :scheme http/3 pseudo-header, returning `&mut Self` for chaining
    #[inline]
    pub fn set_scheme(&mut self, scheme: Option<Cow<'static, str>>) -> &mut Self {
        self.inner.scheme = scheme;
        self
    }
    ///Owned chainable setter for the :scheme http/3 pseudo-header, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_scheme(mut self, scheme: Option<Cow<'static, str>>) -> Self {
        self.inner.scheme = scheme;
        self
    }
    ///Takes the :scheme http/3 pseudo-header, leaving a None in its place
    #[inline]
    pub fn take_scheme(&mut self) -> Option<Cow<'static, str>> {
        self.inner.scheme.take()
    }
    ///Consumes self, returning the :scheme http/3 pseudo-header
    #[inline]
    pub fn into_scheme(self) -> Option<Cow<'static, str>> {
        self.inner.scheme
    }
    ///Borrows the :protocol http/3 pseudo-header
    #[inline]
    pub fn protocol(&self) -> Option<&str> {
        self.inner.protocol.as_deref()
    }
    ///Mutably borrow the :protocol http/3 pseudo-header
    #[inline]
    pub fn protocol_mut(&mut self) -> Option<&mut Cow<'static, str>> {
        self.inner.protocol.as_mut()
    }
    ///Sets the :protocol http/3 pseudo-header, returning `&mut Self` for chaining
    #[inline]
    pub fn set_protocol(&mut self, protocol: Option<Cow<'static, str>>) -> &mut Self {
        self.inner.protocol = protocol;
        self
    }
    ///Owned chainable setter for the :protocol http/3 pseudo-header, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_protocol(mut self, protocol: Option<Cow<'static, str>>) -> Self {
        self.inner.protocol = protocol;
        self
    }
    ///Takes the :protocol http/3 pseudo-header, leaving a None in its place
    #[inline]
    pub fn take_protocol(&mut self) -> Option<Cow<'static, str>> {
        self.inner.protocol.take()
    }
    ///Consumes self, returning the :protocol http/3 pseudo-header
    #[inline]
    pub fn into_protocol(self) -> Option<Cow<'static, str>> {
        self.inner.protocol
    }
    ///Borrows the http version
    #[inline]
    pub fn http_version(&self) -> &Version {
        &self.inner.version
    }
    ///Mutably borrow the http version
    #[inline]
    pub fn http_version_mut(&mut self) -> &mut Version {
        &mut self.inner.version
    }
    ///Sets the http version, returning `&mut Self` for chaining
    #[inline]
    pub fn set_http_version(&mut self, http_version: Version) -> &mut Self {
        self.inner.version = http_version;
        self
    }
    ///Owned chainable setter for the http version, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_http_version(mut self, http_version: Version) -> Self {
        self.inner.version = http_version;
        self
    }
    ///Consumes self, returning the http version
    #[inline]
    pub fn into_http_version(self) -> Version {
        self.inner.version
    }
    ///Returns a copy of the http response status set on the underlying `Conn` before the upgrade
    #[inline]
    pub fn status(&self) -> Option<Status> {
        self.inner.status
    }
    ///Mutably borrow the http response status set on the underlying `Conn` before the upgrade
    #[inline]
    pub fn status_mut(&mut self) -> Option<&mut Status> {
        self.inner.status.as_mut()
    }
    ///Sets the http response status set on the underlying `Conn` before the upgrade, returning `&mut Self` for chaining
    #[inline]
    pub fn set_status(&mut self, status: Option<Status>) -> &mut Self {
        self.inner.status = status;
        self
    }
    ///Owned chainable setter for the http response status set on the underlying `Conn` before the upgrade, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_status(mut self, status: Option<Status>) -> Self {
        self.inner.status = status;
        self
    }
    ///Takes the http response status set on the underlying `Conn` before the upgrade, leaving a None in its place
    #[inline]
    pub fn take_status(&mut self) -> Option<Status> {
        self.inner.status.take()
    }
    ///Returns a copy of whether this connection was deemed secure by the handler stack
    #[inline]
    pub fn is_secure(&self) -> bool {
        self.inner.secure
    }
    ///Mutably borrow whether this connection was deemed secure by the handler stack
    #[inline]
    pub fn secure_mut(&mut self) -> &mut bool {
        &mut self.inner.secure
    }
    ///Sets whether this connection was deemed secure by the handler stack, returning `&mut Self` for chaining
    #[inline]
    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.inner.secure = secure;
        self
    }
    ///Owned chainable setter for whether this connection was deemed secure by the handler stack, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.inner.secure = secure;
        self
    }
    /**Borrows Inbound trailers, populated either by a fully-consumed pre-upgrade body or by
the post-upgrade inbound state machine. `Some` only when non-empty.*/
    #[inline]
    pub fn received_trailers(&self) -> Option<&Headers> {
        self.inner.received_trailers.as_ref()
    }
    /**Mutably borrow Inbound trailers, populated either by a fully-consumed pre-upgrade body or by
the post-upgrade inbound state machine. `Some` only when non-empty.*/
    #[inline]
    pub fn received_trailers_mut(&mut self) -> Option<&mut Headers> {
        self.inner.received_trailers.as_mut()
    }
    /**Takes Inbound trailers, populated either by a fully-consumed pre-upgrade body or by, leaving a None in its place
the post-upgrade inbound state machine. `Some` only when non-empty.*/
    #[inline]
    pub fn take_received_trailers(&mut self) -> Option<Headers> {
        self.inner.received_trailers.take()
    }
}
#[fieldwork(get, get_mut, set, with, take, into_field, rename_predicates, delegatable)]
pub(crate) struct UpgradeInner {
    /// The http headers the peer sent to us
    #[field(deprecate(was = "request_headers", since = "1.3.0"))]
    pub(crate) received_headers: Headers,
    /// The http headers as set before the upgrade was negotiated and sent
    /// to the peer.
    #[field(deprecate(was = "response_headers", since = "1.3.0"))]
    pub(crate) sent_headers: Headers,
    /// The request path
    #[field(get = false)]
    pub(crate) path: Cow<'static, str>,
    /// The http request method
    #[field(copy)]
    pub(crate) method: Method,
    /// Any state that has been accumulated on the Conn before negotiating the upgrade
    pub(crate) state: TypeSet,
    /// Any bytes that have been read from the underlying transport already.
    #[field(
        deref = "[u8]",
        into_field = false,
        set = false,
        with = false,
        get_mut = false
    )]
    pub(crate) buffer: Buffer,
    /// The [`HttpContext`] shared for this server
    #[field(deref = false)]
    pub(crate) context: Arc<HttpContext>,
    /// the ip address of the connection, if available
    #[field(copy)]
    pub(crate) peer_ip: Option<IpAddr>,
    /// the wall-clock time at which the underlying `Conn` was constructed
    #[field(copy)]
    pub(crate) start_time: Instant,
    /// the :authority http/3 pseudo-header
    pub(crate) authority: Option<Cow<'static, str>>,
    /// the :scheme http/3 pseudo-header
    pub(crate) scheme: Option<Cow<'static, str>>,
    #[field = false]
    pub(crate) protocol_session: (),
    /// the :protocol http/3 pseudo-header
    pub(crate) protocol: Option<Cow<'static, str>>,
    /// the http version
    #[field = "http_version"]
    pub(crate) version: Version,
    /// the http response status set on the underlying `Conn` before the upgrade
    #[field(copy)]
    pub(crate) status: Option<Status>,
    /// whether this connection was deemed secure by the handler stack
    pub(crate) secure: bool,
    #[field = false]
    pub(crate) received_body_state: (),
    /// Inbound trailers, populated either by a fully-consumed pre-upgrade body or by
    /// the post-upgrade inbound state machine. `Some` only when non-empty.
    #[field(get, get_mut, take, set = false, with = false, into_field = false)]
    pub(crate) received_trailers: Option<Headers>,
    #[field = false]
    pub(crate) content_length_in: Option<u64>,
    #[field = false]
    pub(crate) peer_gone: Option<()>,
}
impl UpgradeInner {
    ///Borrows The http headers the peer sent to us
    #[inline]
    pub fn received_headers(&self) -> &Headers {
        &self.received_headers
    }
    ///Borrows The http headers the peer sent to us
    #[deprecated(since = "1.3.0", note = "use `received_headers` instead")]
    #[inline]
    pub fn request_headers(&self) -> &Headers {
        &self.received_headers
    }
    ///Mutably borrow The http headers the peer sent to us
    #[inline]
    pub fn received_headers_mut(&mut self) -> &mut Headers {
        &mut self.received_headers
    }
    ///Mutably borrow The http headers the peer sent to us
    #[deprecated(since = "1.3.0", note = "use `received_headers_mut` instead")]
    #[inline]
    pub fn request_headers_mut(&mut self) -> &mut Headers {
        &mut self.received_headers
    }
    ///Sets The http headers the peer sent to us, returning `&mut Self` for chaining
    #[inline]
    pub fn set_received_headers(&mut self, received_headers: Headers) -> &mut Self {
        self.received_headers = received_headers;
        self
    }
    ///Sets The http headers the peer sent to us, returning `&mut Self` for chaining
    #[deprecated(since = "1.3.0", note = "use `set_received_headers` instead")]
    #[inline]
    pub fn set_request_headers(&mut self, request_headers: Headers) -> &mut Self {
        self.received_headers = request_headers;
        self
    }
    ///Owned chainable setter for The http headers the peer sent to us, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_received_headers(mut self, received_headers: Headers) -> Self {
        self.received_headers = received_headers;
        self
    }
    ///Owned chainable setter for The http headers the peer sent to us, returning `Self`
    #[deprecated(since = "1.3.0", note = "use `with_received_headers` instead")]
    #[inline]
    #[must_use]
    pub fn with_request_headers(mut self, request_headers: Headers) -> Self {
        self.received_headers = request_headers;
        self
    }
    ///Consumes self, returning The http headers the peer sent to us
    #[inline]
    pub fn into_received_headers(self) -> Headers {
        self.received_headers
    }
    ///Consumes self, returning The http headers the peer sent to us
    #[deprecated(since = "1.3.0", note = "use `into_received_headers` instead")]
    #[inline]
    pub fn into_request_headers(self) -> Headers {
        self.received_headers
    }
    /**Borrows The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[inline]
    pub fn sent_headers(&self) -> &Headers {
        &self.sent_headers
    }
    /**Borrows The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[deprecated(since = "1.3.0", note = "use `sent_headers` instead")]
    #[inline]
    pub fn response_headers(&self) -> &Headers {
        &self.sent_headers
    }
    /**Mutably borrow The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[inline]
    pub fn sent_headers_mut(&mut self) -> &mut Headers {
        &mut self.sent_headers
    }
    /**Mutably borrow The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[deprecated(since = "1.3.0", note = "use `sent_headers_mut` instead")]
    #[inline]
    pub fn response_headers_mut(&mut self) -> &mut Headers {
        &mut self.sent_headers
    }
    /**Sets The http headers as set before the upgrade was negotiated and sent, returning `&mut Self` for chaining
to the peer.*/
    #[inline]
    pub fn set_sent_headers(&mut self, sent_headers: Headers) -> &mut Self {
        self.sent_headers = sent_headers;
        self
    }
    /**Sets The http headers as set before the upgrade was negotiated and sent, returning `&mut Self` for chaining
to the peer.*/
    #[deprecated(since = "1.3.0", note = "use `set_sent_headers` instead")]
    #[inline]
    pub fn set_response_headers(&mut self, response_headers: Headers) -> &mut Self {
        self.sent_headers = response_headers;
        self
    }
    /**Owned chainable setter for The http headers as set before the upgrade was negotiated and sent, returning `Self`
to the peer.*/
    #[inline]
    #[must_use]
    pub fn with_sent_headers(mut self, sent_headers: Headers) -> Self {
        self.sent_headers = sent_headers;
        self
    }
    /**Owned chainable setter for The http headers as set before the upgrade was negotiated and sent, returning `Self`
to the peer.*/
    #[deprecated(since = "1.3.0", note = "use `with_sent_headers` instead")]
    #[inline]
    #[must_use]
    pub fn with_response_headers(mut self, response_headers: Headers) -> Self {
        self.sent_headers = response_headers;
        self
    }
    /**Consumes self, returning The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[inline]
    pub fn into_sent_headers(self) -> Headers {
        self.sent_headers
    }
    /**Consumes self, returning The http headers as set before the upgrade was negotiated and sent
to the peer.*/
    #[deprecated(since = "1.3.0", note = "use `into_sent_headers` instead")]
    #[inline]
    pub fn into_response_headers(self) -> Headers {
        self.sent_headers
    }
    ///Mutably borrow The request path
    #[inline]
    pub fn path_mut(&mut self) -> &mut Cow<'static, str> {
        &mut self.path
    }
    ///Sets The request path, returning `&mut Self` for chaining
    #[inline]
    pub fn set_path(&mut self, path: Cow<'static, str>) -> &mut Self {
        self.path = path;
        self
    }
    ///Owned chainable setter for The request path, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_path(mut self, path: Cow<'static, str>) -> Self {
        self.path = path;
        self
    }
    ///Consumes self, returning The request path
    #[inline]
    pub fn into_path(self) -> Cow<'static, str> {
        self.path
    }
    ///Returns a copy of The http request method
    #[inline]
    pub fn method(&self) -> Method {
        self.method
    }
    ///Mutably borrow The http request method
    #[inline]
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }
    ///Sets The http request method, returning `&mut Self` for chaining
    #[inline]
    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }
    ///Owned chainable setter for The http request method, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }
    ///Borrows Any state that has been accumulated on the Conn before negotiating the upgrade
    #[inline]
    pub fn state(&self) -> &TypeSet {
        &self.state
    }
    ///Mutably borrow Any state that has been accumulated on the Conn before negotiating the upgrade
    #[inline]
    pub fn state_mut(&mut self) -> &mut TypeSet {
        &mut self.state
    }
    ///Sets Any state that has been accumulated on the Conn before negotiating the upgrade, returning `&mut Self` for chaining
    #[inline]
    pub fn set_state(&mut self, state: TypeSet) -> &mut Self {
        self.state = state;
        self
    }
    ///Owned chainable setter for Any state that has been accumulated on the Conn before negotiating the upgrade, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_state(mut self, state: TypeSet) -> Self {
        self.state = state;
        self
    }
    ///Consumes self, returning Any state that has been accumulated on the Conn before negotiating the upgrade
    #[inline]
    pub fn into_state(self) -> TypeSet {
        self.state
    }
    ///Borrows Any bytes that have been read from the underlying transport already.
    #[inline]
    pub fn buffer(&self) -> &[u8] {
        &*self.buffer
    }
    ///Borrows The [`HttpContext`] shared for this server
    #[inline]
    pub fn context(&self) -> &Arc<HttpContext> {
        &self.context
    }
    ///Mutably borrow The [`HttpContext`] shared for this server
    #[inline]
    pub fn context_mut(&mut self) -> &mut Arc<HttpContext> {
        &mut self.context
    }
    ///Sets The [`HttpContext`] shared for this server, returning `&mut Self` for chaining
    #[inline]
    pub fn set_context(&mut self, context: Arc<HttpContext>) -> &mut Self {
        self.context = context;
        self
    }
    ///Owned chainable setter for The [`HttpContext`] shared for this server, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_context(mut self, context: Arc<HttpContext>) -> Self {
        self.context = context;
        self
    }
    ///Consumes self, returning The [`HttpContext`] shared for this server
    #[inline]
    pub fn into_context(self) -> Arc<HttpContext> {
        self.context
    }
    ///Returns a copy of the ip address of the connection, if available
    #[inline]
    pub fn peer_ip(&self) -> Option<IpAddr> {
        self.peer_ip
    }
    ///Mutably borrow the ip address of the connection, if available
    #[inline]
    pub fn peer_ip_mut(&mut self) -> Option<&mut IpAddr> {
        self.peer_ip.as_mut()
    }
    ///Sets the ip address of the connection, if available, returning `&mut Self` for chaining
    #[inline]
    pub fn set_peer_ip(&mut self, peer_ip: Option<IpAddr>) -> &mut Self {
        self.peer_ip = peer_ip;
        self
    }
    ///Owned chainable setter for the ip address of the connection, if available, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_peer_ip(mut self, peer_ip: Option<IpAddr>) -> Self {
        self.peer_ip = peer_ip;
        self
    }
    ///Takes the ip address of the connection, if available, leaving a None in its place
    #[inline]
    pub fn take_peer_ip(&mut self) -> Option<IpAddr> {
        self.peer_ip.take()
    }
    ///Returns a copy of the wall-clock time at which the underlying `Conn` was constructed
    #[inline]
    pub fn start_time(&self) -> Instant {
        self.start_time
    }
    ///Mutably borrow the wall-clock time at which the underlying `Conn` was constructed
    #[inline]
    pub fn start_time_mut(&mut self) -> &mut Instant {
        &mut self.start_time
    }
    ///Sets the wall-clock time at which the underlying `Conn` was constructed, returning `&mut Self` for chaining
    #[inline]
    pub fn set_start_time(&mut self, start_time: Instant) -> &mut Self {
        self.start_time = start_time;
        self
    }
    ///Owned chainable setter for the wall-clock time at which the underlying `Conn` was constructed, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_start_time(mut self, start_time: Instant) -> Self {
        self.start_time = start_time;
        self
    }
    ///Borrows the :authority http/3 pseudo-header
    #[inline]
    pub fn authority(&self) -> Option<&str> {
        self.authority.as_deref()
    }
    ///Mutably borrow the :authority http/3 pseudo-header
    #[inline]
    pub fn authority_mut(&mut self) -> Option<&mut Cow<'static, str>> {
        self.authority.as_mut()
    }
    ///Sets the :authority http/3 pseudo-header, returning `&mut Self` for chaining
    #[inline]
    pub fn set_authority(&mut self, authority: Option<Cow<'static, str>>) -> &mut Self {
        self.authority = authority;
        self
    }
    ///Owned chainable setter for the :authority http/3 pseudo-header, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_authority(mut self, authority: Option<Cow<'static, str>>) -> Self {
        self.authority = authority;
        self
    }
    ///Takes the :authority http/3 pseudo-header, leaving a None in its place
    #[inline]
    pub fn take_authority(&mut self) -> Option<Cow<'static, str>> {
        self.authority.take()
    }
    ///Consumes self, returning the :authority http/3 pseudo-header
    #[inline]
    pub fn into_authority(self) -> Option<Cow<'static, str>> {
        self.authority
    }
    ///Borrows the :scheme http/3 pseudo-header
    #[inline]
    pub fn scheme(&self) -> Option<&str> {
        self.scheme.as_deref()
    }
    ///Mutably borrow the :scheme http/3 pseudo-header
    #[inline]
    pub fn scheme_mut(&mut self) -> Option<&mut Cow<'static, str>> {
        self.scheme.as_mut()
    }
    ///Sets the :scheme http/3 pseudo-header, returning `&mut Self` for chaining
    #[inline]
    pub fn set_scheme(&mut self, scheme: Option<Cow<'static, str>>) -> &mut Self {
        self.scheme = scheme;
        self
    }
    ///Owned chainable setter for the :scheme http/3 pseudo-header, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_scheme(mut self, scheme: Option<Cow<'static, str>>) -> Self {
        self.scheme = scheme;
        self
    }
    ///Takes the :scheme http/3 pseudo-header, leaving a None in its place
    #[inline]
    pub fn take_scheme(&mut self) -> Option<Cow<'static, str>> {
        self.scheme.take()
    }
    ///Consumes self, returning the :scheme http/3 pseudo-header
    #[inline]
    pub fn into_scheme(self) -> Option<Cow<'static, str>> {
        self.scheme
    }
    ///Borrows the :protocol http/3 pseudo-header
    #[inline]
    pub fn protocol(&self) -> Option<&str> {
        self.protocol.as_deref()
    }
    ///Mutably borrow the :protocol http/3 pseudo-header
    #[inline]
    pub fn protocol_mut(&mut self) -> Option<&mut Cow<'static, str>> {
        self.protocol.as_mut()
    }
    ///Sets the :protocol http/3 pseudo-header, returning `&mut Self` for chaining
    #[inline]
    pub fn set_protocol(&mut self, protocol: Option<Cow<'static, str>>) -> &mut Self {
        self.protocol = protocol;
        self
    }
    ///Owned chainable setter for the :protocol http/3 pseudo-header, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_protocol(mut self, protocol: Option<Cow<'static, str>>) -> Self {
        self.protocol = protocol;
        self
    }
    ///Takes the :protocol http/3 pseudo-header, leaving a None in its place
    #[inline]
    pub fn take_protocol(&mut self) -> Option<Cow<'static, str>> {
        self.protocol.take()
    }
    ///Consumes self, returning the :protocol http/3 pseudo-header
    #[inline]
    pub fn into_protocol(self) -> Option<Cow<'static, str>> {
        self.protocol
    }
    ///Borrows the http version
    #[inline]
    pub fn http_version(&self) -> &Version {
        &self.version
    }
    ///Mutably borrow the http version
    #[inline]
    pub fn http_version_mut(&mut self) -> &mut Version {
        &mut self.version
    }
    ///Sets the http version, returning `&mut Self` for chaining
    #[inline]
    pub fn set_http_version(&mut self, http_version: Version) -> &mut Self {
        self.version = http_version;
        self
    }
    ///Owned chainable setter for the http version, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_http_version(mut self, http_version: Version) -> Self {
        self.version = http_version;
        self
    }
    ///Consumes self, returning the http version
    #[inline]
    pub fn into_http_version(self) -> Version {
        self.version
    }
    ///Returns a copy of the http response status set on the underlying `Conn` before the upgrade
    #[inline]
    pub fn status(&self) -> Option<Status> {
        self.status
    }
    ///Mutably borrow the http response status set on the underlying `Conn` before the upgrade
    #[inline]
    pub fn status_mut(&mut self) -> Option<&mut Status> {
        self.status.as_mut()
    }
    ///Sets the http response status set on the underlying `Conn` before the upgrade, returning `&mut Self` for chaining
    #[inline]
    pub fn set_status(&mut self, status: Option<Status>) -> &mut Self {
        self.status = status;
        self
    }
    ///Owned chainable setter for the http response status set on the underlying `Conn` before the upgrade, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_status(mut self, status: Option<Status>) -> Self {
        self.status = status;
        self
    }
    ///Takes the http response status set on the underlying `Conn` before the upgrade, leaving a None in its place
    #[inline]
    pub fn take_status(&mut self) -> Option<Status> {
        self.status.take()
    }
    ///Returns a copy of whether this connection was deemed secure by the handler stack
    #[inline]
    pub fn is_secure(&self) -> bool {
        self.secure
    }
    ///Mutably borrow whether this connection was deemed secure by the handler stack
    #[inline]
    pub fn secure_mut(&mut self) -> &mut bool {
        &mut self.secure
    }
    ///Sets whether this connection was deemed secure by the handler stack, returning `&mut Self` for chaining
    #[inline]
    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.secure = secure;
        self
    }
    ///Owned chainable setter for whether this connection was deemed secure by the handler stack, returning `Self`
    #[inline]
    #[must_use]
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }
    /**Borrows Inbound trailers, populated either by a fully-consumed pre-upgrade body or by
the post-upgrade inbound state machine. `Some` only when non-empty.*/
    #[inline]
    pub fn received_trailers(&self) -> Option<&Headers> {
        self.received_trailers.as_ref()
    }
    /**Mutably borrow Inbound trailers, populated either by a fully-consumed pre-upgrade body or by
the post-upgrade inbound state machine. `Some` only when non-empty.*/
    #[inline]
    pub fn received_trailers_mut(&mut self) -> Option<&mut Headers> {
        self.received_trailers.as_mut()
    }
    /**Takes Inbound trailers, populated either by a fully-consumed pre-upgrade body or by, leaving a None in its place
the post-upgrade inbound state machine. `Some` only when non-empty.*/
    #[inline]
    pub fn take_received_trailers(&mut self) -> Option<Headers> {
        self.received_trailers.take()
    }
}
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use __fieldwork_delegate_UpgradeInner as UpgradeInner;
