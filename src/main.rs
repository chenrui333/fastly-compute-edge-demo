use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};
use ipnetwork::IpNetwork;
use std::net::IpAddr;

const HTTPBIN: &str = "httpbin";

fn is_ip_in_cidr(ip: IpAddr, cidr: IpNetwork) -> bool {
    cidr.contains(ip)
}

fn is_allowed(ip: IpAddr, user_agent: &str) -> bool {
    let cidrs: Vec<&str> = vec![
        "52.119.64.2/32", // hq 4.0 ip
    ]; // Allowlist of CIDR blocks
    let allowlist: Vec<IpNetwork> = cidrs.iter().map(|&cidr| cidr.parse().unwrap()).collect();

    let user_agents: Vec<&str> = vec!["hurl"]; // Allowlist of User-Agents

    let ip_allowed = allowlist.iter().any(|&cidr| is_ip_in_cidr(ip, cidr));
    let user_agent_allowed = user_agents.contains(&user_agent);

    ip_allowed || user_agent_allowed
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    match (req.get_client_ip_addr(), req.get_header_str("User-Agent")) {
        (Some(ip), Some(user_agent)) => {
            if is_allowed(ip, user_agent) {
                println!("The IP and User-Agent are in the allowlist");
            } else {
                println!("The IP or User-Agent are not in the allowlist");
                return Ok(Response::from_status(StatusCode::FORBIDDEN)
                    .with_content_type(mime::TEXT_HTML_UTF_8)
                    .with_body_text_plain("You are not allowed to access this resource\n"));
            }
        }
        _ => {
            // return the response
            return Ok(Response::from_status(StatusCode::BAD_REQUEST)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body_text_plain(""));
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

    let robots_response = r#"User-agent: *
Disallow: /
"#;

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
            .with_body("pong from fastly-compute-edge-demo"))
            .with_header(header::CACHE_CONTROL, "public, max-age=31536000")),

        "/robots.txt" => Ok(Response::from_body(robots_response)
            .with_content_type(mime::TEXT_PLAIN_UTF_8)
            .with_header(header::CACHE_CONTROL, "public, max-age=31536000")),

        // Route wildcard path to httpbin backend
        _ => Ok(req.send(HTTPBIN)?),
    }
}
