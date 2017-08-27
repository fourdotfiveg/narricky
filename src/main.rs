#[macro_use]
extern crate error_chain;
extern crate imap;
extern crate openssl;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod error;
mod config;

use openssl::ssl::{SslConnectorBuilder, SslMethod};
use openssl::ssl::SslStream;
use imap::client::Client;
use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::net::TcpStream;
use toml::Value;

use config::Config;

fn parse_file<P: AsRef<Path>>(path: P) -> Client<SslStream<TcpStream>> {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let config_val: Value = toml::from_str(&buf).unwrap();
    let config = match Config::from_toml(config_val) {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            ::std::process::exit(1);
        }
    };
    let account = config.account;
    let ssl_connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap().build();
    let mut imap_socket = Client::secure_connect(
        (account.domain.as_str(), account.port),
        &account.domain,
        ssl_connector,
    ).unwrap();
    imap_socket
        .login(&account.username, &account.password)
        .unwrap();
    imap_socket
}

fn main() {
    let mut args = env::args();
    let mut imap_socket;
    args.next();
    if let Some(file) = args.next() {
        imap_socket = parse_file(file);
    } else {
        panic!("Missing file");
    }
    //match imap_socket.capability() {
    //    Ok(capabilities) => {
    //        for capability in capabilities.iter() {
    //            println!("{}", capability);
    //        }
    //    }
    //    Err(e) => println!("Error parsing capability: {}", e),
    //};

    //match imap_socket.select("INBOX") {
    //    Ok(mailbox) => {
    //        println!("{}", mailbox);
    //    }
    //    Err(e) => println!("Error selecting INBOX: {}", e),
    //};

    ////imap_socket.create("NEWBOW/SubBox").unwrap();

    //match imap_socket.list("/", "*") {
    //    Ok(a) => {
    //        for b in a {
    //            println!("{}", b);
    //        }
    //    }
    //    Err(e) => println!("Error listing: {}", e),
    //}


    // match imap_socket.fetch("2", "body[text]") {
    //     Ok(lines) => {
    //         for line in lines.iter() {
    //             print!("{}", line);
    //         }
    //     }
    //     Err(e) => println!("Error Fetching email 2: {}", e),
    // };

    imap_socket.logout().unwrap();
}
