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
use std::num::ParseIntError;
use std::str::FromStr;

use super::{
    ActionSubcommand,
    CliActionSubcommand,
    CliActionSkipSubcommand
};


#[derive(Debug, StructOpt)]
pub struct TransferNEARTokens {
    #[structopt(long)]
    pub amount: NearBalance,
    #[structopt(subcommand)]
    pub next_action: Box<ActionSubcommand>
}

#[derive(Debug, StructOpt)]
pub struct CliTransferNEARTokens {
    #[structopt(long)]
    amount: Option<NearBalance>,
    #[structopt(subcommand)]
    next_action: Option<CliActionSkipSubcommand> 
    // next_action: Option<Box<CliActionSubcommand>>  // CliActionSkipSubcommand
}

impl NearBalance {
    pub fn input_amount() -> Self {
        let input: String = Input::new()
            .with_prompt("How many NEAR Tokens do you want to transfer? (example: 10NEAR)")
            .interact_text()
            .unwrap();
        NearBalance::from_str(&input).unwrap()
    }
}

#[derive(Debug)]
pub struct NearBalance (u128);

impl FromStr for NearBalance {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number: u128 = s.parse().unwrap_or_else(|ParseIntError| -> u128 {
            match s.contains("NEAR") {
                true => {
                    let num:u128 = s.trim_matches(char::is_alphabetic)
                        .parse()
                        .unwrap();
                    num * 10u128.pow(24)
                },
                _ => 0
            }
        });
        Ok(NearBalance(number))
    }
}

impl From<CliTransferNEARTokens> for TransferNEARTokens {
    fn from(item: CliTransferNEARTokens) -> Self {
        let amount: NearBalance = match item.amount {
            Some(cli_amount) => cli_amount,
            None => NearBalance::input_amount()
        };
        let next_action: Box<ActionSubcommand> = match item.next_action {
            Some(cli_skip_action) => {
                Box::new(ActionSubcommand::from(cli_skip_action))
            },
            None => Box::new(ActionSubcommand::choose_action_command()) 
        };
        TransferNEARTokens {
            amount,
            next_action
        }
    }
}

