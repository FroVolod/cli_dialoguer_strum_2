use structopt::StructOpt;
use strum_macros::{
    EnumVariantNames,
};
use strum::VariantNames;
use dialoguer::{
    Select,
    Input,
    theme::ColorfulTheme,
    console::Term
};
use async_recursion::async_recursion;


use super::super::ActionSubcommand;

pub(crate) mod function_call_type;
use function_call_type::{
    FunctionCallType,
    CliFunctionCallType
};
pub(crate) mod full_access_type;
use full_access_type::{
    FullAccessType,
    CliFullAccessType
};



#[derive(Debug)]
pub struct AddAccessKeyAction {
    pub public_key: String,
    pub nonce: near_primitives::types::Nonce,
    pub permission: AccessKeyPermission
}

#[derive(Debug, StructOpt)]
pub struct CliAddAccessKeyAction {
    public_key: Option<String>,
    #[structopt(long)]
    nonce: Option<u64>,
    #[structopt(subcommand)]
    permission: Option<CliAccessKeyPermission>,
}

#[derive(Debug, StructOpt)]
pub enum CliAccessKeyPermission {
    FunctionCall(CliFunctionCallType),
    FullAccess(CliFullAccessType),
}

#[derive(Debug, EnumVariantNames)]
pub enum AccessKeyPermission {
    FunctionCall(FunctionCallType),
    FullAccess(FullAccessType),
}

impl From<CliAddAccessKeyAction> for AddAccessKeyAction {
    fn from(item: CliAddAccessKeyAction) -> Self {
        let public_key: near_primitives::types::AccountId = match item.public_key {
            Some(cli_public_key) => near_primitives::types::AccountId::from(cli_public_key),
            None => AddAccessKeyAction::input_public_key()
        };
        let nonce: near_primitives::types::Nonce = match item.nonce {
            Some(cli_nonce) => near_primitives::types::Nonce::from(cli_nonce),
            None => AddAccessKeyAction::input_nonce()
        };
        let permission: AccessKeyPermission = match item.permission {
            Some(cli_permission) => {
                AccessKeyPermission::from(cli_permission)
            },
            None => AccessKeyPermission::choose_permission()
        };
        AddAccessKeyAction {
            public_key,
            nonce,
            permission
        }
    }
}

impl AddAccessKeyAction {
    #[async_recursion(?Send)]
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: String,
        _public_key_string: String,
    ) {
        println!("AddAccessKeyAction process: self:\n       {:?}", &self);
        println!("AddAccessKeyAction process: prepopulated_unsigned_transaction:\n       {:?}", &prepopulated_unsigned_transaction);
        match self.permission {
            AccessKeyPermission::FullAccess(full_access_type) => full_access_type.process(self.nonce, prepopulated_unsigned_transaction, selected_server_url, self.public_key).await,
            AccessKeyPermission::FunctionCall(function_call_type) => function_call_type.process(self.nonce, prepopulated_unsigned_transaction, selected_server_url, self.public_key).await,
        }
    }
    pub fn input_nonce() -> near_primitives::types::Nonce {
            Input::new()
                .with_prompt("Enter the nonce for this access key")
                .interact_text()
                .unwrap()
            
    }
    pub fn input_public_key() -> String {
            Input::new()
                .with_prompt("Enter a public key for this access key")
                .interact_text()
                .unwrap()
    }
}

impl From<CliAccessKeyPermission> for AccessKeyPermission {
    fn from(item: CliAccessKeyPermission) -> Self {
        match item {
            CliAccessKeyPermission::FunctionCall(cli_function_call_type) => {
                let function_call_type: FunctionCallType = FunctionCallType::from(cli_function_call_type);
                AccessKeyPermission::FunctionCall(function_call_type) 
            },
            CliAccessKeyPermission::FullAccess(cli_full_access_type) => {
                let full_access_type: FullAccessType = FullAccessType::from(cli_full_access_type);
                AccessKeyPermission::FullAccess(full_access_type)
            }
        }
    }
}



impl AccessKeyPermission {
    pub fn choose_permission() -> Self {
        let permissions = AccessKeyPermission::VARIANTS;
        let select_permission = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a permission that you want to add to the access key:")
            .items(&permissions)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        match select_permission {
            Some(0) => {
                let allowance: Option<near_primitives::types::Balance> = FunctionCallType::input_allowance();
                let receiver_id: near_primitives::types::AccountId = FunctionCallType::input_receiver_id();
                let method_names: Vec<String> = FunctionCallType::input_method_names();
                let next_action: Box<ActionSubcommand> = Box::new(ActionSubcommand::choose_action_command());
                AccessKeyPermission::FunctionCall(
                    FunctionCallType {
                        allowance,
                        receiver_id,
                        method_names,
                        next_action
                })
            },
            Some(1) => AccessKeyPermission::FullAccess(FullAccessType {
                next_action: Box::new(ActionSubcommand::choose_action_command())
            }),
            _ => unreachable!("Error")
        }
    }
}

