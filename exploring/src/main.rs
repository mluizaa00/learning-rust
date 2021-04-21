use rectangle::area;

mod rectangle;
mod packet;

fn main() {
    packet::create(String::from("Handshake"), 47, 32);
    rectangle::create(25.0, 10.0);
}