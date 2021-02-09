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
    CliReceiver,
    ActionSubcommand,
    CliActionSubcommand
};

use super::SendFrom;



#[derive(Debug,  StructOpt)]
pub struct Sender {
    pub account_id: String,
    #[structopt(subcommand)]
    pub send_to: SendTo
}

#[derive(Debug, StructOpt)]
pub enum SendTo {
    Receiver(Receiver)
}

#[derive(Debug, StructOpt)]
pub struct CliSender {
    pub account_id: Option<String>,
    #[structopt(subcommand)]
    send_to: Option<CliSendTo> 
}
#[derive(Debug, StructOpt)]
pub enum CliSendTo {
    Receiver(CliReceiver),
}


impl Sender {
    pub fn input_account_id() -> String {
        Input::new()
            .with_prompt("What is the account ID of the sender?")
            .interact_text()
            .unwrap()
    }
}


impl From<CliSender> for Sender {
    fn from(item: CliSender) -> Self {
        println!("    ///////  From<CliSender> for Sender   /////////// item: CliSender:   {:?}", item);
        let account_id: String = match item.account_id {
            Some(cli_account_id) => cli_account_id,
            None => Sender::input_account_id()
        };
        let send_to: SendTo = match item.send_to {
            Some(cli_send_to) => SendTo::from(cli_send_to),
            None => SendTo::send_to()
        }; 
        Sender {
            account_id,
            send_to
        }
    }
}

impl SendTo {
    pub fn send_to() -> Self {
        let account_id: String = Receiver::input_account_id();
        let transaction_subcommand: ActionSubcommand = ActionSubcommand::choose_action_command();
        SendTo::Receiver(Receiver {
            account_id,
            transaction_subcommand
        })
    }
}

impl From<CliSendTo> for SendTo {
    fn from(item: CliSendTo) -> Self {
        match item {
            CliSendTo::Receiver(cli_receiver) => {
                let receiver = Receiver::from(cli_receiver);
                SendTo::Receiver(receiver)
            },
            _ => unreachable!("Error")
        }
    }
}

// impl Authentication {
//     pub fn choose_authentication() -> Self {// реализовать функцию

//         println!("Works Authentication!");
//         let authentication_options = vec![
//             "Yes, I want to sign the transaction with my private key",
//             "No, I want to construct the transaction and sign it somewhere else",
//         ];
//         let select_authentication_options = Select::with_theme(&ColorfulTheme::default())
//             .with_prompt("Would you like to sign the transaction?")
//             .items(&authentication_options)
//             .default(0)
//             .interact_on_opt(&Term::stderr())
//             .unwrap();
//         // let send_from = SendFrom::send_from();
//         match select_authentication_options {
//             Some(0) => Authentication::private_key(SignKey {
//                 private_key: SignKey::input_private_key(),
//                 public_key: SignKey::input_public_key(),
//                 send_to: SendTo::send_to()
//             }),
//             Some(1) => Authentication::alternative(SignAlternative {
//                 key_chain: SignAlternative::input_key_chain(),
//                 send_to: SendTo::send_to()
//             }),
//             _ => unreachable!("Error")
//         }
//     }
// }

// impl From<CliAuthentication> for Authentication {
//     fn from(item: CliAuthentication) -> Self {
//         match item {
//             CliAuthentication::private_key(cli_sign_key) => {
//                 let sign_key = SignKey::from(cli_sign_key);
//                 Authentication::private_key(sign_key)
//             },
//             CliAuthentication::alternative(cli_sign_alternative) => {
//                 let sign_alternative = SignAlternative::from(cli_sign_alternative);
//                 Authentication::alternative(sign_alternative)
//             },
//         }
//     }
// }
