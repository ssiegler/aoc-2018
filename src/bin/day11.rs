struct Cell {
    x: usize,
    y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Cell { x, y }
    }

    fn power_level(&self, serial: usize) -> isize {
        let rack_id = self.x + 10;
        let mut result = rack_id * self.y;
        result += serial;
        result *= rack_id;
        result /= 100;
        result %= 10;
        result as isize - 5
    }
}

#[test]
fn power_level_examples() {
    assert_eq!(Cell::new(3, 5).power_level(8), 4);
    assert_eq!(Cell::new(122, 79).power_level(57), -5);
    assert_eq!(Cell::new(217, 196).power_level(39), 0);
    assert_eq!(Cell::new(101, 153).power_level(71), 4);
}
