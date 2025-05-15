use std::{
    net::{UdpSocket, SocketAddr},
    time::{Instant, Duration},
};

/// Gera um pacote UDP simulando uma requisição POST
pub fn generate_udp_post_request(host: &str, path: &str, data: &str) -> Vec<u8> {
    // Formato: [POST][path][\0][host][\0][data]
    let mut packet = Vec::with_capacity(4 + path.len() + 1 + host.len() + 1 + data.len());
    packet.extend_from_slice(b"POST");
    packet.extend_from_slice(path.as_bytes());
    packet.push(0);
    packet.extend_from_slice(host.as_bytes());
    packet.push(0);
    packet.extend_from_slice(data.as_bytes());
    packet
}

/// Analisa um pacote UDP POST em detalhes
pub fn analyze_udp_post_packet(packet: &[u8], received_from: SocketAddr) {
    println!("🔍 Análise Detalhada do Pacote UDP POST ({} bytes)", packet.len());
    println!("{:-<60}", "");
    println!("🌐 Origem: {}", received_from);
    
    // Análise estrutural
    if packet.len() >= 5 && packet.starts_with(b"POST") {
        let mut parts = packet[4..].split(|&b| b == 0);
        let path = parts.next().map(|p| String::from_utf8_lossy(p)).unwrap_or_default();
        let host = parts.next().map(|h| String::from_utf8_lossy(h)).unwrap_or_default();
        let data = parts.next().unwrap_or_default();
        
        println!("\n📝 Estrutura POST:");
        println!("   Path: {}", path);
        println!("   Host: {}", host);
        println!("   Data: {} bytes", data.len());
        
        // Análise de dados binários
        if !data.is_empty() {
            println!("\n📊 Conteúdo dos Dados:");
            print_binary_analysis(data);
        }
    } else {
        println!("⚠️ Pacote não reconhecido como POST");
        print_multi_format(packet);
    }
}

/// Executa uma requisição UDP POST e analisa a resposta
pub fn execute_udp_post(
    server_addr: &str,
    host: &str,
    path: &str,
    data: &str,
    timeout_secs: u64
) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(timeout_secs)))?;
    
    let server_addr: SocketAddr = server_addr.parse()?;
    let request = generate_udp_post_request(host, path, data);
    
    println!("🚀 Enviando UDP POST para {} ({} bytes)", server_addr, request.len());
    socket.send_to(&request, server_addr)?;
    
    let mut buf = [0u8; 1024];
    let start_time = Instant::now();
    
    match socket.recv_from(&mut buf) {
        Ok((bytes_received, src_addr)) => {
            println!("\n📥 Resposta recebida em {:?}", start_time.elapsed());
            analyze_udp_post_packet(&buf[..bytes_received], src_addr);
        }
        Err(e) => eprintln!("❌ Erro ao receber resposta: {}", e),
    }
    
    Ok(())
}

fn print_binary_analysis(data: &[u8]) {
    println!("{:8}   {:16}   {:8}   {:8}   {}", 
        "Offset", "Binário", "Hex", "Decimal", "ASCII");
    println!("{:-<8}   {:-<16}   {:-<8}   {:-<8}   {:-<16}", 
        "", "", "", "", "");
    
    for (i, &byte) in data.iter().enumerate() {
        println!("{:08X}   {:08b}   {:02X}   {:8}   {}", 
            i, byte, byte, byte, 
            if byte.is_ascii_graphic() || byte == b' ' { byte as char } else { '.' });
    }
}

// Função compartilhada com GET.rs
fn print_multi_format(data: &[u8]) {
    /* mesma implementação que em GET.rs */
}