// ================================
// SMTP server setup to send mails
// https://kerkour.com/rust-send-email
// extern crate lettre;
//
// use lettre::{transport::smtp::authentication::Credentials,
//              AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
//
// async fn send_email_smtp(
//     mailer: &AsyncSmtpTransport<Tokio1Executor>,
//     from: &str,
//     to: &str,
//     subject: &str,
//     body: String,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let email = Message::builder()
//         .from(from.parse()?)
//         .to(to.parse()?)
//         .subject(subject)
//         .body(body.to_string())?;
//     mailer.send(email).await?;
//     Ok(())
// }
//
// pub async fn smtp_mail() -> Result<(), Box<dyn std::error::Error>> {
//     let smtp_creds = Credentials::new(
//         "".to_string(),
//         "".to_string(),
//     );
//     let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp-relay.sendinblue.com")?
//         .credentials(smtp_creds)
//         .build();
//     let from = "";
//     let to = "";
//     let subject = "Hello World";
//     let body = "<h1>Hello World</h1>".to_string();
//
//     send_email_smtp(&mailer, from, to, subject, body).await
// }
// ===============================
// use futures_util::StreamExt;
// max_size: usize = 250000
// Bytes data (mut payload: web::Payload)
// let mut body = web::BytesMut::new();
// while let Some(chunk) = payload.next().await {
//     let chunk = chunk.unwrap();
//     if (body.len() + chunk.len()) > MAX_SIZE {
//         return Err(error::ErrorBadRequest("overflow"))
//     }
//     body.extend_from_slice(&chunk);
// };
// println!("{:?}",body);
// println!("{:?}",request.headers().get("authorization").unwrap());
// ================================