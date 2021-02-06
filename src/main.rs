use structopt::StructOpt;
use strum_macros::{Display, EnumString, EnumVariantNames};

mod consts;
mod command;
use command::{
    CliCommand,
    ArgsCommand
};


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


impl From<CliArgs> for Args {
    fn from(item: CliArgs) -> Self {
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
    // let cli = CliArgs::default();
    let cli = CliArgs::from_args();
    println!("cli: {:?}", cli);

    let args = Args::from(cli);
    println!("args {:#?}", args);
}
