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

use crate::consts;
use consts::{
    TESTNET_API_SERVER_URL,
    MAINNET_API_SERVER_URL,
    BETANET_API_SERVER_URL,
};
mod server;
use server::{
    Server,
    SendFrom,
    CliServer,
    CliCustomServer,
    // ActionSubcommand
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
                Mode::Offline(OfflineArgs {})
            }
            _ => unreachable!("Error")
        }
    }
}

#[derive(Debug, Display, EnumVariantNames, StructOpt)]
pub enum CliMode {
    Online(CliOnlineArgs),
    Offline(OfflineArgs),
}

impl From<CliMode> for Mode {
    fn from(item: CliMode) -> Self {
        match item {
            CliMode::Online(cli_online_args) => {
                let selected_server = OnlineArgs::from(cli_online_args);
                Mode::Online(selected_server)
            }
            CliMode::Offline(OfflineArgs{}) => Mode::Offline(OfflineArgs{})
        }
    }
}

#[derive(Debug, StructOpt)]
struct OfflineArgs {
    // #[structopt(long)]
    // block_height: u64,
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

#[derive(Debug, Display, EnumVariantNames, StructOpt)]
enum SelectServer {
    Testnet(Server),
    Mainnet(Server),
    Betanet(Server),
    Custom(Server),
}

#[derive(Debug, Display, EnumVariantNames, StructOpt)]
enum CliSelectServer {
    Testnet(CliServer),
    Mainnet(CliServer),
    Betanet(CliServer),
    Custom(CliCustomServer),
}

impl From<CliSelectServer> for SelectServer {
    fn from(item: CliSelectServer) -> Self {
        match item {
            CliSelectServer::Testnet(cli_server) => {
                Self::Testnet(cli_server.into_server(TESTNET_API_SERVER_URL.to_string()))
            },
            CliSelectServer::Mainnet(cli_server) => {
                Self::Mainnet(cli_server.into_server(MAINNET_API_SERVER_URL.to_string()))
            },
            CliSelectServer::Betanet(cli_server) => {
                Self::Betanet(cli_server.into_server(BETANET_API_SERVER_URL.to_string()))
            },
            CliSelectServer::Custom(cli_custom_server) => {
                Self::Custom(cli_custom_server.into_server())
            },
        }
    }
}



impl SelectServer {
    fn select_server() -> Self {
        println!("Works select server!");
        let servers= SelectServer::VARIANTS;
        let select_server = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select NEAR protocol RPC server:")
            .items(&servers)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        let send_from = SendFrom::send_from();
        match select_server {
            Some(0) => SelectServer::Testnet(Server{
                            url: TESTNET_API_SERVER_URL.to_string(),
                            send_from
                        }),
            Some(1) => SelectServer::Mainnet(Server{
                            url: MAINNET_API_SERVER_URL.to_string(),
                            send_from
                        }),
            Some(2) => SelectServer::Betanet(Server{
                            url: BETANET_API_SERVER_URL.to_string(),
                            send_from
                        }),
            Some(4) => SelectServer::Custom(Server{
                            url: {
                                Input::new()
                                .with_prompt("What is the RPC endpoint?")
                                .interact_text()
                                .unwrap()
                            },
                            send_from
            }),
            _ => unreachable!("Error")
        }
    }
}

