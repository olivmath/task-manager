use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Show Node version
    Version,
    /// Run Node
    Up,
    /// Stop Node
    Down,
    /// Show status [Up] or [Stoped]
    Status,
    /// Show all info
    Info,
    /// Show logs
    Logs,
    /// Install a new version of Node
    Install {
        #[structopt()]
        version: String,
    },
    /// Edit any params of lunesnode.conf and restart Node
    Edit {
        #[structopt()]
        args: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Node CLI",
    about = "A Command Line for management a Lunes Node written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub actions: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
