use std::io::{stdin,stdout,Write};
use std::io::{self, BufRead};
fn main() {
    //clearscreen::clear().unwrap();
    let mut firstline = vec![String::new(),String::new(),String::new()];
    let mut firstcharacter = vec!["1","2","3","4","5","6","7","8","9"];
    cube_inside_character(&firstcharacter, &mut firstline);

    for i in 0..3 {
        println!("{}",firstline[i]);
    }

    println!("\nWelcome to the X-O game. This game is played with 2 people.
According to the number order you see above, you need to write in which position you will put X or O.
first player controls X shapes and second player controls O shapes. Have a Nice Game\n
Enter any character to start the game");
    let mut inputtext= String::new();
    //stdin().read_line(&mut s).expect("Did not enter a correct string");
    clearscreen::clear().unwrap();
    let mut firstplayerturn = true;
    let mut error = String::new();
    let mut playershape = "X";
    let mut firstcharacter = vec![" "," "," "," "," "," "," "," "," "];
    while true {
        clearscreen::clear().unwrap();
        let mut firstline = vec![String::new(),String::new(),String::new()];
        cube_inside_character(&firstcharacter, &mut firstline);
        for i in 0..3 {
            println!("{}",firstline[i]);
        }

        if firstplayerturn {
            println!("First Player Please Make Your Move (X)      {}",error);
            playershape = "X";
        }
        else {
            println!("Second Player Please Make Your Move (O)");
            playershape = "O";
        }
        inputtext = get_input();
        if inputtext == "end" { break;}

        //Number Or Not Check
        let mut number = 0;
        number = match inputtext.parse::<i32>() {
            Ok(n) => (n),
            Err(e) => (-1) ,
        };
        if number == -1 {
            error = String::from("Please Write Number");
            continue;
        }
        if number<10 && number>0 && firstcharacter[(number-1) as usize] == " "
        {
            firstcharacter[(number-1) as usize] = playershape;
            if firstplayerturn { firstplayerturn = false; }
            else { firstplayerturn = true; }
        }



    }

}

fn get_input() -> String {
    let mut inputtext= String::new();
    let _=stdout().flush();
    stdin().read_line(&mut inputtext).expect("Did not enter a correct string");
    if let Some('\n')=inputtext.chars().next_back() {
        inputtext.pop();
    }
    if let Some('\r')=inputtext.chars().next_back() {
        inputtext.pop();
    }
    return inputtext;
}

fn cube_inside_character(character: &Vec<&str>, line: &mut Vec<String>) {
    let mut count = 0;
    for i in 0..3 {
        for j in 0..3 {
            line[i] = line[i].clone() + "|" + character[count].clone() + "| ";
            count = count+1;
        }
    }
}