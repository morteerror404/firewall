use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::Mutex,
};

struct Lateralizador {
    tcp_listener: TcpListener,
    udp_socket: UdpSocket,
    tunnels: Arc<Mutex<Vec<ActiveTunnel>>>,
    policies: Policies,
}

impl Lateralizador {
    pub async fn new() -> Self {
        let policies = load_policies();
        let tcp_listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
        let udp_socket = UdpSocket::bind("0.0.0.0:8081").await.unwrap();
        
        Self {
            tcp_listener,
            udp_socket,
            tunnels: Arc::new(Mutex::new(Vec::new())),
            policies,
        }
    }

    pub async fn run(&self) {
        loop {
            tokio::select! {
                Ok((tcp_stream, addr)) = self.tcp_listener.accept() => {
                    self.handle_tcp(tcp_stream, addr).await;
                },
                Ok((udp_data, addr)) = self.udp_socket.recv_from() => {
                    self.handle_udp(udp_data, addr).await;
                }
            }
        }
    }

    async fn handle_tcp(&self, stream: TcpStream, addr: SocketAddr) {
        let tunnel = match self.should_redirect(&addr, Protocol::Tcp) {
            Some(target) => create_tunnel(stream, target).await,
            None => return,
        };
        
        let mut tunnels = self.tunnels.lock().await;
        tunnels.push(tunnel);
    }

    async fn handle_udp(&self, data: Vec<u8>, addr: SocketAddr) {
        if self.should_redirect(&addr, Protocol::Udp).is_some() {
            // Implementar lógica UDP similar
        }
    }
}