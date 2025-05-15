use std::{
    net::{TcpListener, TcpStream},
    io::{Read, Write},
    thread,
    time::{Duration, Instant},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    collections::HashMap,
};

// Contadores atômicos
static CONNECTION_COUNT: AtomicUsize = AtomicUsize::new(0);
static TOTAL_BYTES: AtomicUsize = AtomicUsize::new(0);

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    listener.set_nonblocking(false)?;
    println!("🦀 Servidor TCP escutando em 127.0.0.1:8080");

    // Shared state para estatísticas
    let stats = Arc::new(ServerStats::new());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Configurações da conexão
                stream.set_read_timeout(Some(Duration::from_secs(10)))?;
                stream.set_write_timeout(Some(Duration::from_secs(10)))?;
                stream.set_nodelay(true)?;

                let stats = Arc::clone(&stats);
                thread::spawn(move || {
                    let conn_id = CONNECTION_COUNT.fetch_add(1, Ordering::SeqCst);
                    let peer_addr = stream.peer_addr().unwrap();
                    println!("🔌 Nova conexão #{} de {}", conn_id, peer_addr);

                    if let Err(e) = handle_connection(stream, conn_id, stats) {
                        eprintln!("❌ Erro na conexão {}: {}", conn_id, e);
                    }
                });
            }
            Err(e) => eprintln!("🔥 Erro ao aceitar conexão: {}", e),
        }
    }

    println!("📊 Estatísticas finais:\n{}", stats);
    Ok(())
}

/// Manipulador de conexões TCP
fn handle_connection(
    mut stream: TcpStream,
    conn_id: usize,
    stats: Arc<ServerStats>,
) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    let mut request_history = Vec::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("🚪 Conexão #{} encerrada pelo cliente", conn_id);
                break;
            }
            Ok(bytes_read) => {
                TOTAL_BYTES.fetch_add(bytes_read, Ordering::SeqCst);
                stats.record_request(bytes_read);

                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                request_history.push(request.to_string());
                
                println!(
                    "📥 Conexão #{}: {} bytes\n{}",
                    conn_id,
                    bytes_read,
                    request
                );

                // Processamento da requisição
                let response = process_request(&request);
                stream.write_all(response.as_bytes())?;
                stream.flush()?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                println!("⏳ Conexão #{} inativa (timeout)", conn_id);
                break;
            }
            Err(e) => {
                eprintln!("⚠️ Erro na conexão #{}: {}", conn_id, e);
                break;
            }
        }
    }

    println!("📝 Histórico da conexão #{}:\n{:#?}", conn_id, request_history);
    Ok(())
}

/// Processador de requisições TCP
fn process_request(request: &str) -> String {
    let request = request.trim();
    match request {
        "PING" => "PONG\n".to_string(),
        "TIME" => {
            let time = chrono::Local::now().format("%H:%M:%S").to_string();
            format!("{}\n", time)
        },
        "STATS" => "Servidor em operação\n".to_string(),
        _ => format!("Comando não reconhecido: '{}'\n", request)
    }
}

/// Estrutura para estatísticas do servidor
struct ServerStats {
    start_time: Instant,
    requests: AtomicUsize,
    bytes_received: AtomicUsize,
    active_connections: AtomicUsize,
    endpoint_stats: Arc<parking_lot::Mutex<HashMap<String, usize>>>,
}

impl ServerStats {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            requests: AtomicUsize::new(0),
            bytes_received: AtomicUsize::new(0),
            active_connections: AtomicUsize::new(0),
            endpoint_stats: Arc::new(parking_lot::Mutex::new(HashMap::new())),
        }
    }

    fn record_request(&self, bytes: usize) {
        self.requests.fetch_add(1, Ordering::SeqCst);
        self.bytes_received.fetch_add(bytes, Ordering::SeqCst);
    }

    fn requests_per_second(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        self.requests.load(Ordering::SeqCst) as f64 / elapsed
    }

    fn bytes_per_second(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        self.bytes_received.load(Ordering::SeqCst) as f64 / elapsed
    }
}

impl std::fmt::Display for ServerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "⏱️ Tempo de atividade: {:.2}s\n\
             📦 Conexões totais: {}\n\
             📊 Bytes recebidos: {}\n\
             📡 Requisições: {}\n\
             🚀 Taxa de requisições: {:.2}/s\n\
             💾 Taxa de dados: {:.2} KB/s",
            self.start_time.elapsed().as_secs_f64(),
            CONNECTION_COUNT.load(Ordering::SeqCst),
            self.bytes_received.load(Ordering::SeqCst),
            self.requests.load(Ordering::SeqCst),
            self.requests_per_second(),
            self.bytes_per_second() / 1024.0
        )
    }
}
