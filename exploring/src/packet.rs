pub struct Packet {
    name: String,
    protocol: i32,
    length: i32
}

pub fn create(name: String, protocol: i32, length: i32) {
    let handshake_packet = Packet { name, protocol, length};

    println!("Created new packet! The following values have been received: ");
    println!("Name: {}", handshake_packet.name);
    println!("Protocol: {}", handshake_packet.protocol);
    println!("Length: {}", handshake_packet.length);
}