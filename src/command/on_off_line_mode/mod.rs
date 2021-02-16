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
use near_primitives::{borsh::BorshSerialize, hash::CryptoHash};


mod select_server;
use select_server::{
    SelectServer,
    CliSelectServer
};
mod server;
use server::{
    Server,
    SendFrom,
    CliSendFrom,
    CliServer,
    CliCustomServer,
    // ActionSubcommand
};



#[derive(Debug, StructOpt)]
pub struct CliOnOffLineMode {
    #[structopt(subcommand)]
    pub mode: Option<CliMode>, // selected_server: SelectServer
}

#[derive(Debug)]
pub struct OnOffLineMode {
    pub mode: Mode,
}

impl OnOffLineMode {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) {
        println!("OnOffLineMode - prepopulated_unsigned_transaction :      {:?}", prepopulated_unsigned_transaction);
        match self.mode {
            Mode::Online(online_args) => {
                println!("Online args:  {:?}", &online_args);                
                // реализовать online запрос для block_hash и nonce
                let nonce = online_args.nonce.clone();
                let block_hash = online_args.block_hash.clone();
                let unsigned_transaction = near_primitives::transaction::Transaction {                    
                    block_hash,
                    nonce,
                    .. prepopulated_unsigned_transaction
                };
                online_args.process(unsigned_transaction).await
            },
            Mode::Offline(offline_args) => {
                println!("Offline args:  {:?}", &offline_args);
                let nonce = offline_args.nonce.clone();
                let block_hash = offline_args.block_hash.clone();
                let unsigned_transaction = near_primitives::transaction::Transaction {                    
                    block_hash,
                    nonce,
                    .. prepopulated_unsigned_transaction
                };
                // let selected_server = offline_args.selected_server;
                // println!("server:   {:?}", &selected_server);
                offline_args.process(unsigned_transaction).await
            },
            _ => unreachable!("Error")
        }
    }
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

#[derive(Debug, Display, EnumVariantNames)]
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
                let nonce: u64 = 55;
                let block_hash: CryptoHash = crate::common::BlobAsBase58String::<near_primitives::hash::CryptoHash>::from_str("C2tEMtPNkJ2Gh8tw24Bs7a4sXbd8APL988d3fqic1opc").unwrap().into_inner();        
                let selected_server = SelectServer::select_server();
                Mode::Online(OnlineArgs {
                        nonce,
                        block_hash,
                        selected_server
                    }) 
            },
            Some(1) => {
                println!("============== {:?}", choose_mode[1]);
                let nonce: u64 = OfflineArgs::input_nonce();
                let block_hash = OfflineArgs::input_block_hash();
                let send_from: SendFrom = SendFrom::send_from();
                // let selected_server = SelectServer::select_server();
                Mode::Offline(OfflineArgs {
                    nonce,
                    block_hash,
                    send_from
                    // selected_server
                })
            }
            _ => unreachable!("Error")
        }
    }
}

#[derive(Debug)]
pub struct OfflineArgs {
    nonce: u64,
    block_hash: near_primitives::hash::CryptoHash,
    send_from: SendFrom
    // selected_server: SelectServer
}

// impl  OfflineArgs {
//     pub async fn process(
//         self,
//         prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
//     ) {
//         println!("OfflineArgs process:        {:?}", prepopulated_unsigned_transaction);
//         let selected_server_url = "".to_string(); 
//         self.send_from.process(prepopulated_unsigned_transaction, selected_server_url).await;
//         // self.selected_server.process(prepopulated_unsigned_transaction).await;
//     }
// }

#[derive(Debug, StructOpt)]
pub struct CliOfflineArgs {
    #[structopt(long)]
    nonce: Option<u64>,
    #[structopt(long)]
    block_hash: Option<crate::common::BlobAsBase58String<near_primitives::hash::CryptoHash>>,
    #[structopt(subcommand)]
    pub send_from: Option<CliSendFrom>
    // selected_server: Option<CliSelectServer> 
}

