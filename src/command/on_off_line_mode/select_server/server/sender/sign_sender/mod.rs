use structopt::StructOpt;
use std::str::FromStr;
use strum_macros::{
    Display,
    EnumString,
    EnumVariantNames,
};
use strum::VariantNames;
use dialoguer::{
    Select,
    Input,
    theme::ColorfulTheme,
    console::Term
};

mod receiver;
use receiver::{
    Receiver,
    CliReceiver
};



#[derive(Debug, StructOpt)]
pub struct SignKey {
    pub private_key: String,
    pub public_key: String,
    #[structopt(subcommand)]
    pub send_to: SendTo
}

#[derive(Debug, StructOpt)]
pub enum SendTo {
    receiver(Receiver)
}

#[derive(Debug, StructOpt)]
pub struct CliSignKey {
    private_key: Option<String>,
    public_key: Option<String>,
    #[structopt(subcommand)]
    send_to: Option<CliSendTo> 
}

#[derive(Debug, StructOpt)]
pub enum CliSendTo {
    receiver(CliReceiver)
}

#[derive(Debug, StructOpt)]
pub struct SignAlternative {
    pub key_chain: String,
    #[structopt(subcommand)]
    pub send_to: SendTo
}

#[derive(Debug, StructOpt)]
pub struct CliSignAlternative {
    key_chain: Option<String>,
    #[structopt(subcommand)]
    send_to: Option<CliSendTo> 
}

impl SendTo {
    pub fn send_to() -> Self {
        SendTo::receiver(Receiver{})
    }
}

impl SignKey {
    pub fn input_private_key() -> String {
        Input::new()
            .with_prompt("Enter the private key")
            .interact_text()
            .unwrap()
    }
    pub fn input_public_key() -> String {
        Input::new()
            .with_prompt("Enter the public key")
            .interact_text()
            .unwrap()
    }
}

impl From<CliSignKey> for SignKey {
    fn from(item: CliSignKey) -> Self {
        let private_key: String = match item.private_key {
            Some(cli_private_key) => cli_private_key,
            None => SignKey::input_private_key()
        };
        let public_key: String = match item.public_key {
            Some(cli_public_key) => cli_public_key,
            None => SignKey::input_public_key()
        };
        let send_to: SendTo = SendTo::send_to();
        SignKey {
            private_key,
            public_key,
            send_to
        }
    }
}

impl SignAlternative {
    pub fn input_key_chain() -> String {
        Input::new()
            .with_prompt("Enter the key chain")
            .interact_text()
            .unwrap()
    }
}

impl From<CliSignAlternative> for SignAlternative {
    fn from(item: CliSignAlternative) -> Self {
        println!("***********.  cli sign alternative {:?}", &item);
        let key_chain: String = match item.key_chain {
            Some(cli_key_chain) => cli_key_chain,
            None => SignAlternative::input_key_chain()
        };
        let send_to: SendTo = SendTo::send_to();
        SignAlternative {
            key_chain,
            send_to
        }
    }
}
