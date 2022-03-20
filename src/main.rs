mod task;
mod tutorial;

use std::{io::Error, io::ErrorKind, io::Result, path::PathBuf};
use structopt::StructOpt;
use tutorial::{Action::Add, Action::Done, Action::List, CommandLineArgs};

fn create_journal() -> Result<PathBuf> {
    use std::fs::File;

    let path = match home::home_dir() {
        Some(mut path) => {
            path.push(".journal.json");
            path
        }
        None => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Erro ao encontrar Home Path",
            ))
        }
    };
    match File::open(&path) {
        Ok(_) => Ok(path),
        Err(_) => {
            match File::create(&path) {
                Ok(_) => (),
                Err(e) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("{}\nErro ao criar arquivo `journal.json`", e),
                    ));
                }
            };
            Ok(path)
        }
    }
}

fn main() {
    let CommandLineArgs { action, journal } = CommandLineArgs::from_args();
    let journal = match journal {
        Some(path) => Some(path),
        None => match create_journal() {
            Ok(path) => Some(path),
            Err(_) => None,
        },
    }
    .expect("Erro ao abrir ao encontrar o arquivo");
    match action {
        Add { text } => task::add(journal, task::Task::new(text)),
        List => task::list(journal),
        Done { position } => task::completed(journal, position),
    }
    .expect("Erro ao corresponder o comando");
}