#[derive(Debug)]
pub struct OnlineArgs {
    nonce: u64,
    block_hash: near_primitives::hash::CryptoHash,
    selected_server: SelectServer
}

#[derive(Debug, Default, StructOpt)]
pub struct CliOnlineArgs {
    nonce: Option<u64>,
    block_hash: Option<crate::common::BlobAsBase58String<near_primitives::hash::CryptoHash>>,
    #[structopt(subcommand)]
    selected_server: Option<CliSelectServer> 
}

impl From<CliOnlineArgs> for OnlineArgs {
    fn from(item: CliOnlineArgs) -> Self {
        let nonce: u64 = 55;
        let block_hash: CryptoHash = Default::default();
        // let block_hash: CryptoHash = crate::common::BlobAsBase58String::<near_primitives::hash::CryptoHash>::from_str("EKmWQyhJCUKjovT3AYgQ5Y8oddUfAFg7wDocHjz8xpn1").unwrap().into_inner();
        let selected_server = match item.selected_server {
            Some(cli_selected_server) => SelectServer::from(cli_selected_server),
            None => SelectServer::select_server()
        };
        OnlineArgs {
            nonce,
            block_hash,
            selected_server
        }
    }
}

impl From<CliOfflineArgs> for OfflineArgs {
    fn from(item: CliOfflineArgs) -> Self {
        let nonce: u64 = match item.nonce {
            Some(cli_nonce) => cli_nonce,
            None => OfflineArgs::input_nonce()
        };
        let block_hash = match item.block_hash {
            Some(cli_block_hash) => cli_block_hash.into_inner(),
            None => OfflineArgs::input_block_hash()
        };
        let send_from: SendFrom = match item.send_from {
            Some(cli_send_from) => SendFrom::from(cli_send_from),
            None => SendFrom::send_from()
        };
        // let selected_server = match item.selected_server {
        //     Some(cli_selected_server) => SelectServer::from(cli_selected_server),
        //     None => SelectServer::select_server()
        // };
        OfflineArgs {
            nonce,
            block_hash,
            send_from
            // selected_server
        }
    }
}

impl OfflineArgs {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) {
        println!("OfflineArgs process:        {:?}", prepopulated_unsigned_transaction);
        let selected_server_url = "".to_string(); 
        self.send_from.process(prepopulated_unsigned_transaction, selected_server_url).await;
        // self.selected_server.process(prepopulated_unsigned_transaction).await;
    }
    fn input_nonce() -> u64 {
        Input::new()
            .with_prompt("Enter transaction nonce (query the access key information with
                `near-cli utils view-access-key frol4.testnet ed25519:...` incremented by 1)")
            .interact_text()
            .unwrap()
    }
    fn input_block_hash() -> near_primitives::hash::CryptoHash {
        let input_block_hash: String = Input::new()
            .with_prompt("Enter recent block hash:")
            .interact_text()
            .unwrap();
        crate::common::BlobAsBase58String::<near_primitives::hash::CryptoHash>::from_str(&input_block_hash).unwrap().into_inner()
            
    }
}

impl OnlineArgs {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) {
        println!("OnlineArgs process:        {:?}", prepopulated_unsigned_transaction);
        // let selected_server_url = "".to_string(); 
        // self.send_from.process(prepopulated_unsigned_transaction, selected_server_url).await;
        self.selected_server.process(prepopulated_unsigned_transaction).await;
    }
    fn input_nonce() -> u64 {    // запрос 
        Input::new()
            .with_prompt("Enter transaction nonce (query the access key information with
                `near-cli utils view-access-key frol4.testnet ed25519:...` incremented by 1)")
            .interact_text()
            .unwrap()
    }
    fn input_block_hash() -> near_primitives::hash::CryptoHash {    // запрос
        let input_block_hash: String = Input::new()
            .with_prompt("Enter recent block hash:")
            .interact_text()
            .unwrap();
        crate::common::BlobAsBase58String::<near_primitives::hash::CryptoHash>::from_str(&input_block_hash).unwrap().into_inner()
            
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
