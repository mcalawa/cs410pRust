// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
pub struct Robot {
    horizontal: isize,
    vertical: isize,
    facing: Direction
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        let robot = Robot { 
            horizontal: x, 
            vertical: y, 
            facing: d 
        };
        robot
    }

    pub fn turn_right(mut self) -> Self {
        if self.direction() == &Direction::North {
            self.facing = Direction::East;
        }
        else if self.direction() == &Direction::East {
            self.facing = Direction::South;
        }
        else if self.direction() == &Direction::South {
            self.facing = Direction::West;
        }
        else {
            self.facing = Direction::North;
        }

        self
    }

    pub fn turn_left(mut self) -> Self {
        if self.direction() == &Direction::North {
            self.facing = Direction::West;
        }
        else if self.direction() == &Direction::West {
            self.facing = Direction::South;
        }
        else if self.direction() == &Direction::South {
            self.facing = Direction::East;
        }
        else {
            self.facing = Direction::North;
        }

        self
    }

    pub fn advance(mut self) -> Self {
        if self.direction() == &Direction::North {
            self.vertical += 1;
        }
        else if self.direction() == &Direction::South {
            self.vertical -= 1;
        }
        else if self.direction() == &Direction::East {
            self.horizontal += 1;
        }
        else {
            self.horizontal -= 1;
        }

        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for instruction in 0..instructions.len() + 1 {
            let (first, _last) = instructions.split_at(instruction);
            if first.ends_with("L") {
                self = self.turn_left();
            }
            else if first.ends_with("R") {
                self = self.turn_right();
            }
            else if first.ends_with("A") {
                self = self.advance();
            }
        }

        self
    }

    pub fn position(&self) -> (isize, isize) {
        (self.horizontal, self.vertical)
    }

    pub fn direction(&self) -> &Direction {
        &self.facing
    }
}
