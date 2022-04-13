use std::io;

fn prompt() -> bool{
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .unwrap();
    return input.to_ascii_lowercase().starts_with("y");
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
    println!("Oh, wow! Omfg I don't know what to do! UH....");
    println!("Wanna play a game?");

    if prompt(){
        play_game();
    }else{
        println!("Oh... Ok! That's no problem,\nit's not like I'm going anywhere, \ncome back when you wanna play!");
    }
}

pub fn play_game() -> (){
    println!("Oh, fuck yeah!");
    println!("uh... I mean, ok cool!");
    println!("You realize you are alone in an almost empty attic of a large house.
There is a box in the corner, do you open it?");

    if prompt(){
        open_box();
    }else{
        println!("The room gets dark and the voices come back");
        println!("You don't.");
    }
}

pub fn open_box() -> (){
    println!("As you walk to the box, you notice it shake.
Committed to discovery you move forward.");
    println!("Picking up the box with both hands seems to make the box shake more!");
    println!("Opening the box you see a shadow rush from the box as the room is brilliantly lit.");
    println!("You see the shadow dart down a previously hidden ladder rushing further into the house.");
    println!("Do you follow the shadow?");
    
    if prompt(){
        follow_shadow();
    }else{
        println!("There's nothing here but an empty room, now you're stuck.\n GAME OVER");
    }
}
pub fn follow_shadow() -> (){
    println!("Rushing after the shadow, you follow as it slithers down the ladder, \nand moves out of sight at the end of a hall.");
    println!("Reaching the end of the hall, you see that you can go left or right.\n
There are shadows dancing to the left, the right side is brightly lit.\n Do you walk towards the light?");

    if prompt(){
        
        println!("Cool, it's true.");
    }else{
        shadows();
    }
}
pub fn shadows() -> (){
    println!("As you walk towards the dancing shadows, they start to move faster \n
    seeming to move towards you as you got closer with shadowy arms thrashing at the empty air.");
    println!("do you continue closer?");

    if prompt(){
        println!("Well that can't be good...");
        notsogood();
    }else{
        maybebetter()
    }
}
pub fn notsogood() -> (){
    println!("Moving closer seems to trigger the shadows, the arms extend and reach out grasping your leg firmly.");
    println!("Though just a shadow, there is clearly something solid about it.");
    println!("Do you attack the shadow, or beg for your life?");
    println!("yes= attack \n beg = beg \n obviously");

    if prompt(){
        attack()
    }else{
        beg()
    }
}
pub fn maybebetter() -> (){
    println!("this might be better");
}
pub fn attack() -> (){
    println!("Being dummy thicc you aren't afraid of shit, \n
    you smash the arm with your other foot, causing the shadow to recoil in agony.");
    //need to fill out the rest of attack and beg fn, stopping here to test
}
pub fn beg() -> (){
    println!("You beg like the little baby that you are, crying to the shadows that you want to be freed.\n
    sadly the shadows can't hear so you die now.");
}

