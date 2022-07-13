fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: i32,
        height: i32,
        test:i32,
    }
    let rectangle= Rectangle{
        width: 32,
        height:32,
        test: 0,
    };

    println!(
        "The area of the rectangle with {:?} is {} square pixels.",
        rectangle,
        area(rectangle.width, rectangle.height)
    );
}

fn area(width: i32, height: i32) -> i32 {
    dbg!(width * height) //This can help us for check what this code doing without taking ownership
}
