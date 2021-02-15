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

#[derive(Debug, Display, EnumVariantNames)]
pub enum SelectServer {
    Testnet(Server),
    Mainnet(Server),
    Betanet(Server),
    Custom(Server),
}

#[derive(Debug, Display, EnumVariantNames, StructOpt)]
pub enum CliSelectServer {
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
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) {
        println!("SelectServer process:  self:         {:?}", &self);
        println!("SelectServer process:  prepopulated_unsigned_transaction:         {:?}", prepopulated_unsigned_transaction);
        match self {
            SelectServer::Testnet(server) => {
                println!("server url:   {:?}", &server.url);
                server.process(prepopulated_unsigned_transaction).await;
            },
            SelectServer::Mainnet(server) => {},
            SelectServer::Betanet(server) => {},
            SelectServer::Custom(server) => {},
            _ => unreachable!("Error")
        }
    }

    pub fn select_server() -> Self {
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
            Some(3) => SelectServer::Custom(Server{
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
