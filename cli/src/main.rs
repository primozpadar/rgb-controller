use std::net::UdpSocket;
use std::env;

const BROADCAST_ADDR: &str = "192.168.1.255:50000";
const BIND_ADDR: &str = "0.0.0.0:50000";

fn main() {
    let socket = UdpSocket::bind(BIND_ADDR).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("couldn't set broadcast");
    
    let args: Vec<String> = env::args().collect();
    
    let r = args.get(1).expect("Red missing");
    let g = args.get(1).expect("Green missing");
    let b = args.get(1).expect("Blue missing");
    
    let msg = get_color_msg(r, g, b);

    socket.send_to(msg.as_bytes(), BROADCAST_ADDR).unwrap();
}

fn get_color_msg(r: &str, g: &str, b: &str) -> String {
    format!("C{:0>3}{:0>3}{:0>3}", r, g, b)
}
