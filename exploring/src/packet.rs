#[warn(dead_code)]

#[derive(Copy, Clone)]
pub enum PacketType {
    SERVER,
    CLIENT
}

pub struct Packet {
    name: String,
    protocol: i32,
    length: i32,
    info: PacketType
}

pub fn create(name: String, protocol: i32, length: i32, info: PacketType) {
    let handshake_packet = Packet { name, protocol, length, info};

    println!("Created new packet! The following values have been received: ");
    println!("Name: {}", handshake_packet.name);
    println!("Protocol: {}", handshake_packet.protocol);
    println!("Length: {}", handshake_packet.length);
    
    match info {
        PacketType::SERVER => println!("Info: SERVER"),
        PacketType::CLIENT => println!("Info: CLIENT")  
    }
}