mod task;
mod tutorial;

use std::path::PathBuf;
use structopt::StructOpt;
use tutorial::{Action::Add, Action::Done, Action::List, CommandLineArgs};

fn search_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".journal.json");
        path
    })
}

fn main() {
    let CommandLineArgs { action, journal } = CommandLineArgs::from_args();
    let journal = journal
        .or_else(search_journal_file)
        .expect("Erro ao procurar arquivo com tarefas");

    match action {
        Add { text } => task::add(journal, task::Task::new(text)),
        List => task::list(journal),
        Done { position } => task::completed(journal, position),
    }
    .expect("Erro ao corresponder o comando");
}
