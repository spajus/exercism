pub fn raindrops(num: u32) -> &'static str {
    let factor = |x, y| x % y == 0;
    match num {
        num if factor(num, 3) => "Pling",
        num if factor(num, 5) => "Plang",
        num if factor(num, 7) => "Plong",
        _ => num.to_string().as_str()
    }
}
