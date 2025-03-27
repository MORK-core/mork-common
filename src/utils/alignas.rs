pub fn align_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

pub fn align_down(value: usize, alignment: usize) -> usize {
    value & !(alignment - 1)
}

pub fn is_aligned(value: usize, alignment: usize) -> bool {
    value % alignment == 0
}