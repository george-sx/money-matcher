use crate::MOLDPRODUCER;
use std::io;
use tokio::net::UdpSocket;

impl MOLDPRODUCER {
    pub async fn initialize(bind_addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        println!("Initialized MOLDPRODUCER on {}", bind_addr);
        Ok(MOLDPRODUCER { socket })
    }

    pub async fn produce(&self, payload: &[u8], addr: std::net::SocketAddr) -> io::Result<usize> {
        let len = self.socket.send_to(payload, addr).await?;
        println!("{:?} bytes sent to {:?}", len, addr);
        Ok(len)
    }
}
