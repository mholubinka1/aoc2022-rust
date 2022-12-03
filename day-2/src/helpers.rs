pub fn circular_subtract(x: i32, y: i32) -> i32 {
    let mut result = x - y;
    if result == 2 {
        result = -1;
    }
    if result == -2 {
        result = 1;
    }
    result
}
