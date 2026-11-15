use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    //for tcp and http
    pub port: u32,
    //for http and tcp clients
    pub host: String,
    //timeout in seconds
    pub timeout: u32,
    //Keyword string
    pub keyword: String,
    //log file to log our commands by default if not set then it shall log
    // to STDOUT
    pub log_file: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            host: String::from("http://127.0.0.1"),
            timeout: 5,
            keyword: String::from("Jarvis"),
            log_file: None,
        }
    }
}
