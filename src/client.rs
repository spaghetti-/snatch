use hyper::client::Client;
use hyper::client::response::Response;
use hyper::error::Error;
use hyper::header::Headers;
use hyper::method::Method;

/// Trait that represents some methods to send a specific request
pub trait GetResponse {
    /// Given a specific URL, get the response from the target server
    fn get_http_response(url: &str) -> Result<Response, Error>;
    /// Given a specific URL and an header, get the response from the target server
    fn get_http_response_using_headers(url: &str, header: Headers) -> Result<Response, Error>;
}

impl GetResponse for Client {
    fn get_http_response(url: &str) -> Result<Response, Error> {
        Client::new().request(Method::Get, url).send()
    }
    fn get_http_response_using_headers(url: &str,
                                       custom_header: Headers)
                                       -> Result<Response, Error> {
        Client::new().request(Method::Get, url).headers(custom_header).send()
    }
}