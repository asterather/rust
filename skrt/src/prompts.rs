use std::io;

fn prompt() -> bool{
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .unwrap();
    return input.to_ascii_lowercase().starts_with("y");
}
fn prompt2() -> bool{
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .unwrap();
    return input.to_ascii_lowercase().starts_with("h");
}
pub fn intro() -> (){
    println!("Is your name Stacy?");


    if prompt() {
        answered();
    }else{
        println!("AAAAAAAAAAAAAAAAAAHHHHHH");
    }
}
pub fn answered() -> () {
    println!("Are you THE Stacy??");
    if prompt() {
        challanged();
    }else{
        println!("ouch");
    }
}

pub fn challanged() -> () {
    println!("Dang, that's pretty cool.");
    println!("Ok, let's begin.




    You are walking down a street and you see a gray human figure rush into the nearby thick forest.
    Do you persue?");

    if prompt(){
        println!("Not really something to do alone, but ok.
        You run after the figure, in your excitement you bound past a tree.
        The last thing you remember are the vines coming alive.
        Good try.");
    }else{
        run();
    }
}

pub fn run() -> () {
           println!("Good, you wouldn't want to die so soon, eh?");
           println!("The figure is clearly gone, but you get the sense that someone is watching.");
           println!("Do you hide to see if someone comes by, or do you leave?");
           println!("choose hide or leave:");
           if prompt2(){
               println!("You were already being watched, they saw where you hid.
               You stay for hours, but see nothing expect an ant fighting a wasp, we get it, you were bored.");
               println!("The last thing you remember is a humming sound as your head left your body.");
           }else{
               leave();
           }
}
pub fn leave() -> () {
    println!("You quickly leave the area, but your haste continues to draw attention.
    You see no one, but you know for SURE that someone is watching you.");
    println!("As you rush away, you can hear a humming sound, like a happy man about to eat,
    but you can't place the sound.");
    println!("With nothing but woods on one side of the road,
    and a deep lake on the other side and only forboding houses for miles, you keep moving.
    Coming up the road, you see a child walking towards you.
    Dressed in rags, their face covered in grime, the child greets you:
    Why are you out here?");
  
    let mut here = String::new();
    io::stdin()
    .read_line(&mut here)
    .unwrap();
       
       println!("I get that you said: {}", here);
       println!("that's not good enough, try again.");
       println!("Why are you here?");
       

}


