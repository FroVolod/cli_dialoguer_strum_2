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



#[derive(Debug, StructOpt)]
pub struct CliSender {
    pub account_id: String,
    // #[structopt(subcommand)]
    // auth: Authentication
}

#[derive(Debug, StructOpt)]
pub enum CliAuthentication {
    private_key,
    alternativ
}

#[derive(Debug, StructOpt)]
pub struct Sender {
    pub account_id: String,
    // #[structopt(subcommand)]
    // auth: Authentication
}

#[derive(Debug, StructOpt)]
pub enum Authentication {
    private_key,
    alternativ
}

impl From<CliSender> for Sender {
    fn from(item: CliSender) -> Self {
        Sender {
            account_id: "vova".to_string()
        }
    }
}
