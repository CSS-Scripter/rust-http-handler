use std::{net::{TcpListener, TcpStream}, io::{BufRead, BufReader, Write}, fs};
mod http;
use http::paths;
use http::response;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    let mut paths = paths::Paths::new(default_route_handler);
    paths.add_route(String::from("GET"), String::from("/"), handle_get_root);
    paths.add_route(String::from("GET"), String::from("/500"), handle_500);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(&paths, stream);
    }
}

fn handle_connection(paths: &paths::Paths, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let request_segments = request_line.split(" ").collect::<Vec<&str>>();
    let method = request_segments.get(0).unwrap();
    let path = request_segments.get(1).unwrap();

    let response = paths.handle(method.to_string(), path.to_string());
    stream.write_all(response.to_string().as_bytes()).unwrap();
}

fn handle_get_root() -> response::Response {
    let mut response = response::Response::new();
    response.set_status(200);
    response.set_content(fs::read_to_string("html/hello.html").unwrap());
    response.add_header("Content-Type".to_string(), "text/html".to_string());
    return response;
}

fn handle_500() -> response::Response {
    let mut response = response::Response::new();
    response.set_status(500);
    let contents = fs::read_to_string("html/error.html").unwrap()
        .replace("{{ CODE }}", "500")
        .replace("{{ MESSAGE }}", "Internal Server Error");

    response.set_content(contents);
    response.add_header("Content-Type".to_string(), "text/html".to_string());
    return response;
}

fn default_route_handler() -> response::Response {
    let mut response = response::Response::new();
    response.set_status(404);
    let contents = fs::read_to_string("html/error.html").unwrap()
        .replace("{{ CODE }}", "404")
        .replace("{{ MESSAGE }}", "Not Found");

    response.set_content(contents);
    response.add_header("Content-Type".to_string(), "text/html".to_string());
    return response;
}