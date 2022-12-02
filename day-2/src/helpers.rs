pub fn circular_subtract(x: i64, y: i64) -> i64 {
    let mut result = x - y;
    if result == 2 {
        result = -1;
    }
    if result == -2 {
        result = 1;
    }
    return result;
}
