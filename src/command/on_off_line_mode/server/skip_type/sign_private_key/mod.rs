use structopt::StructOpt;
use dialoguer::{
    Select,
    Input,
    theme::ColorfulTheme,
    console::Term
};



#[derive(Debug, Default, StructOpt)]
pub struct SignPrivateKey {
    signer_public_key: String,
    signer_secret_key: String
}

impl SignPrivateKey {
    pub fn signer_keys() -> Self {
        let signer_public_key: String = Input::new()
            .with_prompt("enter sender's public key")
            .interact_text()
            .unwrap();
        let signer_secret_key: String = Input::new()
            .with_prompt("enter sender's private key")
            .interact_text()
            .unwrap();
        Self{signer_public_key, signer_secret_key}
    }
}
