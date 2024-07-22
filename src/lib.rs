use std::{
    collections::VecDeque,
    fmt::Display,
    string::FromUtf8Error,
    sync::{Arc, RwLock},
};

pub mod cli;
pub mod test;

#[derive(Clone, Default, Debug)]
pub struct DataStore {
    data: Arc<RwLock<VecDeque<String>>>,
}

impl DataStore {
    pub fn publish(&mut self, message: String) -> Result<(), Error> {
        println!("Publishing message: {}", &message);
        self.data
            .write()
            .map_err(|e| Error::WriteLockError(e.to_string()))?
            .push_back(message);

        Ok(())
    }

    pub fn retrieve(&self) -> Result<Option<String>, Error> {
        let message = {
            let msg_arc = self.data.read().map_err(|_e| Error::EmptyMessage)?;
            msg_arc.clone()
        }
        .pop_front();

        println!("Retrieving {:?}", message);

        Ok(message)
    }
}

pub fn parse(data: &str) -> Result<Command, Error> {
    let parts: Vec<&str> = data.splitn(2, ' ').collect();

    let parts = parts.as_slice();

    match parts[0] {
        "RETRIEVE\n" => {
            if parts.len() > 1 {
                return Err(Error::UnexpectedPayload);
            }
            Ok(Command::Retrieve)
        }
        "PUBLISH" | "PUBLISH\n" => {
            if parts.len() < 2 {
                Err(Error::MissingPayload)
            } else if (parts[1].contains('\n') && !parts[1].ends_with('\n'))
                || parts[1].trim().contains('\n')
            {
                Err(Error::TrailingData)
            } else {
                Ok(Command::Publish(parts[1].trim().to_string()))
            }
        }
        "RETRIEVE" => {
            if parts.len() < 2 {
                return Err(Error::IncompleteMessage);
            }
            Err(Error::UnexpectedPayload)
        }
        "" => Err(Error::IncompleteMessage),
        "\n" => Err(Error::EmptyMessage),
        _ => Err(Error::UnknownCommand),
    }
}

/// Stores and Retrieves from DataStore
pub fn process_command(command: Command, mut data_store: DataStore) -> Result<(), Error> {
    match command {
        Command::Publish(message) => data_store.publish(message),
        Command::Retrieve => data_store.retrieve().map(|_| ()),
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteMessage,
    TrailingData,
    EmptyMessage,
    ListernerStartFail(String),
    UnknownCommand,
    UnexpectedPayload,
    MissingPayload,
    BytesConversion(FromUtf8Error),
    TcpStreamError(String),
    WriteLockError(String),
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::IncompleteMessage => "Incomplete message",
            Error::TrailingData => "Trailing data",
            Error::EmptyMessage => "Empty message",
            Error::ListernerStartFail(e) => e,
            Error::UnknownCommand => "Unknown command",
            Error::UnexpectedPayload => "Unexpected payload",
            Error::MissingPayload => "Missing payload",
            Error::BytesConversion(_e) => "Byte conversion error",
            Error::TcpStreamError(e) => e,
            Error::WriteLockError(e) => e,
            Error::Other(e) => e,
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {}
