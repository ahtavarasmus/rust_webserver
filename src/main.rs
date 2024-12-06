use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // bind to localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        let mut stream= stream?;

        // read the request
        let mut buffer = [0;1024];
        let bytes_read = stream.read(&mut buffer)?;

        // let's print the request out
        let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Request:\n{}", request_str);

        // construct a simple http response
        let response_body = "Hello from rust server!";
        let response = format!(
            "HTTP/1.1 200 OK\r\n
            Content-Length: {}\r\n
            Content-Type: text/plain\r\n
            Connection: close\r\n
            \r\n
            {}",
            response_body.len(),
            response_body
        );

        // write the response back to the stream
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}
