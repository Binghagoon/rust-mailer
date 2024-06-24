use dotenv::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

fn main() -> () {
    dotenv().ok();
    let sender_email =
        env::var("EMAIL_SENDER_EMAIL").expect("Error: EMAIL_SENDER_EMAIL must be set");
    let sender_name = env::var("EMAIL_SENDER_NAME")
        .or_else(|e| {
            println!("Warning: EMAIL_SENDER_NAME is not set");
            Err(e)
        })
        .ok();
    let reply = env::var("EMAIL_REPLY")
        .or_else(|e| {
            println!("Warning: EMAIL_REPLY is not set, using sender email as reply-to");
            Err(e)
        })
        .unwrap_or(sender_email.clone());
    let to = env::var("EMAIL_TO").expect("Error: EMAIL_TO must be set");
    let relay = env::var("EMAIL_RELAY").expect("Error: EMAIL_RELAY must be set");
    // Define the email
    let email = Message::builder()
        .from(lettre::message::Mailbox {
            name: sender_name,
            email: sender_email
                .parse()
                .expect("Error: Sender address is invalid"),
        })
        .reply_to(reply.parse().expect("Error: Reply-to address is invalid"))
        .to(to.parse().expect("Error: Recipient address is invalid"))
        .subject("Rust Email")
        .body(String::from("Hello, this is a test email from Rust!"))
        .unwrap();
    let username = env::var("EMAIL_USERNAME").expect("Error: EMAIL_USERNAME must be set");
    let password = env::var("EMAIL_PASSWORD").expect("Error: EMAIL_PASSWORD must be set");
    // Set up the SMTP client
    let credentials = Credentials::new(username, password);
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&relay)
        .expect("Error: Failed to connect to server")
        .credentials(credentials)
        .build();
    let result = mailer.send(&email);

    result
        .and_then(|_| {
            println!("Alert: Email sent successfully");
            Ok(())
        })
        .expect("Error: Failed to send email")
}
