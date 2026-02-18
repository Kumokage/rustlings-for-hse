#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize = -1,
    Move = 0,
    Echo = 1,
    ChangeColor = 5,
    Quit
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
