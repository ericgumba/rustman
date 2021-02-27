use rand::prelude::*; 
use std::io::{self};
use std::iter::FromIterator;
fn main() { 
    let words:[String; 12] = ["detailed".to_string(),
    "wistful".to_string(),
    "piquant".to_string(),
    "sore".to_string(),
    "cough".to_string(),
    "saw".to_string(),
    "wiry".to_string(),
    "laborer".to_string(),
    "snakes".to_string(),
    "thrill".to_string(),
    "instruct".to_string(),
    "remind".to_string()];
 
    let mut rng = thread_rng();
     
    let mut lives = 10;
    println!("words for {}", lives);
    let challenger = &words[rng.gen_range(0..12)];  

    
    let mut protagonist = Vec::new();

    for (_i, _c) in challenger.chars().enumerate() {  

        protagonist.push('_');
    }
    let s = String::from_iter(protagonist);
 


    while lives > 0 && ! challenger.eq(&String::from_iter(&protagonist) ) {

        println!("enter input:" );
        let mut input = String::new();
    
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Couldn't read line");    

        let x = input.trim();  
        let mut _lucked_out = false;

        for (i, c) in challenger.chars().enumerate() { 
            if x.contains(c) { 
                protagonist[i] = c;
                _lucked_out = true;
            }
        }


        if _lucked_out {
            println!("you lucked out punk. {}", &String::from_iter(&protagonist)); 
            _lucked_out = false; 
        } else {

        lives = lives - 1;
        println!("still not done {}", lives);

        }

    }
    
}
