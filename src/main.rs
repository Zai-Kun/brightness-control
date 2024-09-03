mod gammastep_wrapper;
use gammastep_wrapper::Gammastep;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:53482").await.unwrap();
    let mut gammastep = Gammastep::new();

    println!("{}", gammastep.current_state());

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader
            .lines()
            .next_line()
            .await
            .expect("ERROR reading line")
            .unwrap();

        match request_line.trim() {
            "GET /inc HTTP/1.1" => {
                gammastep.update(10, true);
                gammastep.restart_gammastep();
                let resposne = make_response("", "HTTP/1.1 200 OK");
                stream.write_all(resposne.as_bytes()).await.unwrap();
            }
            "GET /dec HTTP/1.1" => {
                gammastep.update(10, false);
                gammastep.restart_gammastep();
                let resposne = make_response("", "HTTP/1.1 200 OK");
                stream.write_all(resposne.as_bytes()).await.unwrap();
            }
            _ => {
                let resposne = make_response("", "HTTP/1.1 404 Not Found");
                stream.write_all(resposne.as_bytes()).await.unwrap();
            }
        }
        println!("{}", gammastep.current_state());
    }
}

fn make_response(content: &str, status_line: &str) -> String {
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    )
}
