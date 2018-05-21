extern crate lettre;

#[cfg(test)]
#[cfg(feature = "sendmail-transport")]
mod test {

    use lettre::{EmailAddress, EmailTransport, SimpleSendableEmail};
    use lettre::sendmail::SendmailTransport;

    // TODO (darren): these don't work in the Docker environment, look into removing the
    // lettre submodule..
    #[test]
    #[ignore]
    fn sendmail_transport_simple() {
        let mut sender = SendmailTransport::new();
        let email = SimpleSendableEmail::new(
            EmailAddress::new("user@localhost".to_string()),
            vec![EmailAddress::new("root@localhost".to_string())],
            "sendmail_id".to_string(),
            "Hello sendmail".to_string(),
        );

        let result = sender.send(&email);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

}
