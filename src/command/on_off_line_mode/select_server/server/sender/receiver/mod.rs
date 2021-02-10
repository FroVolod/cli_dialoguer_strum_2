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

mod transfer_near_tokens_type;
use transfer_near_tokens_type::{
    TransferNEARTokens,
    CliTransferNEARTokens,
    NearBalance
};
mod skip_type;
use skip_type::{
    SignTransaction,
    Skip,
    CliSkip
};
mod action_to_account_type;
use action_to_account_type::{
    ActionToAccount,
    CliActionToAccount
};
mod delete_access_key_type;
use delete_access_key_type::{
    DeleteAccessKey,
    CliDeleteAccessKey
};
mod add_access_key_type;
use add_access_key_type::{
    AddAccessKey,
    CliAddAccessKey
};



#[derive(Debug, StructOpt)]
pub struct Receiver {
    pub account_id: String,
    #[structopt(subcommand)]
    pub transaction_subcommand: ActionSubcommand
}

#[derive(Debug, EnumVariantNames, StructOpt)]
pub enum ActionSubcommand {
    TransferNEARTokens(TransferNEARTokens),
    CallFunction,
    StakeNEARTokens,
    CreateAccount(ActionToAccount),
    DeleteAccount(ActionToAccount),
    AddAccessKey(AddAccessKey),
    DeleteAccessKey(DeleteAccessKey),
    Skip(Skip)
}

#[derive(Debug, StructOpt)]
pub struct CliReceiver {
    account_id: Option<String>,
    #[structopt(subcommand)]
    transaction_subcommand: Option<CliActionSubcommand> 
}

#[derive(Debug, StructOpt)]
pub enum CliActionSubcommand {
    TransferNEARTokens(CliTransferNEARTokens),
    CallFunction,
    StakeNEARTokens,
    CreateAccount(CliActionToAccount),
    DeleteAccount(CliActionToAccount),
    AddAccessKey(CliAddAccessKey),
    DeleteAccessKey(CliDeleteAccessKey),
    Skip(CliSkip)
}

#[derive(Debug, StructOpt)]
pub enum CliActionSkipSubcommand {
    Skip
}


// impl Default for ActionSubcommand {
//     fn default() -> Self {
//         ActionSubcommand::Skip(Skip{
//             sign_option: SignTransaction::choose_sign_option()
//         })
//     }
// }

impl ActionSubcommand {
    pub fn choose_action_command() -> Self {
        let action_subcommands= ActionSubcommand::VARIANTS;
        let select_action_subcommand = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an action that you want to add to the action:")
            .items(&action_subcommands)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        match select_action_subcommand {
            Some(0) => {
                let amount: NearBalance = NearBalance::input_amount();
                let next_action: Box<ActionSubcommand> = Box::new(ActionSubcommand::choose_action_command());
                ActionSubcommand::TransferNEARTokens(TransferNEARTokens {
                    amount,
                    next_action
                })
            },
            Some(1) => ActionSubcommand::CallFunction,
            Some(2) => ActionSubcommand::StakeNEARTokens,
            Some(3) => {
                let account_id: String = ActionToAccount::input_account_id();
                let next_action: Box<ActionSubcommand> = Box::new(ActionSubcommand::choose_action_command());
                ActionSubcommand::CreateAccount(ActionToAccount {
                    account_id,
                    next_action
                })
            },
            Some(4) => {
                let account_id: String = ActionToAccount::input_account_id();
                let next_action: Box<ActionSubcommand> = Box::new(ActionSubcommand::choose_action_command());
                ActionSubcommand::DeleteAccount(ActionToAccount {
                    account_id,
                    next_action
                })
            },
            Some(5) => {
                let next_action: Box<ActionSubcommand> = Box::new(ActionSubcommand::choose_action_command());
                ActionSubcommand::AddAccessKey(AddAccessKey {
                    next_action
                })
            },
            Some(6) => {
                let access_key: String = DeleteAccessKey::input_access_key();
                let next_action: Box<ActionSubcommand> = Box::new(ActionSubcommand::choose_action_command());
                ActionSubcommand::DeleteAccessKey(DeleteAccessKey {
                    access_key,
                    next_action
                })
            },
            Some(7) => ActionSubcommand::Skip(Skip{sign_option: SignTransaction::choose_sign_option()}),
            _ => unreachable!("Error")
        }
    }
}

impl Receiver {
    pub fn input_account_id() -> String {
        Input::new()
            .with_prompt("What is the account ID of the receiver?")
            .interact_text()
            .unwrap()
    }
}

impl From<CliReceiver> for Receiver {
    fn from(item: CliReceiver) -> Self {
        let account_id: String = match item.account_id {
            Some(cli_account_id) => cli_account_id,
            None => Receiver::input_account_id()
        };
        let transaction_subcommand: ActionSubcommand = match item.transaction_subcommand {
            Some(cli_action_subcommand) => ActionSubcommand::from(cli_action_subcommand),
            None => ActionSubcommand::choose_action_command()
        };
        Receiver {
            account_id,
            transaction_subcommand
        }
    }
}

impl From<CliActionSubcommand> for ActionSubcommand {
    fn from(item: CliActionSubcommand) -> Self {
        println!("-----------  From<CliActionSubcommand> for ActionSubcommand   -------------- {:?}", &item);
        match item {
            CliActionSubcommand::TransferNEARTokens(cli_transfer_near_token) => {
                let transfer_near_token: TransferNEARTokens = TransferNEARTokens::from(cli_transfer_near_token);
                ActionSubcommand::TransferNEARTokens(transfer_near_token)
            },
            CliActionSubcommand::CreateAccount(cli_create_account) => {
                let create_account: ActionToAccount = ActionToAccount::from(cli_create_account);
                ActionSubcommand::CreateAccount(create_account)
            },
            CliActionSubcommand::DeleteAccount(cli_delete_account) => {
                let delete_account: ActionToAccount = ActionToAccount::from(cli_delete_account);
                ActionSubcommand::DeleteAccount(delete_account)
            },
            CliActionSubcommand::AddAccessKey(cli_add_access_key) => {
                let add_access_key: AddAccessKey = AddAccessKey::from(cli_add_access_key);
                ActionSubcommand::AddAccessKey(add_access_key)
            },
            CliActionSubcommand::DeleteAccessKey(cli_delete_access_key) => {
                let delete_access_key: DeleteAccessKey = DeleteAccessKey::from(cli_delete_access_key);
                ActionSubcommand::DeleteAccessKey(delete_access_key)
            },
            _ => unreachable!("Error")
        }
    }
}

impl From<CliActionSkipSubcommand> for ActionSubcommand {
    fn from(item: CliActionSkipSubcommand) -> Self {
        match item {
            _ => ActionSubcommand::Skip(Skip{sign_option: SignTransaction::choose_sign_option()}),

        }
    }
}
