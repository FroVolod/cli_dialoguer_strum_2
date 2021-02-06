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

mod sign_private_key;
use sign_private_key::SignPrivateKey;




#[derive(Debug, StructOpt)]
pub struct Skip {
    #[structopt(subcommand)]
    sign_option: SignTransaction
}
#[derive(Debug, EnumVariantNames, StructOpt)]
#[strum(serialize_all = "kebab_case")]
pub enum SignTransaction {
    SignPrivateKey(SignPrivateKey),
    SignAlternative
}

impl Default for Skip {
    fn default() -> Self {
        Self{sign_option: SignTransaction::SignPrivateKey(SignPrivateKey::default())}
    }
}

impl Skip {
    pub fn choose_sign_option() -> Self {
        let sign_options = SignTransaction::VARIANTS;
        let select_sign_options = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to sign the transaction?")
            .items(&sign_options)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        match select_sign_options {
            Some(0) => Self{sign_option: SignTransaction::SignPrivateKey(SignPrivateKey::signer_keys())},
            Some(1) => Self{sign_option: SignTransaction::SignAlternative},
            _ => Self{sign_option: SignTransaction::SignPrivateKey(SignPrivateKey::signer_keys())}
        }
    }
}
