use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

/// Gera uma requisição HTTP GET completa
pub fn generate_get_request(host: &str, path: &str) -> String {
    format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: Rust-TCP-Analyzer\r\n\
         Connection: close\r\n\
         \r\n",
        path, host
    )
}

/// Analisa um pacote TCP bit a bit
pub fn analyze_packet(packet: &[u8]) {
    println!("🔍 Análise Detalhada do Pacote ({} bytes)", packet.len());
    println!("{:-<60}", "");

    // Cabeçalho TCP (primeiros 20 bytes)
    if packet.len() >= 20 {
        let tcp_header = &packet[..20];
        println!("📦 Cabeçalho TCP:");
        print_hex(tcp_header);
        print_binary(tcp_header);
        print_decimal(tcp_header);
    }

    // Dados HTTP
    if packet.len() > 20 {
        let http_data = &packet[20..];
        println!("📄 Dados HTTP:");
        print_ascii(http_data);
    }
}

/// Executa uma requisição GET e analisa a resposta
pub fn execute_get(host: &str, port: u16, path: &str) -> std::io::Result<()> {
    let request = generate_get_request(host, path);
    let mut stream = TcpStream::connect((host, port))?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    println!("🚀 Enviando GET para {}{}", host, path);
    stream.write_all(request.as_bytes())?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    println!("\n📥 Resposta recebida:");
    analyze_packet(&response);

    Ok(())
}

// Funções auxiliares de análise
fn print_hex(data: &[u8]) {
    println!("🔢 Hexadecimal:");
    for chunk in data.chunks(16) {
        print!("   ");
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        println!();
    }
}

fn print_binary(data: &[u8]) {
    println!("🔣 Binário:");
    for byte in data {
        println!("   {:08b}", byte);
    }
}

fn print_decimal(data: &[u8]) {
    println!("🔢 Decimal:");
    for (i, byte) in data.iter().enumerate() {
        println!("   Byte {}: {}", i, byte);
    }
}

fn print_ascii(data: &[u8]) {
    println!("📝 ASCII:");
    println!("{}", String::from_utf8_lossy(data));
}