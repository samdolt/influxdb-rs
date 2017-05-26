#[macro_use] extern crate hyper;

extern crate reqwest;
extern crate url;
extern crate semver;

mod params;
mod connection;
mod lines;


pub use connection::Connection;
pub use params::ConnectParams;
pub use params::Credential;
pub use lines::LinesBuilder;
pub use lines::Lines;

mod headers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
