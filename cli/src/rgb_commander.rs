use std::net::SocketAddr;
use std::net::UdpSocket;

type RgbSendResult = std::io::Result<usize>;

pub struct RgbCommander {
  address: SocketAddr,
  socket: UdpSocket,
}

type Color = (u8, u8, u8);

impl RgbCommander {
  pub fn connect(address: SocketAddr) -> Result<RgbCommander, std::io::Error> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", address.port()))?;
    socket.set_broadcast(true)?;
    Ok(RgbCommander { socket, address })
  }

  pub fn send_raw(&self, payload: &[u8]) -> RgbSendResult {
    self.socket.send_to(payload, self.address)
  }

  pub fn set_color(&self, color: Color) -> RgbSendResult {
    self.send_raw(format!("00C{:0>3}{:0>3}{:0>3}", color.0, color.1, color.2).as_bytes())
  }
}
