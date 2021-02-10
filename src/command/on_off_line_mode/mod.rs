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

mod select_server;
use select_server::{
    SelectServer,
    CliSelectServer
};


#[derive(Debug, StructOpt)]
pub struct CliOnOffLineMode {
    #[structopt(subcommand)]
    pub mode: Option<CliMode>, // selected_server: SelectServer
}

#[derive(Debug, StructOpt)]
pub struct OnOffLineMode {
    #[structopt(subcommand)]
    pub mode: Mode,
}

impl From<CliOnOffLineMode> for OnOffLineMode {
    fn from(item: CliOnOffLineMode) -> Self {
        let mode = match item.mode {
            Some(cli_mode) => Mode::from(cli_mode),
            None => Mode::choose_mode()
        };
        Self { mode }
    }
}

#[derive(Debug, Display, EnumVariantNames, StructOpt)]
pub enum Mode {
    Online(OnlineArgs),
    Offline(OfflineArgs),
}

impl Mode {
    pub fn choose_mode() -> Self {
        let choose_mode= vec![
            "Yes, I keep it simple",
            "No, I want to work in no-network (air-gapped) environment"
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
        match select_mode {
            Some(0) => {
                println!("============== {:?}", choose_mode[0]);
                let selected_server = SelectServer::select_server();
                Mode::Online(OnlineArgs {
                        selected_server
                    }) 
            },
            Some(1) => {
                println!("============== {:?}", choose_mode[1]);
                let nonce: u64 = OfflineArgs::input_nonce();
                let block_height: String = OfflineArgs::input_block_height();
                let selected_server = SelectServer::select_server();
                Mode::Offline(OfflineArgs {
                    nonce,
                    block_height,
                    selected_server
                })
            }
            _ => unreachable!("Error")
        }
    }
}

#[derive(Debug, StructOpt)]
struct OfflineArgs {
    #[structopt(long)]
    nonce: u64,
    #[structopt(long)]
    block_height: String,
    #[structopt(subcommand)]
    selected_server: SelectServer
}

#[derive(Debug, StructOpt)]
struct CliOfflineArgs {
    #[structopt(long)]
    nonce: Option<u64>,
    #[structopt(long)]
    block_height: Option<String>,
    #[structopt(subcommand)]
    selected_server: Option<CliSelectServer> 
}

#[derive(Debug, StructOpt)]
struct OnlineArgs {
    #[structopt(subcommand)]
    selected_server: SelectServer
}

#[derive(Debug, Default, StructOpt)]
struct CliOnlineArgs {
    #[structopt(subcommand)]
    selected_server: Option<CliSelectServer> 
}

impl From<CliOnlineArgs> for OnlineArgs {
    fn from(item: CliOnlineArgs) -> Self {
        let selected_server = match item.selected_server {
            Some(cli_selected_server) => SelectServer::from(cli_selected_server),
            None => SelectServer::select_server()
        };
        OnlineArgs {selected_server}
    }
}

impl From<CliOfflineArgs> for OfflineArgs {
    fn from(item: CliOfflineArgs) -> Self {
        let nonce: u64 = match item.nonce {
            Some(cli_nonce) => cli_nonce,
            None => OfflineArgs::input_nonce()
        };
        let block_height: String = match item.block_height {
            Some(cli_block_height) => cli_block_height,
            None => OfflineArgs::input_block_height()
        };
        let selected_server = match item.selected_server {
            Some(cli_selected_server) => SelectServer::from(cli_selected_server),
            None => SelectServer::select_server()
        };
        OfflineArgs {
            nonce,
            block_height,
            selected_server
        }
    }
}

impl OfflineArgs {
    fn input_nonce() -> u64 {
        Input::new()
            .with_prompt("Enter transaction nonce (query the access key information with
                `near-cli utils view-access-key frol4.testnet ed25519:...` incremented by 1)")
            .interact_text()
            .unwrap()
    }
    fn input_block_height() -> String {
        Input::new()
            .with_prompt("Enter recent block hash:")
            .interact_text()
            .unwrap()
    }
}

#[derive(Debug, Display, EnumVariantNames, StructOpt)]
pub enum CliMode {
    Online(CliOnlineArgs),
    Offline(CliOfflineArgs),
}

impl From<CliMode> for Mode {
    fn from(item: CliMode) -> Self {
        match item {
            CliMode::Online(cli_online_args) => {
                let online_args: OnlineArgs = OnlineArgs::from(cli_online_args);
                Mode::Online(online_args)
            },
            CliMode::Offline(cli_offline_args) => {
                let offline_args:OfflineArgs = OfflineArgs::from(cli_offline_args);
                Mode::Offline(offline_args)
            }
        }
    }
}
