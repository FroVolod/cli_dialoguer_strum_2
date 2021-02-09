use structopt::StructOpt;
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

use super::{
    ActionSubcommand,
    CliActionSubcommand,
    CliActionSkipSubcommand
};


#[derive(Debug, StructOpt)]
pub struct CreateAccount {
    pub account_id: String,
    #[structopt(subcommand)]
    pub next_action: Box<ActionSubcommand>
}

#[derive(Debug, StructOpt)]
pub struct CliCreateAccount {
    account_id: Option<String>,
    #[structopt(subcommand)]
    next_action: Option<CliActionSkipSubcommand>
}

impl From<CliCreateAccount> for CreateAccount {
    fn from(item: CliCreateAccount) -> Self {
        let account_id: String = match item.account_id {
            Some(cli_account_id) => cli_account_id,
            None => CreateAccount::input_account_id()
        };
        let next_action: Box<ActionSubcommand> = match item.next_action {
            Some(cli_skip_action) => {
                Box::new(ActionSubcommand::from(cli_skip_action))
            },
            None => Box::new(ActionSubcommand::choose_action_command()) 
        };
        CreateAccount {
            account_id,
            next_action
        }
    }
}

impl CreateAccount {
    pub fn input_account_id() -> String {
        Input::new()
            .with_prompt("Enter the account ID to create new account")
            .interact_text()
            .unwrap()
    }
}
