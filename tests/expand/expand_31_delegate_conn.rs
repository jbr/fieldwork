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

// Written out rather than derived, since std's derive expands differently across toolchains
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
#[derive(fieldwork::Fieldwork)]
pub struct Conn<Transport> {
    #[field(delegate)]
    inner: Box<ConnInner>,

    /// the transport
    #[field(get, get_mut)]
    transport: Transport,
}

#[derive(fieldwork::Fieldwork)]
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
