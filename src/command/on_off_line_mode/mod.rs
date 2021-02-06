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
