use iced::widget::{button, column, text, Column};

enum Piece {
    Cross
    Circle
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PutCross(place: u8),
    PutCircle(place: u8),
    Restart,
}

struct State {
    value: Vec<Piece>
    winner: Option<Piece>
}   

fn main() {
    println!("Hello, world!");
}
