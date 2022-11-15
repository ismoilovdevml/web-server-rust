use web_server_rust::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // cheklangan ip yozamiz so'rovga javob berish uchun
    let pool = ThreadPool::new(10);

    for stream in listener.incoming().take(5){
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Oʻchirish.")
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("index.html").unwrap();
        let lenght = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Lenght: {lenght}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
        
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let lenght = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Lenght: {lenght}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }

    // Web Server ko'p tarmoqli qilish 
    // Server hamma so'rovlarga javob berishi uchun kod yozamiz
    // 2 ta brazuzer oyna oching va bittasiga 127.0.0.1:7878 keyingisiga 127.0.0.1:7878/sleep ni yozing
    // 2 chisini brazuer oynasini refresh bering server 5 soniyaga to'xtagani ko'rasiz

    let (status_line, filename) = match &request_line[..]{
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
}