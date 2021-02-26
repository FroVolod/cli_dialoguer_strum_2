use structopt::StructOpt;

pub(crate) mod common;
pub(crate) mod utils_subcommand;
mod consts;
mod command;
use command::{
    CliCommand,
    ArgsCommand,
};



#[derive(Debug)]
struct Args {
    subcommand: ArgsCommand,
}

#[derive(Debug, Default, StructOpt)]
struct CliArgs {
    #[structopt(subcommand)]
    subcommand: Option<CliCommand>,
}

impl From<CliArgs> for Args {
    fn from(item: CliArgs) -> Self {
        let subcommand = match item.subcommand {
            Some(cli_subcommand) => ArgsCommand::from(cli_subcommand),
            None => ArgsCommand::choose_command(),
        };
        Self {
            subcommand,
        }
    }
}

impl Args {
    async fn process(self) -> String {
        println!("===========    Args process   ===========\n    {:?}", &self.subcommand);
        let transaction_command = &self.subcommand;
        println!("{:?}", &transaction_command);
        match self.subcommand {
            ArgsCommand::ConstructTransactionCommand(mode) => {
                let unsigned_transaction = near_primitives::transaction::Transaction {
                    signer_id: "".to_string(),
                    public_key: near_crypto::PublicKey::empty(near_crypto::KeyType::ED25519),
                    nonce: 0,
                    receiver_id: "".to_string(),
                    block_hash: Default::default(),
                    actions: vec![],
                };
                println!("!!!!!!!!!!!!  {:?}", &mode.process(unsigned_transaction).await);
            },
            _ => unreachable!("Error") 
        };
        "Ok".to_string()
    }
}


fn main() {
    let cli = CliArgs::from_args();
    let args = Args::from(cli);
    println!("args {:#?}", args);

    actix::System::builder()
    .build()
    .block_on(async move { args.process().await });
}
