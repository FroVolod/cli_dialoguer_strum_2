use std::str::FromStr;

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
    pub fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: String,
    ) {
        println!("SignPrivateKey process: self:       {:?}", &self);
        println!("SignPrivateKey process: prepopulated_unsigned_transaction:       {:?}", &prepopulated_unsigned_transaction);
        let public_key = near_crypto::PublicKey::from_str(&self.signer_public_key).unwrap();
        let signer_secret_key = near_crypto::SecretKey::from_str(&self.signer_secret_key).unwrap();
        let unsigned_transaction = near_primitives::transaction::Transaction {
            public_key,
            .. prepopulated_unsigned_transaction
        };
        println!("unsigned_transaction:  {:#?}", &unsigned_transaction);
        
        
    }

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
