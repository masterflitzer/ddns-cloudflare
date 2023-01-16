use std::{error::Error, io::Error as IOError, io::ErrorKind as IOErrorKind};

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Config(IOError),
    ConfigPath(IOError),
    IPv4,
    IPv6,
    JsonDeserialize,
    JsonSerialize,
    NoSuccessHttp { name: String, type_: String },
    NoSuccessJson { name: String, type_: String },
    API,
    Unknown(Box<dyn Error>),
}

pub(crate) fn handle_errors(kind: &ErrorKind) {
    match kind {
        ErrorKind::API => println!("The HTTP client encountered an unexpected error while trying to connect to the API"),
        ErrorKind::Config(e) => println!("An error occurred while parsing the configuration. Please consult the readme for an example configuration.\n\n{}", e),
        ErrorKind::ConfigPath(e) => println!("An error occurred while trying to get the path to the configuration file.\n\n{}", e),
        ErrorKind::IPv4 => println!("An error occurred while trying to determine the IPv4 address"),
        ErrorKind::IPv6 => println!("An error occurred while trying to determine the IPv6 address"),
        ErrorKind::JsonDeserialize => println!("An error occurred while deserializing JSON"),
        ErrorKind::JsonSerialize => println!("An error occurred while serializing JSON"),
        ErrorKind::NoSuccessHttp { name, type_ } => println!(
            "{}: Skipping {} because HTTP status code was not between 200-299",
            name, type_
        ),
        ErrorKind::NoSuccessJson { name, type_ } => println!(
            "{}: Skipping {} because JSON payload did not contain {{ \"success\": true }}",
            name, type_
        ),
        ErrorKind::Unknown(e) => println!("An unexpected error occured!\n\n{}", e),
    };
}

pub(crate) fn create_generic_error() -> IOError {
    IOError::from(IOErrorKind::Other)
}
