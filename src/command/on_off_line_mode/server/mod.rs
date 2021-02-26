use structopt::StructOpt;
use dialoguer::{
    Input,
};

mod sender;
use sender::{CliSender, SendTo, Sender};


#[derive(Debug)]
pub struct Server {
    pub url: String,
    pub send_from: SendFrom
}

impl Server {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) {
        println!("Server process:\n        {:?}", &self);
        let selected_server_url: String = self.url.clone();
        self.send_from.process(prepopulated_unsigned_transaction, selected_server_url).await;
    }
}

#[derive(Debug)]
pub enum SendFrom {
    Sender(Sender)
}

impl SendFrom {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: String,
    ) {
        println!("Sendfrom process:\n      {:?}", &self);
        match self {
            SendFrom::Sender(sender) => sender.process(prepopulated_unsigned_transaction, selected_server_url).await,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct CliServer {
    #[structopt(subcommand)]
    pub send_from: Option<CliSendFrom> 
}

#[derive(Debug, StructOpt)]
pub struct CliCustomServer {
    #[structopt(long)]
    pub url: Option<String>,
    #[structopt(subcommand)]
    send_from: Option<CliSendFrom> 
}

#[derive(Debug, StructOpt)]
pub enum CliSendFrom {
    Sender(CliSender)
}

impl CliServer {
    pub fn into_server(self, url: String) -> Server {
        let send_from: SendFrom = match self.send_from {
            Some(cli_send_from) => SendFrom::from(cli_send_from),
            None => SendFrom::send_from()
        };
        Server {
            url,
            send_from,
        }
    }
}

impl CliCustomServer {
    pub fn into_server(self) -> Server {
        let url = match self.url {
            Some(url) => url,
            None => {
                Input::new()
                    .with_prompt("What is the RPC endpoi?")
                    .interact_text()
                    .unwrap()
            }
        };
        let send_from: SendFrom = match self.send_from {
            Some(cli_send_from) => SendFrom::from(cli_send_from),
            None => SendFrom::send_from()
        };
        Server {
            url,
            send_from,
        }
    }
}

impl From<CliSendFrom> for SendFrom {
    fn from(item: CliSendFrom) -> Self {
        match item {
            CliSendFrom::Sender(cli_sender) => {
                let sender: Sender = Sender::from(cli_sender);
                SendFrom::Sender(sender)
            }
        }
    }
}

impl SendFrom {
    pub fn send_from() -> Self {
        let account_id : String = Sender::input_account_id();
        let send_to: SendTo = SendTo::send_to();
        SendFrom::Sender(Sender {
            account_id,
            send_to
        })
    }
}
