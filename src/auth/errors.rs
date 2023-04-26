#[derive(Debug)]
pub enum AuthErrors {
    AuthMissing,
    InvalidAuthHeaderError,
    WrongCredentials,
    MissingOrgError,
    DecodeTokenError,
}

pub enum DBErrors {
    CouldNot,
}

impl std::error::Error for AuthErrors {}

impl std::fmt::Display for AuthErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthErrors::AuthMissing => write!(f, "Missing authorization in header."),
            AuthErrors::InvalidAuthHeaderError => {
                write!(f, "Invalid authorization format.")
            }
            AuthErrors::WrongCredentials => write!(f, "Wrong credentials entered."),
            AuthErrors::MissingOrgError => write!(f, "No organization was provided!"),
            AuthErrors::DecodeTokenError => write!(f, "Failed to generate token!"),
        }
    }
}
