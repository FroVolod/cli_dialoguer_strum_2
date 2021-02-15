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
mod create_account_type;
use create_account_type::{
    CreateAccount,
    CliCreateAccount
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
mod delete_account_type;
use delete_account_type::{
    DeleteAccount,
    CliDeleteAccount
};



#[derive(Debug)]
pub struct Receiver {
    pub account_id: String,
    pub transaction_subcommand: ActionSubcommand
}

#[derive(Debug, EnumVariantNames)]
pub enum ActionSubcommand {
    TransferNEARTokens(TransferNEARTokens),
    CallFunction,
    StakeNEARTokens,
    CreateAccount(CreateAccount),
    DeleteAccount(DeleteAccount),
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
    CreateAccount(CliCreateAccount),
    DeleteAccount(CliDeleteAccount),
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
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: String,
    ) {
        println!("ActionSubcommand process: self:       {:?}", &self);
        println!("ActionSubcommand process: prepopulated_unsigned_transaction:       {:?}", &prepopulated_unsigned_transaction);
        match self {
            ActionSubcommand::TransferNEARTokens(args_transfer) => args_transfer.process(prepopulated_unsigned_transaction, selected_server_url).await,
            // ActionSubcommand::CallFunction(args_function) => {},
            // ActionSubcommand::StakeNEARTokens(args_stake) => {},
            ActionSubcommand::CreateAccount(args_create_account) => {},
            ActionSubcommand::DeleteAccount(args_delete_account) => {},
            ActionSubcommand::AddAccessKey(args_add_access_key) => {},
            ActionSubcommand::DeleteAccessKey(args_delete_access_key) => {},
            ActionSubcommand::Skip(args_skip) => {},
            _ => unreachable!("Error")
        }
    }

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
                let next_action: Box<ActionSubcommand> = Box::new(ActionSubcommand::choose_action_command());
                ActionSubcommand::CreateAccount(CreateAccount {
                    next_action
                })
            },
            Some(4) => {
                let beneficiary_id: String = DeleteAccount::input_beneficiary_id();
                let next_action: Box<ActionSubcommand> = Box::new(ActionSubcommand::choose_action_command());
                ActionSubcommand::DeleteAccount(DeleteAccount {
                    beneficiary_id,
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
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: String,
    ) {
        println!("Receiver process: self:       {:?}", &self);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            receiver_id: self.account_id.clone(),
            .. prepopulated_unsigned_transaction
        };
        self.transaction_subcommand.process(unsigned_transaction, selected_server_url).await;

    }

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
                let create_account: CreateAccount = CreateAccount::from(cli_create_account);
                ActionSubcommand::CreateAccount(create_account)
            },
            CliActionSubcommand::DeleteAccount(cli_delete_account) => {
                let delete_account: DeleteAccount = DeleteAccount::from(cli_delete_account);
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
