use structopt::StructOpt;
use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};

mod sign_private_key;
use sign_private_key::{
    SignPrivateKey,
    CliSignPrivateKey
};
mod sign_alternative;
use sign_alternative::{
    SignAlternative,
    CliSignAlternative
};


#[derive(Debug)]
pub struct SkipAction {
    pub sign_option: SignTransaction
}

#[derive(Debug)]
pub enum SignTransaction {
    SignPrivateKey(SignPrivateKey),
    SignAlternative(SignAlternative)
}

#[derive(Debug, StructOpt)]
pub struct CliSkipAction {
    #[structopt(subcommand)]
    sign_option: Option<CliSignTransaction> 
}

#[derive(Debug, StructOpt)]
pub enum CliSignTransaction {
    SignPrivateKey(CliSignPrivateKey),
    SignAlternative(CliSignAlternative)
}

impl SkipAction {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: String,
    ) {
        println!("Skip process:\n       {:?}", &self);
        println!("Skip process: prepopulated_unsigned_transaction:\n       {:?}", &prepopulated_unsigned_transaction);
        self.sign_option.process(prepopulated_unsigned_transaction, selected_server_url).await;
    }
}

impl SignTransaction {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: String,
    ) {
        println!("SignTransaction process: self:       {:?}", &self);
        println!("SignTransaction process: prepopulated_unsigned_transaction:       {:?}", &prepopulated_unsigned_transaction);
        match self {
            SignTransaction::SignPrivateKey(keys) => keys.process(prepopulated_unsigned_transaction, selected_server_url).await,
            SignTransaction::SignAlternative(chain) => chain.process(prepopulated_unsigned_transaction, selected_server_url)
        }
    }
    pub fn choose_sign_option() -> Self {
        let sign_options = vec![
            "Yes, I want to sign the transaction with my private key",
            "No, I want to construct the transaction and sign it somewhere else",
        ];
        let select_sign_options = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to sign the transaction?")
            .items(&sign_options)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        match select_sign_options {
            Some(0) => SignTransaction::SignPrivateKey(SignPrivateKey{
                signer_public_key: SignPrivateKey::signer_public_key(),
                signer_secret_key: SignPrivateKey::signer_secret_key()
            }),
            Some(1) => SignTransaction::SignAlternative(SignAlternative{key_chain: SignAlternative::input_key_chain()}),
            _ => SignTransaction::SignPrivateKey(SignPrivateKey{
                signer_public_key: SignPrivateKey::signer_public_key(),
                signer_secret_key: SignPrivateKey::signer_secret_key()
            })
        }
    }
}

impl From<CliSkipAction> for SkipAction {
    fn from(item: CliSkipAction) -> Self {
        let sign_option: SignTransaction = match item.sign_option {
            Some(cli_sign_transaction) => SignTransaction::from(cli_sign_transaction),
            None => SignTransaction::choose_sign_option()
        };
        SkipAction {sign_option}
    }
}

impl From<CliSignTransaction> for SignTransaction {
    fn from(item: CliSignTransaction) -> Self {
        match item {
            CliSignTransaction::SignPrivateKey(cli_private_key) => {
                let privat_key = SignPrivateKey::from(cli_private_key);
                SignTransaction::SignPrivateKey(privat_key)
            },
            CliSignTransaction::SignAlternative(cli_key_chain) => {
                let key_chain = SignAlternative::from(cli_key_chain);
                SignTransaction::SignAlternative(key_chain)
            } 
        }
    }
}
