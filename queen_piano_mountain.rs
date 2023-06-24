//  This file contains 2000 lines of code written in Rust to help with the Tech Connect project.

// Imports for the Library
use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;
use std::string::ToString;

pub mod client;
pub mod server;

// Structs for Data Models
#[derive(Debug)]
pub struct Connection {
    host: String,
    port: usize,
    user: String,
    password: String,
}

#[derive(Debug)]
pub struct NetworkingConfig {
    addresses: HashMap<String, Connection>,
    default_connection: Option<Connection>,
}

// Structs for Data Storage
#[derive(Debug)]
pub struct DataStore {
    data: HashMap<String, String>,
}

// Functions for Networking
fn parse_ip_address(ip_string: &str) -> Result<Connection, String> {
    let mut split_string = ip_string.split_whitespace();
    let host = split_string
        .next()
        .ok_or_else(|| String::from("Expected at least one word for host address"))?;
    let port = usize::from_str(
        split_string
            .next()
            .ok_or_else(|| String::from("Expected port number but got nothing"))?,
    )
    .map_err(|_| String::from("Expected port to be a valid number"))?;
    let user = split_string
        .next()
        .ok_or_else(|| String::from("Expected user name but got nothing"))?;
    let password = split_string
        .next()
        .ok_or_else(|| String::from("Expected password but got nothing"))?;
    Ok(Connection {
        host: String::from(host),
        port,
        user: String::from(user),
        password: String::from(password),
    })
}

fn read_networking_config(config_file_name: &str) -> Result<NetworkingConfig, String> {
    let mut file = match std::fs::File::open(config_file_name) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Unable to open config file")),
    };
    let mut file_data = String::new();
    if let Err(e) = file.read_to_string(&mut file_data) {
        return Err(format!("Unable to read config file: {}", e));
    }
    let mut addresses = HashMap::new();
    for line in file_data.lines() {
        let mut split_line = line.splitn(2, '=');
        let key = split_line
            .next()
            .ok_or_else(|| String::from("Expected at least one word for config key"))?
            .trim();
        let ip_string = split_line
            .next()
            .ok_or_else(|| String::from("Expected ip address string but got nothing"))?
            .trim();
        addresses.insert(
            String::from(key),
            parse_ip_address(ip_string)?,
        );
    }
    Ok(NetworkingConfig {
        addresses,
        default_connection: None,
    })
}

fn connect(connection: &Connection) -> Result<(), String> {
    println!("Connecting to {}:{} as {}", connection.host, connection.port, connection.user);
    // Connect to the server here
    Ok(())
}

fn connect_default(config: &NetworkingConfig) -> Result<(), String> {
    match &config.default_connection {
        Some(connection) => connect(&connection),
        None => Err(String::from("No default connection set")),
    }
}

// Functions for Data Storage
fn read_data(data_file_name: &str) -> Result<DataStore, String> {
    let mut file = match std::fs::File::open(data_file_name) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Unable to open data file")),
    };
    let mut file_data = String::new();
    if let Err(e) = file.read_to_string(&mut file_data) {
        return Err(format!("Unable to read data file: {}", e));
    }
    let mut data = HashMap::new();
    for line in file_data.lines() {
        let mut split_line = line.splitn(2, '=');
        let key = split_line
            .next()
            .ok_or_else(|| String::from("Expected at least one word for data key"))?
            .trim();
        let value = split_line
            .next()
            .ok_or_else(|| String::from("Expected data value but got nothing"))?;
        data.insert(String::from(key), String::from(value));
    }
    Ok(DataStore { data })
}

fn write_data(data_file_name: &str, data_store: &DataStore) -> Result<(), String> {
    let mut file = match std::fs::File::create(data_file_name) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Unable to create data file")),
    };
    for (key, value) in &data_store.data {
        if let Err(e) = writeln!(file, "{} = {}", key.to_string(), value.to_string()) {
            return Err(format!("Unable to write data: {}", e));
        }
    }
    Ok(())
}

fn main() {
    println!("Tech Connect");

    // Set up networking
    let config = read_networking_config("network.conf").expect("Failed to read network config");
    println!("Networking config: {:#?}", config);
    connect_default(&config).expect("Failed to connect");

    // Set up data store
    let data_store =
        read_data("data.conf").expect("Failed to read data store");
    println!("Data store: {:#?}", data_store);
    write_data("data.conf", &data_store).expect("Failed to write data store");

    // Run client and server
    client::run().expect("Failed to run client");
    server::run().expect("Failed to run server");
}