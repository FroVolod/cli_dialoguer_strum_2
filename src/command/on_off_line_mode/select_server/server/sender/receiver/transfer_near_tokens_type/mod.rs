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
use async_recursion::async_recursion;

use super::{
    ActionSubcommand,
    CliActionSubcommand,
    CliActionSkipSubcommand
};


#[derive(Debug)]
pub struct TransferNEARTokensAction {
    pub amount: NearBalance,
    pub next_action: Box<ActionSubcommand>
}

impl TransferNEARTokensAction {
    #[async_recursion(?Send)]
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: String,
    ) {
        println!("TransferNEARTokens process: self:       {:?}", &self);
        println!("TransferNEARTokens process: prepopulated_unsigned_transaction:       {:?}", &prepopulated_unsigned_transaction);
        let amount = match self.amount {
            NearBalance(num) => num,
            _ => unreachable!("Error")
        };
        let action = near_primitives::transaction::Action::Transfer(
            near_primitives::transaction::TransferAction {
                deposit: amount,
            },
        );
        let mut actions= prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            actions,
            .. prepopulated_unsigned_transaction
        };
        println!("unsigned_transaction.    {:?}", &unsigned_transaction);
        match *self.next_action {
            ActionSubcommand::TransferNEARTokens(args_transfer) => args_transfer.process(unsigned_transaction, selected_server_url).await,
            // ActionSubcommand::CallFunction(args_function) => {},
            // ActionSubcommand::StakeNEARTokens(args_stake) => {},
            // ActionSubcommand::CreateAccount(args_create_account) => {},
            // ActionSubcommand::DeleteAccount(args_delete_account) => {},
            // ActionSubcommand::AddAccessKey(args_add_access_key) => {},
            // ActionSubcommand::DeleteAccessKey(args_delete_access_key) => {},
            ActionSubcommand::Skip(args_skip) => args_skip.process(unsigned_transaction, selected_server_url).await,
            _ => unreachable!("Error")
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct CliTransferNEARTokensAction {
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

impl From<CliTransferNEARTokensAction> for TransferNEARTokensAction {
    fn from(item: CliTransferNEARTokensAction) -> Self {
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
        TransferNEARTokensAction {
            amount,
            next_action
        }
    }
}

