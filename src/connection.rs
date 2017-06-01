use url::Url;

use semver::Version;
use reqwest::get;
use reqwest::StatusCode;
use reqwest::Client;

use slog::Logger;


use ::headers::XInfluxDbVersion;
use ::ConnectParams;
use ::Credential;

use ::Lines;
use ::Result;
use ::ErrorKind;


pub struct Connection {
    url: Url,
    auth: Credential,
    version: Version,
    client: Client,
    db: String,
    logger: Logger,
}

impl Connection {
    /// Creates a new connection to a Influxdb database.
    ///
    /// Most applications can use a URL string in the normal format:
    ///
    /// ```notrust
    /// http://[user:password]@host[:port]/database[?param1=val1[[&param2=val2]...]]
    /// ```
    ///
    /// The user and password may be omitted if not required. The default Influxdb port
    /// (8086) is used if none is specified.
    ///
    /// # Examples
    ///
    /// To connect over http:
    ///
    /// ```no_run
    /// use influxdb::Connection;
    ///
    /// let url = "http://myuser:mypassword@localhost:2994/foodb";
    /// let conn = Connection::connect(url).expect("Can't connect to influxdb");
    /// ```
    pub fn connect<T>(params: T) -> Result<Connection> where T: Into<ConnectParams>
    {
        let params = params.into();

        let url = Url::parse(&params.url)?;

        debug!(params.logger, "Url parsed");

        let scheme = match url.scheme() {
            "http" => "http",
            "https" => "https",
            x   => return Err(ErrorKind::UnsupportedProtocol(x.to_string()).into()),
        };

        debug!(params.logger, "Scheme parsed");

        let host = match url.host() {
            Some(host) => host,
            None => return Err(ErrorKind::InvalidUrl(url.to_string()).into()),
        };

        debug!(params.logger, "Host parsed");

        let port = match url.port() {
            Some(port) => port,
            None    => 8086u16,
        };

        debug!(params.logger, "Port parsed");

        let username = match url.username() {
            "" => None,
            u  => Some(u.to_string())
        };

        debug!(params.logger, "Username parsed");

        let password = match url.password(){
            None => None,
            Some(pw) => Some(pw.to_string()),
        };

        debug!(params.logger, "Username parsed");

        let db = url.path()[1..].to_string();

        debug!(params.logger, "db parsed");

        // Can't fail
        let base_url = Url::parse(&format!("{}://{}:{}/", scheme, host, port )).unwrap();
        debug!(params.logger, "Base Url recreated");

        let auth = match username {
            None => Credential{username: "".to_string(), password: "".to_string(), has_auth: false},
            Some(u) => {
                match password {
                    None => Credential{username: "".to_string(), password: "".to_string(), has_auth: false},
                    Some(p) => Credential{username:u ,password:p, has_auth: true}
                }
            }
        };

        debug!(params.logger, "Auth parsed");


        let version = Connection::_ping(&base_url, &auth)?;

        debug!(params.logger, "Version recupered");

        let logger = params.logger.new(o!("influx-version" => version.to_string()));

        info!(logger, "Connected to influxdb");

        Ok(Connection {
            url: base_url.clone(),
            auth: auth,
            version: version,
            client: Client::new().unwrap(),
            db: db,
            logger: logger
        })
    }

    fn _ping(base_url: &Url, auth: &Credential) -> Result<Version> {

        let join = if auth.has_auth {
            format!("ping?u={}&p={}", auth.username, auth.password)
        } else {
            "ping".to_string()
        };

        let endpoint = base_url.join(&join)?;

        let response = get(endpoint)?;
                match *response.status() {
                    StatusCode::NoContent => (),
                    x => return Err(ErrorKind::HttpError(x.to_string()).into()),
                };

                match response.headers().get::<XInfluxDbVersion>() {
                   Some(v) => Ok(Version::parse(v)?),
                    None => Err(ErrorKind::InvalidVersionHeader.into()),
                }


    }

    pub fn write(&self, lines: &Lines) -> Result<()> {
        let url = self.url.join(&format!("write?db={}", self.db)).unwrap();
        debug!(self.logger.new(o!("url" => url.to_string())), "New write url");
        let lines = lines.as_str();
        info!(self.logger, "Write lines {}", lines);
        let response = self.client.post(url).body(lines).send()?;

                match response.status() {
                    &StatusCode::NoContent => Ok(()),
                    x => {
                        error!(self.logger, "Wrong status code");
                        Err(ErrorKind::HttpError(x.to_string()).into())},
                }
    }
}


#[cfg(test)]
mod tests {
    pub use super::*;
    pub use ::LinesBuilder;

    #[test]
    fn test_connect() {
        let con = Connection::connect("http://localhost/monitor").unwrap();
    }

    #[test]
    fn test_write() {
        let con = Connection::connect("http://localhost/monitor").unwrap();
        con.write(&LinesBuilder::new("t")
                      .add_field("a", 32)
                      .build()).unwrap();
    }
}
