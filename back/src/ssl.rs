use std::{io, fs};
use rustls_pemfile::{certs, rsa_private_keys};

pub struct SslConfigBuiler {
    ssl_config: rustls::ServerConfig
}

impl SslConfigBuiler {
    pub fn new(pem_path: &str, private_key_path: &str) -> Self{
        let pem_file = fs::File::open(pem_path).unwrap();
        let pem_file_reader = &mut io::BufReader::new(pem_file);
        let cert_chain = certs(pem_file_reader).unwrap().into_iter().map(|raw_cert| {
            return rustls::Certificate(raw_cert)
        }).collect::<Vec<rustls::Certificate>>();

        let private_key = fs::File::open(private_key_path).unwrap();
        let private_key_reader = &mut io::BufReader::new(private_key);
        let mut keys = rsa_private_keys(private_key_reader).unwrap().into_iter().map(|raw_key| {
            return rustls::PrivateKey(raw_key)
        }).collect::<Vec<rustls::PrivateKey>>();

        Self {
           ssl_config: rustls::ServerConfig::builder()
               .with_safe_default_cipher_suites()
               .with_safe_default_kx_groups()
               .with_safe_default_protocol_versions()
               .unwrap()
               .with_no_client_auth()
               .with_single_cert(cert_chain, keys.remove(0))
               .expect("bad certificate/key")
        }
    }

    pub fn build(self) -> rustls::ServerConfig {
        self.ssl_config
    }
}