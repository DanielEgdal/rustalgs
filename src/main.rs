use rustalgs::*;
// use std::env;
// use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use chrono::{self, Timelike, Datelike};


fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    let triggers = get_triggers();
    let prune = get_prune(&triggers);

    let solutions = solve(&triggers,&prune);

    let time =  chrono::offset::Local::now();
    // println!("{} ",time.to_string()); 
    let ftime = format!("runFrom{}{}{}{}",time.day(),time.hour(),time.minute(),time.second());

    let mut file = File::create(format!("{}.txt",&ftime)).unwrap();

    for sol in &solutions{
        if check_valid_alg(sol){
            writeln!(&mut file, "{}",cancel_moves(sol)).unwrap();
        }
    }


    // println!("{:?}",triggers);
    // println!("{:?}",&prune.len());
    // let plen = &prune.len();
    // println!("Hello, world!");

    // println!("{:?} ",int_move_to_str(&(&prune.into_iter().nth(*&plen-20)).unwrap().1));
}
