use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("Escutando em 127.0.0.1:8080");

    let mut buffer = [0; 1024];
    let (bytes_read, addr) = socket.recv_from(&mut buffer)?;
    println!("Recebido: {} de {}", String::from_utf8_lossy(&buffer[..bytes_read]), addr);
    socket.send_to(b"Oi, UDP!", addr)?;
    Ok(())
}

