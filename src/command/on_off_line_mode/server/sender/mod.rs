use structopt::StructOpt;
use dialoguer::{
    Input,
};

mod receiver;
use receiver::{
    Receiver,
    CliReceiver,
    ActionSubcommand,
};


#[derive(Debug)]
pub struct Sender {
    pub account_id: String,
    pub send_to: SendTo
}

#[derive(Debug)]
pub enum SendTo {
    Receiver(Receiver)
}

#[derive(Debug, StructOpt)]
pub struct CliSender {
    pub account_id: Option<String>,
    #[structopt(subcommand)]
    send_to: Option<CliSendTo> 
}
#[derive(Debug, StructOpt)]
pub enum CliSendTo {
    Receiver(CliReceiver),
}

impl Sender {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: String,
    ) {
        println!("Sender process:\n    {:?}", &self);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self.account_id.clone(),
            .. prepopulated_unsigned_transaction
        };
        self.send_to.process(unsigned_transaction, selected_server_url).await;
    }
    pub fn input_account_id() -> String {
        Input::new()
            .with_prompt("What is the account ID of the sender?")
            .interact_text()
            .unwrap()
    }
}

impl From<CliSender> for Sender {
    fn from(item: CliSender) -> Self {
        let account_id: String = match item.account_id {
            Some(cli_account_id) => cli_account_id,
            None => Sender::input_account_id()
        };
        let send_to: SendTo = match item.send_to {
            Some(cli_send_to) => SendTo::from(cli_send_to),
            None => SendTo::send_to()
        }; 
        Sender {
            account_id,
            send_to
        }
    }
}

impl SendTo {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: String,
    ) {
        println!("SendTo process: self:\n       {:?}", &self);
        println!("SendTo process: prepopulated_unsigned_transaction:\n       {:?}", &prepopulated_unsigned_transaction);
        match self {
            SendTo::Receiver(receiver) => receiver.process(prepopulated_unsigned_transaction, selected_server_url).await
        }
    }
    pub fn send_to() -> Self {
        let account_id: String = Receiver::input_account_id();
        let transaction_subcommand: ActionSubcommand = ActionSubcommand::choose_action_command();
        SendTo::Receiver(Receiver {
            account_id,
            transaction_subcommand
        })
    }
}

impl From<CliSendTo> for SendTo {
    fn from(item: CliSendTo) -> Self {
        match item {
            CliSendTo::Receiver(cli_receiver) => {
                let receiver = Receiver::from(cli_receiver);
                SendTo::Receiver(receiver)
            }
        }
    }
}
