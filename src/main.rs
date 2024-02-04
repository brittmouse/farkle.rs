use rand::Rng;

fn main() {
    println!("{:?}", throw_dice(6));
}

fn throw_dice(n: i32) -> Vec<i32> {
    let mut dice_out: Vec<i32> = Vec::new();
    for _ in 0..n {
        dice_out.push(rand::thread_rng().gen_range(1..=6));
    }
    dice_out
}
