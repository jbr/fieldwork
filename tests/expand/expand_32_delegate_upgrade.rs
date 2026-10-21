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

/// An HTTP upgrade
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, set, with, take, into_field, rename_predicates)]
pub struct Upgrade<Transport> {
    #[field(delegate, skip)]
    inner: Box<UpgradeInner>,

    /// The underlying io (often a `TcpStream` or similar)
    transport: Transport,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(
    get,
    get_mut,
    set,
    with,
    take,
    into_field,
    rename_predicates,
    delegatable
)]
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
