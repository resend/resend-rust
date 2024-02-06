use reqwest::header::CONTENT_TYPE;

use crate::error::Error;

use crate::mail::Mail;

const URL: &str = "https://api.resend.com/emails";

/// A *very* minimal [Resend](https://resend.com) client for sending emails with HTML contents.
///
/// # Examples
///
/// ```no_run
/// use resend_rs::{mail::Mail, resend_client::ResendClient};
///
/// let mail = Mail::new(
///   "<SENDER EMAIL>",
///   &["<RECEIVER EMAIL>"],
///   "subject",
///   "html",
/// );
///
/// let client = ResendClient::new("<API KEY>".to_owned());
/// let res = client.send(mail);
/// println!("{res:?}");
/// ```
#[derive(Debug, Clone)]
pub struct ResendClient {
  api_key: String,
  #[cfg(not(feature = "async"))]
  client: reqwest::blocking::Client,
  #[cfg(feature = "async")]
  client: reqwest::Client,
}

impl ResendClient {
  pub fn new(api_key: String) -> Self {
    #[cfg(not(feature = "async"))]
    let client = reqwest::blocking::Client::new();
    #[cfg(feature = "async")]
    let client = reqwest::Client::new();

    Self { api_key, client }
  }

  #[cfg(not(feature = "async"))]
  pub fn send(&self, mail: Mail<'_>) -> Result<(), Error> {
    let res = self
      .client
      .post(URL)
      .bearer_auth(&self.api_key)
      .header(CONTENT_TYPE, "application/json")
      .body(mail.to_string())
      .send()?;

    if res.status().is_success() {
      Ok(())
    } else {
      Err(Error::ResendError(res.text()?))
    }
  }

  #[cfg(feature = "async")]
  pub async fn send_async(&self, mail: Mail<'_>) -> Result<(), Error> {
    let res = self
      .client
      .post(URL)
      .bearer_auth(&self.api_key)
      .header(CONTENT_TYPE, "application/json")
      .body(mail.to_string())
      .send()
      .await?;

    if res.status().is_success() {
      Ok(())
    } else {
      Err(Error::ResendError(res.text().await?))
    }
  }
}
