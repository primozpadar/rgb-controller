use std::net::SocketAddr;
use std::net::UdpSocket;

type RgbSendResult = std::io::Result<usize>;

pub struct RgbCommander {
    address: SocketAddr,
    default_device_id: String,
    socket: UdpSocket,
}

type Color = (u8, u8, u8);

impl RgbCommander {
    pub fn connect(address: SocketAddr, default_device_id: String) -> Result<RgbCommander, std::io::Error> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", address.port()))?;
        socket.set_broadcast(true)?;
        Ok(RgbCommander {
            socket,
            address,
            default_device_id,
        })
    }

    pub fn send_raw(&self, payload: &[u8]) -> RgbSendResult {
        self.socket.send_to(payload, self.address)
    }

    pub fn set_color(&self, color: Color) -> RgbSendResult {
        println!("{:0>2}C{:0>3}{:0>3}{:0>3}", self.default_device_id, color.0, color.1, color.2);
        self.send_raw(format!("{:0>2}C{:0>3}{:0>3}{:0>3}", self.default_device_id, color.0, color.1, color.2).as_bytes())
    }
}
