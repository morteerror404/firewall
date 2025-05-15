use std::{
    net::{UdpSocket, SocketAddr},
    time::{Instant, Duration},
};

/// Gera um pacote UDP simulando uma requisição GET
pub fn generate_udp_get_request(host: &str, path: &str) -> Vec<u8> {
    // Formato: [GET][path][\0][host][\0]
    let mut packet = Vec::with_capacity(3 + path.len() + 1 + host.len() + 1);
    packet.extend_from_slice(b"GET");
    packet.extend_from_slice(path.as_bytes());
    packet.push(0);
    packet.extend_from_slice(host.as_bytes());
    packet.push(0);
    packet
}

/// Analisa um pacote UDP bit a bit
pub fn analyze_udp_packet(packet: &[u8], received_from: SocketAddr) {
    println!("🔍 Análise Detalhada do Pacote UDP ({} bytes)", packet.len());
    println!("{:-<60}", "");
    println!("🌐 Origem: {}", received_from);
    
    // Análise do cabeçalho (simulado)
    if packet.len() >= 8 {
        println!("\n📦 Cabeçalho Simulado:");
        println!("   Comprimento: {} bytes", packet.len());
        println!("   Checksum: 0x{:04X}", u16::from_be_bytes([packet[6], packet[7]]));
    }

    // Análise dos dados
    println!("\n📊 Dados do Pacote:");
    print_multi_format(packet);
    
    // Tentativa de interpretar como GET
    if packet.starts_with(b"GET") {
        if let Some(pos) = packet.iter().position(|&b| b == 0) {
            let path = String::from_utf8_lossy(&packet[3..pos]);
            let host = if packet.len() > pos + 1 {
                String::from_utf8_lossy(&packet[pos+1..]).split('\0').next().unwrap()
            } else {
                "".into()
            };
            println!("\n🔎 Interpretado como GET:");
            println!("   Path: {}", path);
            println!("   Host: {}", host);
        }
    }
}

/// Executa uma requisição UDP GET e analisa a resposta
pub fn execute_udp_get(
    server_addr: &str,
    host: &str,
    path: &str,
    timeout_secs: u64
) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(timeout_secs)))?;
    
    let server_addr: SocketAddr = server_addr.parse()?;
    let request = generate_udp_get_request(host, path);
    
    println!("🚀 Enviando UDP GET para {} ({} bytes)", server_addr, request.len());
    socket.send_to(&request, server_addr)?;
    
    let mut buf = [0u8; 1024];
    let start_time = Instant::now();
    
    match socket.recv_from(&mut buf) {
        Ok((bytes_received, src_addr)) => {
            println!("\n📥 Resposta recebida em {:?}", start_time.elapsed());
            analyze_udp_packet(&buf[..bytes_received], src_addr);
        }
        Err(e) => eprintln!("❌ Erro ao receber resposta: {}", e),
    }
    
    Ok(())
}

fn print_multi_format(data: &[u8]) {
    println!("{:8}   {:32}   {:10}   {}", "Offset", "Hexadecimal", "Decimal", "ASCII");
    println!("{:-<8}   {:-<32}   {:-<10}   {:-<16}", "", "", "", "");
    
    for (i, chunk) in data.chunks(8).enumerate() {
        let offset = i * 8;
        let hex = chunk.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        let dec = chunk.iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let ascii = chunk.iter()
            .map(|&b| if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' })
            .collect::<String>();
        
        println!("{:08X}   {:32}   {:10}   {}", offset, hex, dec, ascii);
    }
}