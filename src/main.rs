use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener,TcpStream},
    thread,
    time::Duration,
};

use web_server_rust::ThreadPool;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // cheklangan ip yozamiz so'rovga javob berish uchun
    let pool = ThreadPool::new(4)

    for stream in listener.incoming(){
        let stream = stream.unwrap();
// har bir so'rov uchun javob beruvchi ip yozamiz
// har bir so'rovga bitta ip javo beradi so'rovlar ko'paysa server qotadi

        // thread::spawn(||{
        //     handle_connection(stream);
        // });

//buni to'grilaymiz cheklangan ip yozamiz
            pool.execute(||{
                handle_connection(stream);
            })
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
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
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();


    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write_all(response.as_bytes()).unwrap();
    // let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("hello.html").unwrap();
    // let lenght = contents.len();

    // let response =
    //     format!("{status_line}\r\nContent-lenght: {lenght}\r\n\r\n{contents}");
    // stream.write_all(response.as_bytes()).unwrap();


}