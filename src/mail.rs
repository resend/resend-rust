use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Mail<'a> {
  from: &'a str,
  to: &'a [&'a str],
  subject: &'a str,
  html: &'a str,
}

impl<'a> Mail<'a> {
  pub const fn new(from: &'a str, to: &'a [&'a str], subject: &'a str, html: &'a str) -> Self {
    Self {
      from,
      to,
      subject,
      html,
    }
  }
}

impl fmt::Display for Mail<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let tos = self
      .to
      .iter()
      .map(|el| format!("\"{el}\""))
      .collect::<Vec<_>>()
      .join(", ");

    write!(
      f,
      "{{\"from\": \"{}\", \"to\": [{}], \"subject\": \"{}\", \"html\": \"{}\"}}",
      &self.from, tos, &self.subject, &self.html
    )
  }
}

#[cfg(test)]
mod tests {
  use super::Mail;

  #[test]
  fn to_string_test() {
    let mail = Mail::new("from1", &["to1"], "subject1", "html1");

    let expected = r#"{"from": "from1", "to": ["to1"], "subject": "subject1", "html": "html1"}"#;

    assert_eq!(mail.to_string(), expected.to_owned());
  }

  #[test]
  fn to_string_test_multiple_recipients() {
    let mail = Mail::new("from1", &["to1", "to2"], "subject1", "html1");

    let expected =
      r#"{"from": "from1", "to": ["to1", "to2"], "subject": "subject1", "html": "html1"}"#;

    assert_eq!(mail.to_string(), expected.to_owned());
  }
}
