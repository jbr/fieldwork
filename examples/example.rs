#[allow(dead_code)]
fn main() {
    use std::path::PathBuf;

    #[derive(fieldwork::Fieldwork, Default)]
    #[fieldwork(get, set, with, without, get_mut, rename_predicates)]
    struct ServerConfig {
        /// server hostname
        #[field(into)] // accept Into<String>
        host: String,

        /// server port
        port: u16,

        /// path to log directory
        #[field(into, option_set_some)] // accept Into<PathBuf>
        log_dir: Option<PathBuf>,

        /// whether TLS is required
        tls_required: bool,

        /// whether verbose logging is enabled
        verbose: bool,
    }

    let mut config = ServerConfig::default()
        .with_host("LocalHost") // accepts &str via Into<String>
        .with_port(8080)
        .with_log_dir("/var/log") // accepts &str, wraps in Some automatically
        .with_tls_required(); // sets bool to true

    config.host_mut().make_ascii_lowercase();

    assert_eq!(config.host(), "localhost"); // String → &str
    assert_eq!(config.port(), 8080);
    assert_eq!(
        config.log_dir().unwrap(),
        PathBuf::from("/var/log").as_path()
    );
    assert!(config.is_tls_required()); // rename_predicates: bool getters use is_ prefix
    assert!(!config.is_verbose());

    // Chainable setters return &mut Self
    config.set_port(9090).set_verbose(true);

    config = config.without_log_dir(); // clears Option to None
    assert!(config.log_dir().is_none());

    #[derive(fieldwork::Fieldwork)]
    #[fieldwork(get, into_field)]
    enum ServerEvent {
        Started {
            host: String,
            port: u16,
        },
        Request {
            host: String,
            port: u16,
            path: String,
        },
        Shutdown {
            host: String,
            port: u16,
        },
    }

    let event = ServerEvent::Request {
        host: "example.com".to_string(),
        port: 8080,
        path: "/api/health".to_string(),
    };

    // host and port appear in every variant → full coverage, same smart defaults as structs
    assert_eq!(event.host(), "example.com"); // String → &str
    assert_eq!(event.port(), 8080); // Copy types returned by value

    // path appears in only one variant → partial coverage, return type wrapped in Option
    assert_eq!(event.path(), Some("/api/health"));
    assert_eq!(
        ServerEvent::Started {
            host: "example.com".to_string(),
            port: 8080
        }
        .path(),
        None,
    );

    // into_field is generated for full-coverage fields
    assert_eq!(event.into_host(), "example.com");
}
