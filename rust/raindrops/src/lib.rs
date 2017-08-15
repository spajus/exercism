pub fn raindrops(num: u32) -> String {
    let mut sound = String::new();
    let factor = |x, y| x % y == 0;
    if factor(num, 3) { sound.push_str("Pling") };
    if factor(num, 5) { sound.push_str("Plang") };
    if factor(num, 7) { sound.push_str("Plong") };
    if sound.is_empty() {
        sound = num.to_string()
    }
    sound
}
