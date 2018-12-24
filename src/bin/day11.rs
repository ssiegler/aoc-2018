fn main() {
    let grid = Grid::new(5468);
    println!("{:?}", Grid::new(18).find_max_three_square());
    println!("{:?}", Grid::new(42).find_max_three_square());
    println!("{:?}", grid.find_max_three_square());
    println!("{:?}", Grid::new(18).find_max_square());
    println!("{:?}", Grid::new(42).find_max_square());
    println!("{:?}", grid.find_max_square());
}

struct Grid {
    power_levels: [[isize; 300]; 300],
}

impl Grid {
    pub fn new(serial: usize) -> Self {
        let mut grid = Grid {
            power_levels: [[0; 300]; 300],
        };
        grid.compute_power_levels(serial);
        grid
    }

    pub fn find_max_three_square(&self) -> ((usize, usize), isize) {
        let mut result = (0, 0);
        let mut max_level = isize::min_value();

        for x in 0..=297 {
            for y in 0..=297 {
                let level = self.sum_square(x, y, 3);
                if level > max_level {
                    max_level = level;
                    result = (x + 1, y + 1);
                }
            }
        }
        (result, max_level)
    }

    pub fn find_max_square(&self) -> ((usize, usize, usize), isize) {
        let mut result = (0, 0, 0);
        let mut max_level = isize::min_value();

        for left in 0..300 {
            for top in 0..300 {
                let mut level = 0;
                for size in 0..300 - top.max(left) {
                    for i in 0..size {
                        level += self.power_levels[left + size][top + i];
                        level += self.power_levels[left + i][top + size];
                    }
                    level += self.power_levels[left + size][top + size];
                    if level > max_level {
                        max_level = level;
                        result = (left + 1, top + 1, size + 1);
                    }
                }
            }
        }
        (result, max_level)
    }

    fn sum_square(&self, left: usize, top: usize, size: usize) -> isize {
        let mut sum = 0;
        for x in left..left + size {
            for y in top..top + size {
                sum += self.power_levels[x][y];
            }
        }
        sum
    }

    fn compute_power_levels(&mut self, serial: usize) {
        for (x, mut row) in self.power_levels.iter_mut().enumerate() {
            for (y, mut col) in row.iter_mut().enumerate() {
                *col = Self::power_level(x + 1, y + 1, serial);
            }
        }
    }

    fn power_level(left: usize, top: usize, serial: usize) -> isize {
        let rack_id = left + 10;
        let mut result = rack_id * top;
        result += serial;
        result *= rack_id;
        result /= 100;
        result %= 10;
        result as isize - 5
    }
}
