use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use std::io::Result;
use std::str::FromStr;
use structopt::StructOpt;

use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames};

mod consts;
use consts::{BETANET_API_SERVER_URL, MAINNET_API_SERVER_URL, TESTNET_API_SERVER_URL};

#[derive(Debug)]
struct Args {
    name: String,
    subcommand: ArgsCommand,
}

#[derive(Debug, Default, StructOpt)]
struct CliArgs {
    #[structopt(long)]
    name: Option<String>,
    #[structopt(subcommand)]
    subcommand: Option<CliCommand>,
}

#[derive(Debug, Display, EnumVariantNames, StructOpt)]
#[strum(serialize_all = "kebab_case")]
enum CliCommand {
    ConstructTransactionCommand(CliOnOffLineMode),
    Utils,
}

#[derive(Debug, Display, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum ArgsCommand {
    ConstructTransactionCommand(OnOffLineMode),
    Utils,
}

impl From<CliCommand> for ArgsCommand {
    fn from(item: CliCommand) -> Self {
        match item {
            CliCommand::ConstructTransactionCommand(cli_onoffline_mode) => {
                let onoffline_mode = match cli_onoffline_mode.mode {
                    Some(mode) => OnOffLineMode { mode },
                    None => OnOffLineMode::choose_command(),
                };
                ArgsCommand::ConstructTransactionCommand(onoffline_mode)
            }
            CliCommand::Utils => ArgsCommand::Utils,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct CliOnOffLineMode {
    #[structopt(subcommand)]
    mode: Option<Mode>, // selected_server: SelectServer
}

#[derive(Debug)]
pub struct OnOffLineMode {
    mode: Mode, // selected_server: SelectServer
}

#[derive(Debug, Display, EnumVariantNames, StructOpt)]
enum Mode {
    Online(OnlineArgs),
    Offline(OfflineArgs),
}

#[derive(Debug, StructOpt)]
struct OfflineArgs {
    #[structopt(long)]
    online: bool,
    sender: String,
    // #[structopt(long)]
    // block_height: u64,
}

#[derive(Debug, Default, StructOpt)]
struct OnlineArgs {
    #[structopt(long)]
    online: bool,
    #[structopt(long)]
    sender: String,
    #[structopt(long)]
    receiver: String,
    #[structopt(subcommand)]
    selected_server: SelectServer,
}

#[derive(Debug, Display, EnumString, EnumVariantNames, StructOpt)]
enum SelectServer {
    Testnet(Server),
    Mainnet(Server),
    Betanet(Server),
    Custom(Server),
}

#[derive(Debug, Default, StructOpt)]
struct Server {
    url: String,
    #[structopt(subcommand)]
    transaction_subcommand: TransactionSubcommand,
}

impl Default for SelectServer {
    fn default() -> Self {
        SelectServer::Testnet(Server {
            url: TESTNET_API_SERVER_URL.to_string(),
            transaction_subcommand: TransactionSubcommand::default(),
        })
    }
}

#[derive(Debug, EnumString, EnumVariantNames, StructOpt)]
enum TransactionSubcommand {
    TransferNEARTokens,
    CallFunction,
    StakeNEARTokens,
    CreateAccount,
    DeleteAccount,
    AddAccessKey,
    DeteteAccessKey,
}

impl Default for TransactionSubcommand {
    fn default() -> Self {
        TransactionSubcommand::TransferNEARTokens
    }
}

pub trait ChooseCommand {
    fn choose_command() -> Self;
}

impl ChooseCommand for ArgsCommand {
    fn choose_command() -> Self {
        let commands = Self::VARIANTS;
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your action")
            .items(&commands)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(0) => {
                println!("============== {:?}", commands[0]);
                Self::ConstructTransactionCommand(OnOffLineMode::choose_command())
            }
            Some(1) => {
                println!("============== {:?}", commands[1]);
                Self::Utils
            }
            _ => unreachable!("Error"),
        }
    }
}

impl ChooseCommand for OnOffLineMode {
    fn choose_command() -> Self {
        let choose_mode = vec![
            "Yes, I keep it simple",
            "No, I want to work in no-network (air-gapped) environment",
        ];
        println!("\n");
        let select_mode = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(
                "To construct a transaction you will need to provide information about sender (signer) and receiver accounts, and actions that needs to be performed.
                 \nDo you want to derive some information required for transaction construction automatically querying it online?"
            )
            .items(&choose_mode)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        let servers = SelectServer::VARIANTS;
        let select_server = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select NEAR protocol RPC server:")
            .items(&servers)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        let custom_api_server_url: String = "".to_string();

        let transaction_subcommands = TransactionSubcommand::VARIANTS;
        let select_transaction_subcommand = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an action that you want to add to the transaction:")
            .items(&transaction_subcommands)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        let transaction_subcommand = match select_transaction_subcommand {
            Some(num) => TransactionSubcommand::from_str(transaction_subcommands[num]).unwrap(),
            _ => TransactionSubcommand::default(),
        };
        let selected_server: SelectServer = match select_server {
            // Some(num) => SelectServer::from_str(servers[num]).unwrap() ,
            Some(0) => SelectServer::Testnet(Server {
                url: TESTNET_API_SERVER_URL.to_string(),
                transaction_subcommand,
            }),
            Some(1) => SelectServer::Mainnet(Server {
                url: MAINNET_API_SERVER_URL.to_string(),
                transaction_subcommand,
            }),
            Some(2) => SelectServer::Betanet(Server {
                url: BETANET_API_SERVER_URL.to_string(),
                transaction_subcommand,
            }),
            Some(3) => SelectServer::Custom(Server {
                url: custom_api_server_url,
                transaction_subcommand,
            }),
            _ => SelectServer::Betanet(Server {
                url: BETANET_API_SERVER_URL.to_string(),
                transaction_subcommand,
            }),
        };
        let sender: String = Input::new()
            .with_prompt("What is the account ID of the sender?")
            .interact_text()
            .unwrap();
        let receiver: String = Input::new()
            .with_prompt("What is the account ID of the receiver?")
            .interact_text()
            .unwrap();

        match select_mode {
            Some(0) => {
                println!("============== {:?}", choose_mode[0]);
                Self {
                    mode: Mode::Online(OnlineArgs {
                        sender,
                        receiver,
                        online: true,
                        selected_server,
                    }),
                }
            }
            Some(1) => {
                println!("============== {:?}", choose_mode[1]);
                Self {
                    mode: Mode::Offline(OfflineArgs {
                        sender,
                        online: false,
                    }),
                }
            }
            _ => unreachable!("Error"),
        }
    }
}

impl From<CliArgs> for Args {
    fn from(item: CliArgs) -> Self {
        //let subcommand = if let Some(cli_subcommand) = item.subcommand {
        //    ArgsCommand::from(cli_subcommand)
        //} else {
        //    ArgsCommand::choose_command()
        //};
        let subcommand = match item.subcommand {
            Some(cli_subcommand) => ArgsCommand::from(cli_subcommand),
            None => ArgsCommand::choose_command(),
        };
        Self {
            name: item.name.unwrap_or_default(),
            subcommand,
        }
    }
}

fn main() {
    let cli = CliArgs::from_args();
    println!("cli: {:?}", cli);

    let args = Args::from(cli);
    println!("args {:#?}", args);

    /*    let args = Args {
            name: "q".to_string(),
            subcommand: CliArgSubcommand::ConstructTransactionSubcommand(
                ConstructTransactionArgs {
                    //
                }
            )
        };

        let construct_transaction_args = ConstructTransactionArgs {
            //
        };

        run(construct_transaction_args);
    */

}
