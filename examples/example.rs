use std::path::PathBuf;

#[derive(fieldwork::Fieldwork, Default)]
#[fieldwork(get, set, with, get_mut, into, option_set_some, rename_predicates)]
struct ServerConfig {
    /// server hostname
    host: String,

    /// server port
    #[fieldwork(into = false)]
    port: u16,

    /// path to SSL certificate file
    #[fieldwork(option_borrow_inner = false)]
    ssl_cert: Option<PathBuf>,

    /// path to log directory  
    log_dir: Option<PathBuf>,

    /// whether TLS is required
    tls_required: bool,

    /// whether verbose logging is enabled
    verbose: bool,

    #[fieldwork(skip)]
    _runtime_data: (),
}

fn main() {
    // Usage examples:
    let mut config = ServerConfig::default()
        .with_host("LocalHost") // accepts &str via Into<String>
        .with_port(8080)
        .with_log_dir("/var/log") // accepts &str, wraps in Some() automatically
        .with_tls_required(true)
        .with_verbose(false);

    config.host_mut().make_ascii_lowercase();

    // Getters use idiomatic naming
    assert_eq!(config.host(), "localhost");
    assert_eq!(config.port(), 8080);
    assert_eq!(config.log_dir().unwrap(), PathBuf::from("/var/log"));
    assert!(config.is_tls_required()); // boolean getters use is_ prefix because of `rename_predicates`
    assert!(!config.is_verbose());

    // SSL cert requires explicit Option since option_set_some = false
    // Chainable setters return &mut Self
    config
        .set_ssl_cert(PathBuf::from("/etc/ssl/cert.pem"))
        .set_port(9090)
        .set_verbose(true);

    let cert = config.ssl_cert_mut().take();
    assert_eq!(cert, Some(PathBuf::from("/etc/ssl/cert.pem")));
}
