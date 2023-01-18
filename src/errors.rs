use std::{error::Error, io::Error as IOError};

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Api,
    Config(IOError),
    ConfigPath(IOError),
    IPv4,
    IPv6,
    Json,
    NonAddressRecord,
    NoSuccessHttp,
    NoSuccessJson,
    Unknown(Box<dyn Error>),
}

pub(crate) fn handle_errors(kind: &ErrorKind) {
    match kind {
        ErrorKind::Api => println!("The HTTP client encountered an unexpected error while trying to connect to the API"),
        ErrorKind::Config(e) => println!("An error occurred while parsing the configuration. Please consult the readme for an example configuration.\n{}", e),
        ErrorKind::ConfigPath(e) => println!("An error occurred while trying to get the path to the configuration file.\n{}", e),
        ErrorKind::IPv4 => println!("An error occurred while trying to determine the IPv4 address"),
        ErrorKind::IPv6 => println!("An error occurred while trying to determine the IPv6 address"),
        ErrorKind::Json => println!("An error occurred while (de)serializing JSON"),
        ErrorKind::NonAddressRecord => println!(
            "Encountered a record that was not of type \"A\" or \"AAAA\""
        ),
        ErrorKind::NoSuccessHttp => println!(
            "A HTTP response was unsuccessful (status code not between 200-299)"
        ),
        ErrorKind::NoSuccessJson => println!(
            "A JSON response contained invalid data (missing {{ \"success\": true }})"
        ),
        ErrorKind::Unknown(e) => println!("An unexpected error occured!\n{}", e),
    };
}
