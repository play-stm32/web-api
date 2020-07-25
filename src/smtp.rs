use lettre::{SmtpClient, ClientSecurity, Transport, ClientTlsParameters};
use lettre::smtp::authentication::IntoCredentials;
use lettre::smtp::error::SmtpResult;
use native_tls::TlsConnector;
use lettre_email::{EmailBuilder, Email};
use lettre_email::error::Error;

pub fn check_email(to: &str) -> Result<Email, Error> {
    EmailBuilder::new()
        .to(to)
        .from("example@example.com")
        .build()
}

pub fn send_email(to: &str, message: &str) -> SmtpResult {
    let smtp_address = "xxx";
    let username = "xxxx";
    let password = "xxxx";

    let email = EmailBuilder::new()
        .to(to)
        .from(username)
        .subject("active your account")
        .text(message)
        .build()
        .unwrap()
        .into();

    let credentials = (username, password).into_credentials();
    let tls = ClientTlsParameters::new(smtp_address.into(), TlsConnector::new().unwrap());
    let client_secure = ClientSecurity::Required(tls);

    let mut client = SmtpClient::new((smtp_address, 587), client_secure)
        .unwrap()
        .credentials(credentials)
        .smtp_utf8(true)
        .transport();

    client.send(email)
}