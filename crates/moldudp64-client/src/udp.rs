use crate::MOLDCONSUMER;
use tokio::net::UdpSocket;
use std::io;

impl MOLDCONSUMER {
	pub async fn initialize(bind_addr: &str) -> io::Result<Self> {
		let socket = UdpSocket::bind(bind_addr).await?;
		println!("Initialized MOLDCONSUMER on {}", bind_addr);
		Ok(MOLDCONSUMER { socket })
	}

	pub async fn consume(&self) -> io::Result<()> {
		let mut buf = [0; 1024];
		loop {
			let (len, addr) = self.socket.recv_from(&mut buf).await?;
			println!("{:?} bytes received from {:?}", len, addr);
		}
    }

}