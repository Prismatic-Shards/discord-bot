use std::io::{self, Write};
use std::{
  error::Error,
  fs::{File, OpenOptions},
  io::BufWriter,
  path::Path,
};

use chrono::Local;

pub enum LogMessage {
  Bot(String),
  App(String),
  ShutDown,
}

pub fn get_log_file() -> Result<BufWriter<File>, Box<dyn Error>> {
  let now = Local::now();
  let path = format!("{}.txt", now.format("%Y-%m-%d_%H-%M-%S"));
  let path = Path::new(path.as_str());
  // Open a file in write-only mode, returns `io::Result<File>`
  let file = OpenOptions::new()
    .append(true) // Open file in append mode
    .create(true) // Create the file if it doesn't exist
    .open(path)?;
  let writer = BufWriter::new(file);
  return Ok(writer);
}

pub async fn write_entry(
  writer: &mut BufWriter<File>,
  message: LogMessage,
) -> Result<(), io::Error> {
  if let LogMessage::App(string) = message {
    writeln!(writer, "[APP]: {}", string)?;
  } else if let LogMessage::Bot(string) = message {
    writeln!(writer, "[BOT]: {}", string)?;
  } else {
    writeln!(writer, "[BOT]: Shut down at {}", Local::now())?;
    writer.flush()?;
  }
  Ok(())
}
