use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnknowConfigDir(),
    Io(std::io::Error),
    Toml(toml::de::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknowConfigDir() => {
                write!(f, "failed to lock up a system configuration directory")
            }
            Error::Io(err) => write!(f, "{}", err),
            Error::Toml(err) => write!(f, "{}", err),
        }
    }
}
