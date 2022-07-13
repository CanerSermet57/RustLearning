fn main() {
    //Slice Tests
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("{}",slice[0] );

    //array and function test
    let mut arr = [3,5,6];
    uupdate(&mut arr[1..3]);
    println!("{:?}",arr);

    //Struct Tests
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(20, 10, 30);
    let origin = Point(0, 0, 0);
    println!("{}",black.0);
}

fn uupdate(arr:&mut [i32]){
    for i in 0..2
    {
        arr[i]=arr[i]+2;
        println!("{}",arr[i]);
    }
    println!("{:?}",arr);
}
