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
pub struct Receiver {

}

#[derive(Debug, StructOpt)]
pub struct CliReceiver {

}
