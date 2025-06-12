use tiny_http::{Header, Method, Response, Server};

fn main() {
    let server = Server::http("127.0.0.1:8000").expect("failed to bind server");
    println!("Server running at http://127.0.0.1:8000/");
    for request in server.incoming_requests() {
        if request.method() == &Method::Get && request.url() == "/" {
            let html = include_str!("../../assets/editor.html");
            let response = Response::from_string(html).with_header(
                Header::from_bytes("Content-Type", "text/html; charset=utf-8").unwrap(),
            );
            let _ = request.respond(response);
        } else {
            let _ = request.respond(Response::empty(404));
        }
    }
}
