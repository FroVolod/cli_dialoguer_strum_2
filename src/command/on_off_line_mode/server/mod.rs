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

// mod transfer_near_tokens_type;
// use transfer_near_tokens_type::TransferNEARTokens;
// mod skip_type;
// use skip_type::{SignTransaction, Skip};

mod sender;
use sender::{
    Sender,
    CliSender,
    Authentication,
    CliAuthentication
};

use crate::Args;


#[derive(Debug, StructOpt)]
pub struct Server {
    pub url: String,
    #[structopt(subcommand)]
    pub send_from: SendFrom
    // #[structopt(subcommand)]
    // pub transaction_subcommand: ActionSubcommand
}

// impl Default for Server {
//     fn default() -> Self {
//         Server{
//             url: "".to_string(),
//             send_from: SendFrom::send_from()
//             }
//     } 
// }

#[derive(Debug, StructOpt)]
pub enum SendFrom {
    sender(Sender)
}

// impl Default for SendFrom {
//     fn default() -> Self {
//         SendFrom::send_from()
//     }
// }

#[derive(Debug, StructOpt)]
pub struct CliServer {
    #[structopt(subcommand)]
    send_from: Option<CliSendFrom> 
    // #[structopt(subcommand)]
    // pub transaction_subcommand: ActionSubcommand
}

#[derive(Debug, StructOpt)]
pub struct CliCustomServer {
    pub url: Option<String>,
    #[structopt(subcommand)]
    send_from: Option<CliSendFrom> 
    // #[structopt(subcommand)]
    // pub transaction_subcommand: ActionSubcommand
}

#[derive(Debug, StructOpt)]
pub enum CliSendFrom {
    sender(CliSender)
}

impl CliServer {
    pub fn into_server(self, url: String) -> Server {
        println!("=================================== item.url  {:?}", self);
        // println!("{:?}", super::OnlineArgs);
        let send_from: SendFrom = match self.send_from {
            Some(cli_send_from) => SendFrom::from(cli_send_from),
            None => SendFrom::send_from()
        };
        Server {
            url,
            send_from,
        }
    }
}

impl CliCustomServer {
    pub fn into_server(self) -> Server {
        println!("=================================== item.url  {:?}", self);
        // println!("{:?}", super::OnlineArgs);
        let url = match self.url {
            Some(url) => url,
            None => {
                Input::new()
                    .with_prompt("What is the RPC endpoi?")
                    .interact_text()
                    .unwrap()
            }
        };
        let send_from: SendFrom = match self.send_from {
            Some(cli_send_from) => SendFrom::from(cli_send_from),
            None => SendFrom::send_from()
        };
        Server {
            url,
            send_from,
        }
    }
}

impl From<CliSendFrom> for SendFrom {
    fn from(item: CliSendFrom) -> Self {
        
        match item {
            CliSendFrom::sender(cli_sender) => {
                let sender: Sender = Sender::from(cli_sender);
                
                SendFrom::sender(sender)
            },
            _ => unreachable!("Error")
        }
    }
}

impl SendFrom {
    pub fn send_from() -> Self {
        let account_id : String = Input::new()
            .with_prompt("What is the account ID of the sender?")
            .interact_text()
            .unwrap();
        SendFrom::sender(Sender { account_id })
    }
}


// #[derive(Debug, EnumVariantNames, StructOpt)]
// pub enum ActionSubcommand {
//     TransferNEARTokens(TransferNEARTokens),
//     CallFunction,
//     StakeNEARTokens,
//     CreateAccount,
//     DeleteAccount,
//     AddAccessKey,
//     DeteteAccessKey,
//     Skip(Skip)
// }

// impl Default for ActionSubcommand {
//     fn default() -> Self {
//         ActionSubcommand::Skip(Skip::default())
//     }
// }

// impl ActionSubcommand {
//     pub fn choose_action_command() -> Self {
//         let action_subcommands= ActionSubcommand::VARIANTS;
//         let select_action_subcommand = Select::with_theme(&ColorfulTheme::default())
//             .with_prompt("Select an action that you want to add to the action:")
//             .items(&action_subcommands)
//             .default(0)
//             .interact_on_opt(&Term::stderr())
//             .unwrap();
//         match select_action_subcommand {
//             Some(0) => ActionSubcommand::TransferNEARTokens(TransferNEARTokens::input_amount()),
//             Some(1) => ActionSubcommand::CallFunction,
//             Some(7) => ActionSubcommand::Skip(Skip::choose_sign_option()),
//             _ => ActionSubcommand::default()
//         }
//     }
// }
