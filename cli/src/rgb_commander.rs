use std::net::SocketAddr;
use std::net::UdpSocket;

use crate::utils::Color;

type RgbSendResult = std::io::Result<usize>;

pub struct RgbCommander {
    address: SocketAddr,
    default_channel: u8,
    socket: UdpSocket,
}

impl RgbCommander {
    pub fn connect(address: SocketAddr, default_channel: u8) -> Result<RgbCommander, std::io::Error> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", address.port()))?;
        socket.set_broadcast(true)?;
        Ok(RgbCommander {
            socket,
            address,
            default_channel,
        })
    }

    pub fn set_color(&self, color: &Color, channel: Option<u8>) -> RgbSendResult {
        let send_to = channel.unwrap_or(self.default_channel);
        self.socket.send_to(&[send_to, color.r, color.g, color.b], self.address)
    }
}
