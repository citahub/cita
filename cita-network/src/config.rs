use serde_derive::Deserialize;
use util::parse_config;

#[derive(Debug, Deserialize, Clone)]
pub struct NetConfig {
    pub port: Option<usize>,
    pub peers: Option<Vec<PeerConfig>>,
    pub max_connects: Option<usize>,
    pub enable_tls: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PeerConfig {
    pub ip: Option<String>,
    pub port: Option<usize>,
}

impl NetConfig {
    pub fn new(path: &str) -> Self {
        parse_config!(NetConfig, path)
    }
}

#[cfg(test)]
mod tests {
    use super::NetConfig;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn basic_test() {
        let toml_str = r#"
        port = 4000
        enable_tls = true
        max_connects = 4
        id_card = 9
        [[peers]]
            ip = "0.0.0.0"
            port = 4001
            common_name = "test1.cita"
        [[peers]]
            ip = "0.0.0.0"
            port = 4002
        "#;

        let mut tmp_file: NamedTempFile = NamedTempFile::new().unwrap();
        tmp_file.write_all(toml_str.as_bytes()).unwrap();
        let path = tmp_file.path().to_str().unwrap();
        let config = NetConfig::new(path);

        assert_eq!(config.port, Some(4000));
        assert_eq!(config.max_connects, Some(4));
        assert_eq!(config.enable_tls, Some(true));
        assert_eq!(config.peers.unwrap().len(), 2);
    }
}
