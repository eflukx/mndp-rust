use mndp::Packet;
use std::{io, net::UdpSocket};

fn main() -> io::Result<()> {
    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:5678")?;
    println!(
        "Listening for MNDP (UDP) broadcasts on {} . . .",
        socket.local_addr()?
    );

    // Create a buffer to receive data
    let mut buf = [0u8; 2048];
    let mut success_count = 0;

    loop {
        // Receive a datagram
        let (len, src_addr) = socket.recv_from(&mut buf)?;
        let bytes = Vec::from(&buf[..len]);

        match Packet::from_bytes(bytes) {
            Ok(p) => {
                success_count += 1;
                println!(
                    "Decoded packet from {src_addr} (len {len}, packet {success_count})\n{:?}",
                    p.to_neighbor()
                )
            }
            Err(e) => println!("BOOM, decoding packet failed! : {e:?}"),
        }

        println!();
    }
}
