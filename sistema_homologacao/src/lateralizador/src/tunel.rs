use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct Tunnel {
    pub id: Uuid,
    pub src: SocketAddr,
    pub dest: SocketAddr,
    pub protocol: Protocol,
    pub created_at: DateTime<Utc>,
}

pub async fn create_tunnel(
    src_stream: TcpStream,
    dest_addr: SocketAddr,
) -> Result<Tunnel, TunnelError> {
    let dest_stream = TcpStream::connect(dest_addr).await?;
    let src_addr = src_stream.peer_addr()?;
    let id = Uuid::new_v4();
    
    tokio::spawn(async move {
        let (mut src_reader, mut src_writer) = src_stream.into_split();
        let (mut dest_reader, mut dest_writer) = dest_stream.into_split();
        
        let client_to_server = tokio::io::copy(&mut src_reader, &mut dest_writer);
        let server_to_client = tokio::io::copy(&mut dest_reader, &mut src_writer);
        
        tokio::try_join!(client_to_server, server_to_client)?;
        Ok::<_, TunnelError>(())
    });
    
    Ok(Tunnel {
        id,
        src: src_addr,
        dest: dest_addr,
        protocol: Protocol::Tcp,
        created_at: Utc::now(),
    })
}