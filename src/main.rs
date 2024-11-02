mod commands;

use crate::commands::roll::roll;
use clap::Parser;


#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub dice_sides: String,

    #[arg(short, long)]
    pub modifier: Option<String>,

    #[arg(short, long)]
    pub times: Option<String>,
}

pub fn get_args() -> Args {
    Args::parse()
}

pub fn get_dice_sides() -> u32 {
    let args = get_args();
    let dice_sides = args.dice_sides.parse::<u32>().unwrap();
    return dice_sides;
}

pub fn get_modifier() -> u32 {
    let args = get_args();
    if let Some(modifier_arg) = args.modifier {
        let modifier = modifier_arg.parse::<u32>().unwrap();
        return modifier;
    }
    return 0;
}

pub fn get_times() -> u32 {
    let args = get_args();
    if let Some(times_arg) = args.times {
        let times = times_arg.parse::<u32>().unwrap();
        return times;
    }
    return 1;
}

fn main() {
    let dice_sides = get_dice_sides();
    let modifier = get_modifier();
    let times = get_times();

    let value = roll(dice_sides, modifier, times);

    println!("{:?}", value);
}
