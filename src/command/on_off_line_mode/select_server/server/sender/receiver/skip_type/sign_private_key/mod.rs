use structopt::StructOpt;
use dialoguer::{
    Select,
    Input,
    theme::ColorfulTheme,
    console::Term
};



#[derive(Debug, Default, StructOpt)]
pub struct SignPrivateKey {
    pub signer_public_key: String,
    pub signer_secret_key: String
}

#[derive(Debug, Default, StructOpt)]
pub struct CliSignPrivateKey {
    signer_public_key: Option<String>,
    signer_secret_key: Option<String>
}

impl SignPrivateKey {
    pub fn signer_public_key() -> String {
        Input::new()
            .with_prompt("enter sender's public key")
            .interact_text()
            .unwrap()
    }
    pub fn signer_secret_key() -> String {
        Input::new()
            .with_prompt("enter sender's private key")
            .interact_text()
            .unwrap()
    }
}

impl From<CliSignPrivateKey> for SignPrivateKey {
    fn from(item: CliSignPrivateKey) -> Self {
        let signer_public_key: String = match item.signer_public_key {
            Some(cli_public_key) => cli_public_key,
            None => SignPrivateKey::signer_public_key()
        };
        let signer_secret_key: String = match item.signer_secret_key {
            Some(cli_secret_key) => cli_secret_key,
            None => SignPrivateKey::signer_secret_key()
        };
        SignPrivateKey {
            signer_public_key,
            signer_secret_key
        }
    }
}
