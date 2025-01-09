use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // let address = "127.0.0.1:8080";
    let address = "0.0.0.0:8080";

    let listener = TcpListener::bind(&address).unwrap();

    println!("Conectado desde la direccion: {}", &address);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_conection(stream);
    }
}

fn handle_conection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // println!("Stream en curso");

    let status_buffer = String::from_utf8_lossy(&buffer[..]);
    // println!("{}", &status_buffer);

    let arrRequest: Vec<_> = status_buffer.split(" ").collect();

    let router = arrRequest[1];
    let arrPath: Vec<_> = router.split("/").collect();

    let mut path = String::from(arrPath[1]);

    if path == "" {
        path = String::from("index");
    }

    path = String::from(format!("public/{}.html", path));

    let verbose: [&str; 7] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

    loop {
        if buffer.starts_with("GET".as_bytes()) {
            App(router, stream, path);
        }

        if buffer.starts_with("POST".as_bytes()) {}

        if buffer.starts_with("PUT".as_bytes()) {}

        if buffer.starts_with("PATCH".as_bytes()) {}

        if buffer.starts_with("DELETE".as_bytes()) {}

        if buffer.starts_with("HEAD".as_bytes()) {}

        if buffer.starts_with("OPTIONS".as_bytes()) {}
        break;
    }
}

fn App(route: &str, mut stream: TcpStream, path: String) {
    let mut https_status_code = "";

    match fs::File::open(&path) {
        Ok(_file) => {
            https_status_code = "200 OK";

            print!("GET {} {}\n", route, https_status_code);

            let content = fs::read_to_string(&path).unwrap();
            let res = format!(
                "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                &https_status_code,
                content.len(),
                content
            );
            stream.write(res.as_bytes()).unwrap();

            stream.flush().unwrap();
        }
        _ => {
            https_status_code = "404 Not Found";

            print!("GET {} {}\n", route, https_status_code);

            let content = fs::read_to_string("public/404.html").unwrap();
            let res = format!(
                "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                &https_status_code,
                content.len(),
                content
            );
            stream.write(res.as_bytes()).unwrap();

            stream.flush().unwrap();
        }
    }

    // let mut https_status_code = "";

    // if status_code == 200 {
    //     https_status_code = "200 OK";
    // }
    // if status_code == 400 {
    //     https_status_code = "400 Bad Request";
    // }
    // if status_code == 404 {
    //     https_status_code = "404 Not Found";
    // }
}
