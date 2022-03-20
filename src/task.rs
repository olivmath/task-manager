use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::{
    fs::{File, OpenOptions},
    io::{Error, ErrorKind, Result, Seek, SeekFrom},
    path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    text: String,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        Task {
            text,
            created: Utc::now(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}
pub fn write_file(file: File, tasks: Vec<Task>, path: &PathBuf) -> Result<()> {
    match serde_json::to_writer(file, &tasks) {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nErro ao salvar o arquivo {:#?} com a nova `task`",
                    e, path
                ),
            ))
        }
    }
}

pub fn rewind_file(mut file: File, path: &PathBuf) -> Result<()> {
    match file.seek(SeekFrom::Start(0)) {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(Error::new(
                ErrorKind::WriteZero,
                format!("{}\nErro ao rebobinar o arquivo {:#?}", e, path),
            ))
        }
    }
}

pub fn open_file(path: &PathBuf, read: bool, write: bool, create: bool) -> Result<File> {
    match OpenOptions::new()
        .read(read)
        .write(write)
        .create(create)
        .open(path)
    {
        Ok(file) => Ok(file),
        Err(e) => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{}\nErro ao abrir o arquivo {:#?}", e, path),
            ))
        }
    }
}

pub fn collect_task(file: File, path: &PathBuf) -> Result<Vec<Task>> {
    rewind_file(file.try_clone()?, &path)?;
    match serde_json::from_reader(&file) {
        Ok(tasks) => {
            rewind_file(file, &path)?;
            Ok(tasks)
        }
        Err(e) if e.is_eof() => {
            rewind_file(file, &path)?;
            Ok(Vec::new())
        }
        Err(e) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{}\nErro ao ler as `tasks` no arquivo {:#?}", e, path),
            ))
        }
    }
}

pub fn add(journal: PathBuf, task: Task) -> Result<()> {
    let file = open_file(&journal, true, true, true)?;
    let mut tasks: Vec<Task> = collect_task(file.try_clone()?, &journal)?;

    tasks.push(task);

    write_file(file, tasks, &journal)
}

pub fn completed(journal: PathBuf, position: usize) -> Result<()> {
    let file = open_file(&journal, true, true, true)?;
    let mut tasks: Vec<Task> = collect_task(file.try_clone()?, &journal)?;

    if position == 0 || position > tasks.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Sua `task` é invalida"),
        ));
    };
    tasks.remove(position - 1);
    match file.set_len(0) {
        Ok(_) => (),
        Err(e) => {
            return Err(Error::new(
                ErrorKind::WriteZero,
                format!("{}\nErro ao truncar o arquivo {:#?}", e, journal),
            ))
        }
    };

    write_file(file, tasks, &journal)
}

pub fn list(journal: PathBuf) -> Result<()> {
    let file = open_file(&journal, true, false, false)?;
    let tasks = collect_task(file, &journal)?;

    if tasks.is_empty() {
        println!("\n---\nYou don't have tasks! 📭\n---\n");
    } else {
        for (position, task) in tasks.iter().enumerate() {
            println!(
                "\n---\nIndex: {}\nTask: {:#?}\nCreated: {}\n---\n",
                position + 1, task.text, task.created
            );
        }
    };

    Ok(())
}
