use url::Url;
use slog::Logger;
use slog;

pub struct Credential {
    pub username: String,
    pub password: String,
    pub has_auth: bool,
}

pub struct ConnectParams {
    pub url: String,
    pub logger: Logger,
    _PRIVATE: (),
}




impl<'a> From<&'a str> for ConnectParams  {
    fn from(url: &'a str) -> ConnectParams {
        ConnectParams {
            url: url.to_string(),
            logger: slog::Logger::root(
                slog::Discard,
                o!(),
            ),
            _PRIVATE: ()
        }
    }
}

impl<'a> From<(&'a str, Logger)> for ConnectParams {
    fn from(tuple: (&'a str, Logger)) -> ConnectParams {
        let (url, logger) = tuple;
        ConnectParams {
            url: url.to_string(),
            logger: logger,
            _PRIVATE: (),
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

}
