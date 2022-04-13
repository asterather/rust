use std::io;
mod prompts;

/*fn prompt() -> bool {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .unwrap();

    return input.to_ascii_lowercase().starts_with("b");
}*/

fn main(){
    println!("test");
   let mut test = String::new();
   
    io::stdin()
        .read_line(&mut test)
        .expect("Nope");

   let mut test: u32 = test.trim().parse().expect("whoops");
    //println!("yo, that's {}", test);
    //let test = prompt();

    if test == 25 {
        println!("woot!");
    }else{
        println!("{}?That's not a thing", test);
    }
    println!("Choose red or blue");

    let mut test2 = String::new();
 
   io::stdin()
        .read_line(&mut test2)
        .expect("Not here");

    if test2 == "blue\n" { 
        println!("So you have chosen Bloooo!");
        println!("You have chosen wisely.\nWhat is your name bloooooo chooser?");
        
        let mut name = String::new();

        io::stdin()
        .read_line(&mut name)
        .expect("nope, name");
        
        println!("I am not worthy, {}", name);
        println!("I have a secret number, 
if my number and your number make the correct number,\nyou may continue.");
        println!("What is your number?");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("na, num");
        
        let num: u32 = num.trim().parse().expect("no, that's a nah not num");

        println!("You guessed {}", num);
        println!("with my number that is {}", num + 7);


    }else if test2 == "red\n" {
        println!("red. how delightful");
        }else {
            println!("nope.");
        }
}
