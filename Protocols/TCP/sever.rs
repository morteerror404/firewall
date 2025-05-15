use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Servidor escutando em 127.0.0.1:8080");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        println!("Recebido: {}", String::from_utf8_lossy(&buffer));
        stream.write_all(b"Mensagem recebida!")?;
    }
    Ok(())
}