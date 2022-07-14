use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;
use std::io::{self, Read};

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

    //Match Test
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}


    //VEC Test
    let v = vec![1, 2, 3, 4, 5];

    let third = v[2];
    println!("The third element is {}", third);

    match v.get(1) {
        Some(f) => println!("The third element is {}", f),
        None => println!("There is no third element."),
    }


    let mut v = vec!["test","test2"];

    let first = &v[0];

    v.push("test3");

    println!("The first element is: {:?}", v);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }


    let hello = "Здaравствуйте";

    let s = &hello[0..5];

    println!("Test {}",s);

        use std::collections::HashMap;

        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            println!("count: {:?}",&map);
            let count = dbg!(map.entry(word).or_insert(5));
            *count += 10;
        }

        println!("{:?}", map);

    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(fd) => fd,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut s = String::new();

     let mut st = match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };

}

fn uupdate(arr:&mut [i32]){
    for i in 0..2
    {
        arr[i]=arr[i]+2;
        println!("{}",arr[i]);
    }
    println!("{:?}",arr);
}
