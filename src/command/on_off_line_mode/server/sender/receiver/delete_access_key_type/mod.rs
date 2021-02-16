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
pub struct DeleteAccessKeyAction {
    pub access_key: String,
    pub next_action: Box<ActionSubcommand>
}

#[derive(Debug, StructOpt)]
pub struct CliDeleteAccessKeyAction {
    #[structopt(long)]
    access_key: Option<String>,
    #[structopt(subcommand)]
    next_action: Option<CliActionSkipSubcommand>
}

impl From<CliDeleteAccessKeyAction> for DeleteAccessKeyAction {
    fn from(item: CliDeleteAccessKeyAction) -> Self {
        let access_key: String = match item.access_key {
            Some(cli_access_key) => cli_access_key,
            None => DeleteAccessKeyAction::input_access_key()
        };
        let next_action: Box<ActionSubcommand> = match item.next_action {
            Some(cli_skip_action) => {
                Box::new(ActionSubcommand::from(cli_skip_action))
            },
            None => Box::new(ActionSubcommand::choose_action_command()) 
        };
        DeleteAccessKeyAction {
            access_key,
            next_action
        }
    }
}

impl DeleteAccessKeyAction {
    pub fn input_access_key() -> String {
        Input::new()
            .with_prompt("Enter the access key to remove it")
            .interact_text()
            .unwrap()
    }
}
