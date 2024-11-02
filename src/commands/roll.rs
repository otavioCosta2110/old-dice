use rand::random;

pub fn roll(dice_side: u32, modifier: u32, times: u32) -> Vec<u32> {
    let mut values = Vec::new();
    for _ in 0..times {
        let dice_value = random::<u32>() % dice_side + 1;
        let total_value = dice_value + modifier;
        println!("{:?} + {:?} = {:?}", dice_value, modifier, total_value);
        values.push(total_value)
    }
    return values
}
