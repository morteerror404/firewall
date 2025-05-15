use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

/// Gera uma requisição HTTP POST completa
pub fn generate_post_request(host: &str, path: &str, data: &str) -> String {
    format!(
        "POST {} HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: Rust-TCP-Analyzer\r\n\
         Content-Type: application/x-www-form-urlencoded\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        path, host, data.len(), data
    )
}

/// Analisa um pacote TCP bit a bit (versão avançada)
pub fn analyze_packet(packet: &[u8]) {
    println!("🔍 Análise Detalhada do Pacote ({} bytes)", packet.len());
    println!("{:-<60}", "");

    // Análise de flags TCP
    if packet.len() >= 14 {
        let flags = packet[13];
        println!("🚩 Flags TCP:");
        println!("   URG: {}", (flags & 0b00100000) != 0);
        println!("   ACK: {}", (flags & 0b00010000) != 0);
        println!("   PSH: {}", (flags & 0b00001000) != 0);
        println!("   RST: {}", (flags & 0b00000100) != 0);
        println!("   SYN: {}", (flags & 0b00000010) != 0);
        println!("   FIN: {}", (flags & 0b00000001) != 0);
    }

    // Dados HTTP
    if packet.len() > 20 {
        let http_data = &packet[20..];
        println!("\n📄 Dados HTTP:");
        print_ascii_with_offsets(http_data);
    }
}

/// Executa uma requisição POST e analisa a resposta
pub fn execute_post(host: &str, port: u16, path: &str, data: &str) -> std::io::Result<()> {
    let request = generate_post_request(host, path, data);
    let mut stream = TcpStream::connect((host, port))?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    println!("🚀 Enviando POST para {}{}", host, path);
    stream.write_all(request.as_bytes())?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    println!("\n📥 Resposta recebida:");
    analyze_packet(&response);

    Ok(())
}

// Funções auxiliares avançadas
fn print_ascii_with_offsets(data: &[u8]) {
    println!("{:8}   {:48}   {}", "Offset", "Hexadecimal", "ASCII");
    println!("{:-<8}   {:-<48}   {:-<16}", "", "", "");

    for (i, chunk) in data.chunks(16).enumerate() {
        let offset = i * 16;
        let hex: String = chunk.iter()
            .map(|b| format!("{:02X} ", b))
            .collect();
        let ascii: String = chunk.iter()
            .map(|&b| if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' })
            .collect();

        println!("{:08X}   {:48}   {}", offset, hex, ascii);
    }
}