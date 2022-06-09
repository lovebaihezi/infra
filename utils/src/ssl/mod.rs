use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader, path::Path};

pub fn ssl_init<T>(key_pem: T, cert_pem: T) -> anyhow::Result<ServerConfig>
where
    T: AsRef<Path>,
{
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    let cert = &mut BufReader::new(File::open(cert_pem)?);
    let key = &mut BufReader::new(File::open(key_pem)?);
    let cert_chain = certs(cert)?.into_iter().map(Certificate).collect();
    let mut key: Vec<_> = pkcs8_private_keys(key)?
        .into_iter()
        .map(PrivateKey)
        .collect();
    assert!(
        !key.is_empty(),
        "key should not be empty!Could not locale the pkcs8 key"
    );
    Ok(config.with_single_cert(cert_chain, key.remove(0)).unwrap())
}
