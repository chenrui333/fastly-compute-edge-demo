use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

const HTTPBIN: &str = "httpbin";

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Filter request methods...
    match req.get_method() {
        // Allow GET and HEAD requests.
        &Method::GET | &Method::HEAD => (),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    // Pattern match on the path...
    match req.get_path() {
        "/" => {
            // Send a default synthetic response.
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("welcome.html")))
        }

        "/ping" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body("pong from fastly-compute-edge-demo"))
        }

        // Route wildcard path to httpbin backend
        _ => Ok(req.send(HTTPBIN)?),
    }
}
