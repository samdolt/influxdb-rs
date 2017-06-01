error_chain!{

    foreign_links {
        Io(::std::io::Error);
        Url(::url::ParseError);
        Request(::reqwest::Error);
        Version(::semver::SemVerError);
    }

    errors {
         UnsupportedProtocol(t: String) {
            description("unsupported protocol")
            display("unsupported protocol: '{}'", t)
        }

        InvalidUrl(t: String) {
            description("invalid url")
            display("invalid url: '{}'", t)
        }

        HttpError(t: String){
            description("Wrong http status code provided by server")
            display("http error: '{}'", t)
        }

        InvalidVersionHeader{
            description("Influxdb doesn't respond with a version header")
            display("Influxdb doesn't respond with a version header")
        }
    }

}