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

use super::ActionSubcommand;


#[derive(Debug, StructOpt)]
pub struct TransferNEARTokens {
    #[structopt(long)]
    amount: NearBalance,
    #[structopt(subcommand)]
    next_action: Box<ActionSubcommand>
}

impl TransferNEARTokens {
    pub fn input_amount() -> Self {
        let input: String = Input::new()
            .with_prompt("How many NEAR Tokens do you want to transfer? (example: 10NEAR)")
            .interact_text()
            .unwrap();
        let amount = NearBalance::from_str(&input).unwrap();

        Self {amount, next_action: Box::from(ActionSubcommand::choose_action_command())}
    }
}

#[derive(Debug)]
struct NearBalance (u128);

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

