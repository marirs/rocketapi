use openssl::{
    asn1::Asn1Time,
    hash::MessageDigest,
    nid::Nid,
    pkey::PKey,
    rsa::Rsa,
    x509::{X509Name, X509},
};

use std::{
    fs,
    fs::File,
    io::{Result, Write},
};

use crate::server::config::SSL;

pub fn generate_cert(cert_config: SSL) -> Result<()> {
    //! Generates a SSL Certificate
    //!
    //! ## Example usage:
    //!
    //! ```ignore
    //! generate_cert(cert_configuration)
    //! ```
    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();

    let mut name = X509Name::builder().unwrap();
    name.append_entry_by_nid(Nid::COMMONNAME, &cert_config.common_name)
        .unwrap();
    let name = name.build();
    let time_before = Asn1Time::days_from_now(0).unwrap();
    let time_after = Asn1Time::days_from_now(cert_config.certificate_validity).unwrap();
    let mut builder = X509::builder().unwrap();
    builder.set_version(1).unwrap();
    builder.set_subject_name(&name).unwrap();
    builder.set_issuer_name(&name).unwrap();
    builder.set_pubkey(&pkey).unwrap();
    builder.set_not_before(time_before.as_ref()).unwrap();
    builder.set_not_after(time_after.as_ref()).unwrap();
    builder.sign(&pkey, MessageDigest::sha256()).unwrap();

    let certificate: X509 = builder.build();
    fs::create_dir("private")?;
    let mut cert = File::create("private/cert")?;
    let mut key = File::create("private/key")?;
    cert.write_all(&certificate.to_pem().unwrap())?;
    key.write_all(&pkey.private_key_to_pem_pkcs8().unwrap())?;
    Ok(())
}
