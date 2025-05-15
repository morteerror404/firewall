use anyhow::Result;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, UdpSocket},
    sync::Mutex,
    time,
};

// Constantes para configuração dos testes
const TEST_TCP_PORT: u16 = 58423;
const TEST_UDP_PORT: u16 = 58424;
const LOCALHOST: &str = "127.0.0.1";

lazy_static::lazy_static! {
    static ref SERVER_STARTED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    
    // Inicia os servidores em background
    start_servers().await?;
    
    // Executa bateria de testes
    run_tcp_tests().await?;
    run_udp_tests().await?;
    
    Ok(())
}

async fn start_servers() -> Result<()> {
    let mut started = SERVER_STARTED.lock().await;
    if *started {
        return Ok(());
    }
    
    tokio::spawn(async {
        let _ = firewall::Protocols::TCP::sever::start_server(TEST_TCP_PORT).await;
    });
    
    tokio::spawn(async {
        let _ = firewall::Protocols::UDP::sever::start_server(TEST_UDP_PORT).await;
    });
    
    // Espera os servidores inicializarem
    time::sleep(Duration::from_millis(500)).await;
    *started = true;
    
    Ok(())
}

async fn run_tcp_tests() -> Result<()> {
    log::info!("🚀 Iniciando testes TCP...");
    
    // Teste de conexão básica
    let addr = format!("{}:{}", LOCALHOST, TEST_TCP_PORT);
    let mut stream = TcpStream::connect(&addr).await?;
    
    // Teste de envio/recebimento
    let test_data = b"TEST_TCP";
    stream.write_all(test_data).await?;
    
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).await?;
    assert_eq!(&buf[..n], test_data);
    
    log::info!("✅ Testes TCP concluídos com sucesso");
    Ok(())
}

async fn run_udp_tests() -> Result<()> {
    log::info!("🚀 Iniciando testes UDP...");
    
    let client = UdpSocket::bind("127.0.0.1:0").await?;
    let server_addr = format!("{}:{}", LOCALHOST, TEST_UDP_PORT);
    
    // Teste de envio/recebimento
    let test_data = b"TEST_UDP";
    client.send_to(test_data, &server_addr).await?;
    
    let mut buf = [0u8; 1024];
    let (n, _) = client.recv_from(&mut buf).await?;
    assert_eq!(&buf[..n], test_data);
    
    // Teste de performance
    let start = time::Instant::now();
    for _ in 0..100 {
        client.send_to(test_data, &server_addr).await?;
        let _ = client.recv_from(&mut buf).await?;
    }
    log::info!("📊 100 pacotes UDP em {:?}", start.elapsed());
    
    log::info!("✅ Testes UDP concluídos com sucesso");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    
    #[rstest]
    #[tokio::test]
    async fn test_tcp_connection() {
        let addr = format!("{}:{}", LOCALHOST, TEST_TCP_PORT);
        assert!(TcpStream::connect(&addr).await.is_ok());
    }
    
    #[rstest]
    #[tokio::test]
    async fn test_udp_echo() {
        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let server_addr = format!("{}:{}", LOCALHOST, TEST_UDP_PORT);
        
        let test_data = b"UDP_TEST";
        client.send_to(test_data, &server_addr).await.unwrap();
        
        let mut buf = [0u8; 1024];
        let (n, _) = client.recv_from(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], test_data);
    }
}