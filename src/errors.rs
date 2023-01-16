use std::{error::Error, io::Error as IOError};

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Config(IOError),
    ConfigPath(IOError),
    IPv4,
    IPv6,
    JsonDeserialize,
    JsonSerialize,
    NoSuccessHttp(String),
    NoSuccessJson(String),
    API,
    Unknown(Box<dyn Error>),
}

pub(crate) fn handle_errors(kind: &ErrorKind) {
    match kind {
        ErrorKind::API => println!("The HTTP client encountered an unexpected error while trying to connect to the API"),
        ErrorKind::Config(e) => println!("An error occurred while parsing the configuration. Please consult the readme for an example configuration.\n{}", e),
        ErrorKind::ConfigPath(e) => println!("An error occurred while trying to get the path to the configuration file.\n{}", e),
        ErrorKind::IPv4 => println!("An error occurred while trying to determine the IPv4 address"),
        ErrorKind::IPv6 => println!("An error occurred while trying to determine the IPv6 address"),
        ErrorKind::JsonDeserialize => println!("An error occurred while deserializing JSON"),
        ErrorKind::JsonSerialize => println!("An error occurred while serializing JSON"),
        ErrorKind::NoSuccessHttp(name) => println!(
            "{}: Skipping record/zone because HTTP status code was not between 200-299",
            name
        ),
        ErrorKind::NoSuccessJson(name) => println!(
            "{}: Skipping record/zone because JSON payload did not contain {{ \"success\": true }}",
            name
        ),
        ErrorKind::Unknown(e) => println!("An unexpected error occured!\n{}", e),
    };
}
