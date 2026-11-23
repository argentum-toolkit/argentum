use argentum_standard_business::di_container::DiError;
use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};

/// Builds an SMTP transport.
///
/// `tls_mode`: `tls`, `starttls` (default) or `none` (e.g. for a local MailHog).
pub fn build_smtp_transport(
    host: &str,
    port: u16,
    tls_mode: &str,
    username: &str,
    password: &str,
) -> Result<SmtpTransport, DiError> {
    let builder = match tls_mode {
        "none" => Ok(SmtpTransport::builder_dangerous(host).port(port)),
        "tls" => SmtpTransport::relay(host).map(|builder| builder.port(port)),
        _ => SmtpTransport::starttls_relay(host).map(|builder| builder.port(port)),
    }
    .map_err(|e| {
        DiError::with_context(format!("Can't build SMTP transport for host `{host}`"), e)
    })?;

    let builder = if username.is_empty() {
        builder
    } else {
        builder.credentials(Credentials::new(username.to_string(), password.to_string()))
    };

    Ok(builder.build())
}
