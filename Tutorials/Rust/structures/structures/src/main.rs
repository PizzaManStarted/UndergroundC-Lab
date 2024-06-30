struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let black = Color(0, 0, 0);
    let origin:  Point = Point(0, 0, 0);
    
    println!("Access data with a . and the index: {}", black.0);


    dbg!(3 +1  - 1 /2 );

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}