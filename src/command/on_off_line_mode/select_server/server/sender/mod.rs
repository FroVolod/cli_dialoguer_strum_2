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
    alternative(CliSignAlternative)
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
    alternative(SignAlternative)
}

impl From<CliSender> for Sender {
    fn from(item: CliSender) -> Self {
        println!("    ///////  From<CliSender> for Sender   /////////// item: CliSender:   {:?}", item);
        let account_id: String = match item.account_id {
            Some(cli_account_id) => cli_account_id,
            None => Sender::input_account_id()
        };
        let auth: Authentication = match item.auth {
            Some(cli_auth) => Authentication::from(cli_auth),
            None => Authentication::choose_authentication()
        }; 
        Sender {
            account_id,
            auth
        }
    }
}

impl Authentication {
    pub fn choose_authentication() -> Self {// реализовать функцию

        println!("Works Authentication!");
        let authentication_options = vec![
            "Yes, I want to sign the transaction with my private key",
            "No, I want to construct the transaction and sign it somewhere else",
        ];
        let select_authentication_options = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to sign the transaction?")
            .items(&authentication_options)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        // let send_from = SendFrom::send_from();
        match select_authentication_options {
            Some(0) => Authentication::private_key(SignKey {
                private_key: SignKey::input_private_key(),
                public_key: SignKey::input_public_key(),
                send_to: SendTo::send_to()
            }),
            Some(1) => Authentication::alternative(SignAlternative {
                key_chain: SignAlternative::input_key_chain(),
                send_to: SendTo::send_to()
            }),
            _ => unreachable!("Error")
        }
    }
}

impl From<CliAuthentication> for Authentication {
    fn from(item: CliAuthentication) -> Self {
        match item {
            CliAuthentication::private_key(cli_sign_key) => {
                let sign_key = SignKey::from(cli_sign_key);
                Authentication::private_key(sign_key)
            },
            CliAuthentication::alternative(cli_sign_alternative) => {
                let sign_alternative = SignAlternative::from(cli_sign_alternative);
                Authentication::alternative(sign_alternative)
            },
        }
    }
}
