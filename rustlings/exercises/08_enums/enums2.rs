#[derive(Debug)]
enum Message{
    
    Resize{width: u64 , height: u64 },
   
    Move{x: u64, y: u64},

    Echo(String),

    ChangeColor(u64,u64,u64),

    Quit
}


impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move{ x: 10, y: 15 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

