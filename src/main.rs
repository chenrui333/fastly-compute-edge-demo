use std::net::IpAddr;
use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};
use ipnetwork::IpNetwork;

const HTTPBIN: &str = "httpbin";

fn is_ip_in_cidr(ip: IpAddr, cidr: IpNetwork) -> bool {
  cidr.contains(ip)
}

fn is_allowed(ip: IpAddr) -> bool {
  let cidrs: Vec<&str> = vec![
    "52.119.64.2/32" // hq 4.0 ip
  ]; // Allowlist of CIDR blocks
  let allowlist: Vec<IpNetwork> = cidrs.iter().map(|&cidr| cidr.parse().unwrap()).collect();
  allowlist.iter().any(|&cidr| is_ip_in_cidr(ip, cidr))
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    match req.get_client_ip_addr() {
        Some(ip) => {
            if is_allowed(ip) {
                println!("client ip: {} is allowed", ip);
            } else {
                println!("client ip: {} is not allowed", ip);
                return Ok(Response::from_status(StatusCode::FORBIDDEN)
                    .with_content_type(mime::TEXT_HTML_UTF_8)
                    .with_body(format!("Forbidden, your ip is {}", ip)));
            }
        }
        None => {
            println!("no client ip");
        }
    }

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

        "/ping" => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body("pong from fastly-compute-edge-demo")),

        // Route wildcard path to httpbin backend
        _ => Ok(req.send(HTTPBIN)?),
    }
}
