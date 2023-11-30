// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit,                           // 没有关联数据
    Echo(String),                   // 关联一个字符串
    Move { x: i32, y: i32 },        // 包含一个匿名结构体
    ChangeColor(i32, i32, i32),     // 关联三个 i32 类型的值
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("Hello world".to_string()));
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::ChangeColor(255, 255, 255));
}