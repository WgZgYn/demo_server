use openssl::error::ErrorStack;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

pub fn config_ssl() -> Result<SslAcceptorBuilder, ErrorStack> {
    // 配置证书信息
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("private.key", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("cert.pem")?;
    Ok(builder)
}
