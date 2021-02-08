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

mod sign_sender;
use sign_sender::{
    SignKey,
    CliSignKey,
    SignAlternative,
    CliSignAlternative,
    SendTo
};

use super::SendFrom;




#[derive(Debug, StructOpt)]
pub struct CliSender {
    pub account_id: Option<String>,
    #[structopt(subcommand)]
    auth: Option<CliAuthentication> 
}
#[derive(Debug, StructOpt)]
pub enum CliAuthentication {
    private_key(CliSignKey),
    alternativ(CliSignAlternative)
}

#[derive(Debug,  StructOpt)]
pub struct Sender {
    pub account_id: String,
    #[structopt(subcommand)]
    pub auth: Authentication
}

impl Sender {
    pub fn input_account_id() -> String {
        Input::new()
            .with_prompt("What is the account ID of the sender?")
            .interact_text()
            .unwrap()
    }
}

#[derive(Debug, StructOpt)]
pub enum Authentication {
    private_key(SignKey),
    alternativ(SignAlternative)
}

impl From<CliSender> for Sender {
    fn from(item: CliSender) -> Self {
        println!("    ///////  From<CliSender> for Sender   /////////// item: CliSender:   {:?}", item);
        let account_id: String = match item.account_id {
            Some(cli_account_id) => cli_account_id,
            None => Sender::input_account_id()
        };
        let auth: Authentication = Authentication::choose_authentication();
        Sender {
            account_id,
            auth
        }
    }
}

impl Authentication {
    pub fn choose_authentication() -> Self {// реализовать функцию
        Authentication::private_key(SignKey{
            private_key: "private key".to_string(),
            public_key: "public key".to_string(),
            send_to: SendTo::send_to()
        })    
    }
}

impl From<CliAuthentication> for Authentication {
    fn from(item: CliAuthentication) -> Self {
        match item {
            CliAuthentication::private_key(cli_sign_key) => {
                let sign_key = SignKey::from(cli_sign_key);
                Authentication::private_key(sign_key)
            },
            CliAuthentication::alternativ(cli_sign_alternative) => {
                let sign_alternative = SignAlternative::from(cli_sign_alternative);
                Authentication::alternativ(sign_alternative)
            },
        }
    }
}
