use std::fmt;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    // TODO: figure out how to incorporate this
    //MidirConnectError(midir::ConnectError),
    MidirInit(midir::InitError),
    MidirPortInfo(midir::PortInfoError),
    MidirSend(midir::SendError),
    MidiInputPortNotFound,
    MidiOutputPortNotFound,
    InvalidGlobalChannel(u8),
    InvalidControlMode(u8),
    InvalidLedMode(u8),
    InvalidMidiChannel(u8),
    ConnectionClosed,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (error_type, error) = match *self {
            Error::MidirInit(err) => ("Midir Init", err.to_string()),
            Error::MidirPortInfo(err) => ("Midir Init", err.to_string()),
            Error::MidirSend(err) => ("Midir Init", err.to_string()),
            Error::MidiInputPortNotFound =>
                ("MIDI input ports", "MIDI input device was not found.".to_string()),
            Error::MidiOutputPortNotFound =>
                ("MIDI output ports", "MIDI output device was not found.".to_string()),
            Error::InvalidGlobalChannel(channel) =>
                ("Invalid global MIDI channel",
                format!("Channel {} is not a valid global channel. Expected 0-15.", channel)),
            Error::InvalidControlMode(channel) =>
                ("Invalid control mode",
                format!("{} is not a valid control mode. Expected 0-5.", channel)),
            Error::InvalidLedMode(channel) =>
                ("Invalid LED mode",
                format!("{} is not a valid global channel. Expected 0 or 1.", channel)),
            Error::InvalidMidiChannel(channel) =>
                ("Invalid MIDI channel",
                format!("Channel {} is not a valid global channel. Expected 0-16.", channel)),
            Error::ConnectionClosed => ("Connection closed", "Connection closed".to_string()),
        };

        write!(f, "{} error: {}", error_type, error)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::MidirInit(ref err) => err.description(),
            Error::MidirPortInfo(ref err) => err.description(),
            Error::MidirSend(ref err) => err.description(),
            Error::MidiInputPortNotFound => "MIDI input device was not found.",
            Error::MidiOutputPortNotFound => "MIDI output device was not found.",
            Error::InvalidGlobalChannel(_) => "Invalid global MIDI channel.",
            Error::InvalidControlMode(_) => "Invalid control mode.",
            Error::InvalidLedMode(_) => "Invalid LED mode.",
            Error::InvalidMidiChannel(_) => "Invalid MIDI channel.",
            Error::ConnectionClosed => "Connection closed."
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::MidirInit(err) => Some(err),
            Error::MidirPortInfo(err) => Some(err),
            Error::MidirSend(err) => Some(err),
            _ => None,
        }
    }
}

impl From<midir::InitError> for Error {
    fn from(err: midir::InitError) -> Self {
        Error::MidirInit(err)
    }
}

impl From<midir::PortInfoError> for Error {
    fn from(err: midir::PortInfoError) -> Self {
        Error::MidirPortInfo(err)
    }
}

impl From<midir::SendError> for Error {
    fn from(err: midir::SendError) -> Self {
        Error::MidirSend(err)
    }
}
