use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, Instant},
    io,
    sync::atomic::{AtomicUsize, Ordering},
};

// Contador atômico de pacotes
static PACKET_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() -> io::Result<()> {
    // Configuração do socket
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    socket.set_nonblocking(false)?; // Comportamento bloqueante
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    socket.set_write_timeout(Some(Duration::from_secs(5)))?;
    
    println!("🦀 Servidor UDP escutando em 127.0.0.1:8080");
    println!("📊 Modo: Bloqueante | Timeout: 5s");

    let mut buffer = [0u8; 1024]; // Buffer de 1KB
    let mut stats = ServerStats::new();

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((bytes_read, addr)) => {
                PACKET_COUNT.fetch_add(1, Ordering::SeqCst);
                stats.update(bytes_read);
                
                let received_data = &buffer[..bytes_read];
                println!("📥 Pacote #{} de {}: {} bytes",
                    PACKET_COUNT.load(Ordering::SeqCst),
                    addr,
                    bytes_read
                );

                // Processamento do pacote
                if let Err(e) = handle_packet(&socket, received_data, addr) {
                    eprintln!("❌ Erro ao processar pacote: {}", e);
                }
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Timeout ocorreu
                println!("⏳ Aguardando pacotes...");
                continue;
            }
            Err(e) => {
                eprintln!("🔥 Erro fatal: {}", e);
                break;
            }
        }
    }

    println!("📊 Estatísticas finais:\n{}", stats);
    Ok(())
}

/// Manipulador de pacotes UDP
fn handle_packet(socket: &UdpSocket, data: &[u8], addr: SocketAddr) -> io::Result<()> {
    // Simples eco com resposta personalizada
    let response = match data {
        b"PING" => b"PONG",
        b"TIME" => {
            let time = chrono::Local::now().format("%H:%M:%S").to_string();
            time.as_bytes()
        },
        _ => b"Oi, UDP! (resposta padrao)"
    };

    socket.send_to(response, addr)?;
    Ok(())
}

/// Estrutura para estatísticas do servidor
struct ServerStats {
    start_time: Instant,
    total_bytes: usize,
    packets: usize,
}

impl ServerStats {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_bytes: 0,
            packets: 0,
        }
    }

    fn update(&mut self, bytes: usize) {
        self.total_bytes += bytes;
        self.packets += 1;
    }

    fn bytes_per_second(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        self.total_bytes as f64 / elapsed
    }
}

impl std::fmt::Display for ServerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "⏱️ Tempo de atividade: {:.2}s\n\
             📦 Pacotes recebidos: {}\n\
             📊 Bytes totais: {}\n\
             🚀 Taxa média: {:.2} KB/s",
            self.start_time.elapsed().as_secs_f64(),
            self.packets,
            self.total_bytes,
            self.bytes_per_second() / 1024.0
        )
    }
}