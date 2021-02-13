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


#[derive(Debug)]
pub struct DeleteAccount {
    pub beneficiary_id: String,
    pub next_action: Box<ActionSubcommand>
}

#[derive(Debug, StructOpt)]
pub struct CliDeleteAccount {
    beneficiary_id: Option<String>,
    #[structopt(subcommand)]
    next_action: Option<CliActionSkipSubcommand>
}

impl From<CliDeleteAccount> for DeleteAccount {
    fn from(item: CliDeleteAccount) -> Self {
        let beneficiary_id: String = match item.beneficiary_id {
            Some(cli_account_id) => cli_account_id,
            None => DeleteAccount::input_beneficiary_id()
        };
        let next_action: Box<ActionSubcommand> = match item.next_action {
            Some(cli_skip_action) => {
                Box::new(ActionSubcommand::from(cli_skip_action))
            },
            None => Box::new(ActionSubcommand::choose_action_command()) 
        };
        DeleteAccount {
            beneficiary_id,
            next_action
        }
    }
}

impl DeleteAccount {
    pub fn input_beneficiary_id() -> String {
        Input::new()
            .with_prompt("Enter the beneficiary ID to delete this account ID")
            .interact_text()
            .unwrap()
    }
}
