use rand::Rng;
// use std::{collections::HashMap, io};

fn main() {
    // let mut points: u32 = 0;
    println!("{:?}", throw_dice(6));
}

fn throw_dice(n: u32) -> Vec<u32> {
    let mut dice_out: Vec<u32> = Vec::new();
    (0..n).for_each(|_| dice_out.push(rand::thread_rng().gen_range(1..=6)));
    dice_out
}
