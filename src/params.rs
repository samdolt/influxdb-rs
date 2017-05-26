use url::Url;

pub struct Credential {
    pub username: String,
    pub password: String,
    pub has_auth: bool,
}

pub struct ConnectParams {
    pub url: Url,
    pub port: u16,
    pub credential: Credential,
    pub db: String,
}

impl ConnectParams{


    pub fn from_url(in_url: &str) -> Result<ConnectParams, ()> {
        let in_url = match Url::parse(in_url) {
            Ok(url) => url,
            Err(_) => return Err(()),
        };

        let scheme = match in_url.scheme() {
            "http" => "http",
            "https" => "https",
            _   => return Err(()),
        };

        let host = match in_url.host() {
            Some(host) => host,
            None => return Err(()),
        };

        let port = match in_url.port() {
            Some(port) => port,
            None    => 8086u16,
        };

        let username = match in_url.username() {
            "" => None,
            u  => Some(u.to_string())
        };

        let password = match in_url.password(){
            None => None,
            Some(pw) => Some(pw.to_string()),
        };

        let db = in_url.path()[1..].to_string();

        // Can't fail
        let base_url = Url::parse(&format!("{}://{}:{}/", scheme, host, port )).unwrap();

        let auth = match username {
            None => Credential{username: "".to_string(), password: "".to_string(), has_auth: false},
            Some(u) => {
                match password {
                    None => Credential{username: "".to_string(), password: "".to_string(), has_auth: false},
                    Some(p) => Credential{username:u ,password:p, has_auth: true}
                }
            }
        };


        Ok(ConnectParams {
            url: base_url,
            credential: auth,
            port: port,
            db:db,
        })
    }

}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_connectparams_from_url() {
        let basic = ConnectParams::from_url("http://localhost").unwrap();
        let auth = ConnectParams::from_url("http://a:b@localhost").unwrap();
        assert!(auth.credential.has_auth)
    }
}
