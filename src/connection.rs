use url::Url;
use url::form_urlencoded::byte_serialize;

use semver::Version;
use reqwest::get;
use reqwest::StatusCode;
use reqwest::Client;



use ::headers::XInfluxDbVersion;
use ::ConnectParams;
use ::Credential;

use ::Lines;


pub struct Connection {
    url: Url,
    auth: Credential,
    version: Version,
    client: Client,
    db: String,
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
    pub fn connect(url: &str) -> Result<Connection, ()>
    {
        let params = ConnectParams::from_url(url)?;

        let version = match Connection::_ping(&params.url, &params.credential) {
            Some(v) => v,
            None    => return Err(()),
        };

        Ok(Connection {
            url: params.url,
            auth: params.credential,
            version: version,
            client: Client::new().unwrap(),
            db: params.db,
        })
    }

    fn _ping(base_url: &Url, auth: &Credential) -> Option<Version> {

        let join = if auth.has_auth {
            format!("ping?u={}&p={}", auth.username, auth.password)
        } else {
            "ping".to_string()
        };

        let endpoint = base_url.join(&join).unwrap();

        match get(endpoint) {
            Ok(response) => {
                match response.status() {
                    &StatusCode::NoContent => (),
                    _ => return None,
                };

                match response.headers().get::<XInfluxDbVersion>() {
                   Some(v) => match Version::parse(v) {
                       Ok(v) => Some(v),
                       Err(_) => None,
                   },

                    None => None,
                }

            },
            Err(_) => None,
        }

    }

    pub fn write(self, lines: &Lines) -> Result<(),()> {
        let url = self.url.join(&format!("write?db={}", self.db)).unwrap();
        let response = self.client.post(url).body(lines.as_str()).send();

        match response {
            Ok(r) => {
                match r.status() {
                    &StatusCode::NoContent => Ok(()),
                    _ => Err(()),
                }
            },
            Err(_) => Err(()),
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
