mod rectangle;
mod packet;

fn main() {
    // Creates a new packet with the following information and prints them
    packet::create(String::from("Handshake"), 47, 32, packet::PacketType::CLIENT);

    /*
     * Creates a new Rectangle with the information provided and prints them
     * plus the Area, Perimeter and Diagonal of the Rectangle
    */
    rectangle::create(25.0, 10.0);
}