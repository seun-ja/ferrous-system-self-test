use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use clap::Parser;
use ferrous_test::{cli::Args, parse, process_command, DataStore, Error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_command = Args::parse();

    let role = cli_command.command();

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    match role {
        ferrous_test::cli::Role::Admin => listen(addr).await,
        ferrous_test::cli::Role::User => send_message(addr).await,
    }
}

async fn listen(addr: SocketAddr) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).map_err(|e| Error::ListernerStartFail(e.to_string()))?;

    // Instantiate DataStore
    let data_store = DataStore::default();
    let store_arc = data_store.clone();

    // Start listening
    for stream in listener.incoming() {
        let mut bytes = Vec::new();
        let mut listener = stream.map_err(|e| Error::TcpStreamError(e.to_string()))?;

        let _ = listener.read_to_end(&mut bytes)?;

        let data = String::from_utf8(bytes).map_err(Error::BytesConversion)?;

        println!("Received: {:?}", data);

        let command = parse(&data)?;

        process_command(command, store_arc.clone())?;
    }
    Ok(())
}

async fn send_message(addr: SocketAddr) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect(addr).map_err(|e| Error::TcpStreamError(e.to_string()))?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        stream.write_all(input.as_bytes())?;
    }
}
