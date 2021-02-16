use structopt::StructOpt;
use strum_macros::{
    EnumVariantNames,
};
use strum::VariantNames;
use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};

mod on_off_line_mode;
use on_off_line_mode::{CliOnOffLineMode, Mode, OnOffLineMode};


#[derive(Debug, EnumVariantNames, StructOpt)]
#[strum(serialize_all = "kebab_case")]
pub enum CliCommand {
    ConstructTransactionCommand(CliOnOffLineMode),
    Utils,
}

#[derive(Debug, EnumVariantNames)]
pub enum ArgsCommand {
    ConstructTransactionCommand(OnOffLineMode),
    Utils,
}

impl From<CliCommand> for ArgsCommand {
    fn from(item: CliCommand) -> Self {
        match item {
            CliCommand::ConstructTransactionCommand(cli_onoffline_mode) => {
                let onoffline_mode = OnOffLineMode::from(cli_onoffline_mode);
                ArgsCommand::ConstructTransactionCommand(onoffline_mode)
            }
            CliCommand::Utils => ArgsCommand::Utils,
        }
    }
}

impl ArgsCommand {
    pub fn choose_command() -> Self {
        let commands= Self::VARIANTS;
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your action")
            .items(&commands)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(0) => {
                println!("============== {:?}", commands[0]);
                Self::ConstructTransactionCommand(OnOffLineMode{mode: Mode::choose_mode()})
            },
            Some(1) => {
                println!("============== {:?}", commands[1]);
                Self::Utils
            },
            _ => unreachable!("Error")
        }
    }
}

