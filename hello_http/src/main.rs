use hello_http::ThreadPool;
// I think this is the first time using a complex structure to import like this.
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        // A stream (in this case `TcpStream` represents an open connection between server and client.
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
    // Interestingly, the discard `_` can also be used to discard the generic type.
    //     let http_request: Vec<_> = buf_reader
    //         .lines()
    //         .map(|result| result.unwrap())
    //         .take_while(|line| !line.is_empty())
    //         .collect();
    // I'm not sure if `#` in the `println` macro has been used before, but it essentially pretty-prints the value.
    //     println!("Request: {http_request:?}");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello_http/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello_http/hello.html")
        // Apparently, if you use a scope, there's no need for a comma, even if another item follows.
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "hello_http/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
