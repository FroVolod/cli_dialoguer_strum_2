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
pub struct AddAccessKey {
    // pub access_key: String,
    #[structopt(subcommand)]
    pub next_action: Box<ActionSubcommand>
}

#[derive(Debug, StructOpt)]
pub struct CliAddAccessKey {
    // access_key: Option<String>,
    #[structopt(subcommand)]
    next_action: Option<CliActionSkipSubcommand>
}

impl From<CliAddAccessKey> for AddAccessKey {
    fn from(item: CliAddAccessKey) -> Self {
        // let access_key: String = match item.access_key {
        //     Some(cli_access_key) => cli_access_key,
        //     None => DeleteAccessKey::input_access_key()
        // };
        let next_action: Box<ActionSubcommand> = match item.next_action {
            Some(cli_skip_action) => {
                Box::new(ActionSubcommand::from(cli_skip_action))
            },
            None => Box::new(ActionSubcommand::choose_action_command()) 
        };
        AddAccessKey {
            // access_key,
            next_action
        }
    }
}

// impl AddAccessKey {
//     pub fn input_access_key() -> String {
//         Input::new()
//             .with_prompt("Enter the access key to remove it")
//             .interact_text()
//             .unwrap()
//     }
// }
