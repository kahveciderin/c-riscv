pub fn nearest_multiple(value: u32, multiple: u32) -> u32 {
    if value == 0 {
        0
    } else {
        (1 + (value - 1) / multiple) * multiple
    }
}
