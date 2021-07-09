use std::{io, fs};
use rustls::internal::pemfile;

pub struct SslConfigBuiler {
    ssl_config: rustls::ServerConfig
}

impl SslConfigBuiler {
    pub fn new() -> Self{
        Self {
           ssl_config: rustls::ServerConfig::new(rustls::NoClientAuth::new())
        }
    }

    pub fn set_ssl_files(&mut self, pem_path: &str, private_key_path: &str) {
        let pem_file = fs::File::open(pem_path).unwrap();
        let pem_file_reader = &mut io::BufReader::new(pem_file);
        let cert_chain = pemfile::certs(pem_file_reader).unwrap();

        let private_key = fs::File::open(private_key_path).unwrap();
        let private_key_reader = &mut io::BufReader::new(private_key);
        let mut keys = pemfile::rsa_private_keys(private_key_reader).unwrap();

        self.ssl_config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    }

    pub fn build(self) -> rustls::ServerConfig {
        self.ssl_config
    }
}