use std::{
    iter::from_fn,
    mem
};

#[derive(Debug, Clone, Eq, PartialEq)]
enum DirectionX {
    East,
    West,
}

impl DirectionX {
    fn get_direction(& self) -> Direction {
        match self {
            DirectionX::East => Direction::East,
            DirectionX::West => Direction::West,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum DirectionY {
    North,
    South,
}

impl DirectionY {
    fn get_direction(& self) -> Direction {
        match self {
            DirectionY::North => Direction::North,
            DirectionY::South => Direction::South,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn get_direction_x(& self) -> DirectionX {
        match self {
            Direction::East => DirectionX::East,
            Direction::West => DirectionX::West,
            _ => panic!("Not DirectionX"),
        }
    }

    fn get_direction_y(& self) -> DirectionY {
        match self {
            Direction::North => DirectionY::North,
            Direction::South => DirectionY::South,
            _ => panic!("Not DirectionY"),
        }
    }

    fn rotate(& self, rotation: &Rotation, value: &usize) -> Direction {
        match self {
            Direction::East => {
                match rotation {
                    Rotation::Left => {
                        match value {
                            90 => Direction::North,
                            180 => Direction::West,
                            270 => Direction::South,
                            _ => panic!("Invalid rotation value")
                        }
                    }
                    Rotation::Right => {
                        match value {
                            90 => Direction::South,
                            180 => Direction::West,
                            270 => Direction::North,
                            _ => panic!("Invalid rotation value")
                        }
                    }
                }
            },
            Direction::South => {
                match rotation {
                    Rotation::Left => {
                        match value {
                            90 => Direction::East,
                            180 => Direction::North,
                            270 => Direction::West,
                            _ => panic!("Invalid rotation value")
                        }
                    }
                    Rotation::Right => {
                        match value {
                            90 => Direction::West,
                            180 => Direction::North,
                            270 => Direction::East,
                            _ => panic!("Invalid rotation value")
                        }
                    }
                }
            },
            Direction::West => {
                match rotation {
                    Rotation::Left => {
                        match value {
                            90 => Direction::South,
                            180 => Direction::East,
                            270 => Direction::North,
                            _ => panic!("Invalid rotation value")
                        }
                    }
                    Rotation::Right => {
                        match value {
                            90 => Direction::North,
                            180 => Direction::East,
                            270 => Direction::South,
                            _ => panic!("Invalid rotation value")
                        }
                    }
                }
            },
            Direction::North => {
                match rotation {
                    Rotation::Left => {
                        match value {
                            90 => Direction::West,
                            180 => Direction::South,
                            270 => Direction::East,
                            _ => panic!("Invalid rotation value")
                        }
                    }
                    Rotation::Right => {
                        match value {
                            90 => Direction::East,
                            180 => Direction::South,
                            270 => Direction::West,
                            _ => panic!("Invalid rotation value")
                        }
                    }
                }
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum NavigationInstruction {
    Move(Direction, usize),
    Turn(Rotation, usize),
    GoForward(usize),
}

fn get_instructions() -> Vec<NavigationInstruction> {
    super::file::read_file("./inputs/day12.txt")
        .map(|l| -> NavigationInstruction {
            match l.split_at(1) {
                ("N", value) => NavigationInstruction::Move(Direction::North, value.parse::<usize>().unwrap()),
                ("S", value) => NavigationInstruction::Move(Direction::South, value.parse::<usize>().unwrap()),
                ("E", value) => NavigationInstruction::Move(Direction::East, value.parse::<usize>().unwrap()),
                ("W", value) => NavigationInstruction::Move(Direction::West, value.parse::<usize>().unwrap()),
                ("L", value) => NavigationInstruction::Turn(Rotation::Left, value.parse::<usize>().unwrap()),
                ("R", value) => NavigationInstruction::Turn(Rotation::Right, value.parse::<usize>().unwrap()),
                ("F", value) => NavigationInstruction::GoForward(value.parse::<usize>().unwrap()),
                _ => panic!("Invalid instruction")
            }
        }).collect()
}

#[derive(Debug)]
struct ShipPart1 {
    pos_x: (DirectionX, usize),
    pos_y: (DirectionY, usize),
    direction: Direction,
}

fn move_object(pos_x: (DirectionX, usize), pos_y: (DirectionY, usize), direction: &Direction, value: &usize) -> ((DirectionX, usize), (DirectionY, usize)) {
    match direction {
        Direction::East => {
            if pos_x.0 == DirectionX::West {
                if pos_x.1 < *value {
                    return ((DirectionX::East, value - pos_x.1), pos_y)
                } else {
                    return ((DirectionX::West, pos_x.1 - value), pos_y)
                }
            } else {
                return ((DirectionX::East, value + pos_x.1), pos_y)
            }
        },
        Direction::West => {
            if pos_x.0 == DirectionX::East {
                if pos_x.1 < *value {
                    return ((DirectionX::West, value - pos_x.1), pos_y)
                } else {
                    return ((DirectionX::East, pos_x.1 - value), pos_y)
                }
            } else {
                return ((DirectionX::West, value + pos_x.1), pos_y)
            }
        },
        Direction::North => {
            if pos_y.0 == DirectionY::South {
                if pos_y.1 < *value {
                    return (pos_x, (DirectionY::North, value - pos_y.1))
                } else {
                    return (pos_x, (DirectionY::South, pos_y.1 - value))
                }
            } else {
                return (pos_x, (DirectionY::North, value + pos_y.1))
            }
        },
        Direction::South => {
            if pos_y.0 == DirectionY::North {
                if pos_y.1 < *value {
                    return (pos_x, (DirectionY::South, value - pos_y.1))
                } else {
                    return (pos_x, (DirectionY::North, pos_y.1 - value))
                }
            } else {
                return (pos_x, (DirectionY::South, value + pos_y.1))
            }
        },
    }
}

impl ShipPart1 {
    fn run_instruction(&mut self, instruction: &NavigationInstruction) {
        match instruction {
            NavigationInstruction::Move(direction, value) => {
                let pos_x = mem::replace(&mut self.pos_x, (DirectionX::East, 0));
                let pos_y = mem::replace(&mut self.pos_y, (DirectionY::North, 0));
                let (pos_x, pos_y) = move_object(pos_x, pos_y, direction, value);
                self.pos_x = pos_x;
                self.pos_y = pos_y;
            },
            NavigationInstruction::Turn(rotation, value) => {
                self.direction = self.direction.rotate(rotation, value)
            },
            NavigationInstruction::GoForward(value) => {
                self.run_instruction(&NavigationInstruction::Move(self.direction.clone(), *value))
            }
        }
    }
}

#[derive(Debug)]
struct ShipPart2 {
    pos_x: (DirectionX, usize),
    pos_y: (DirectionY, usize),
    waypoint: ((DirectionX, usize), (DirectionY, usize)),
}

impl ShipPart2 {
    fn move_ship(&mut self, direction: &Direction, value: &usize) {
        let pos_x = mem::replace(&mut self.pos_x, (DirectionX::East, 0));
        let pos_y = mem::replace(&mut self.pos_y, (DirectionY::North, 0));
        let (pos_x, pos_y) = move_object(pos_x, pos_y, direction, value);
        self.pos_x = pos_x;
        self.pos_y = pos_y;
    }

    fn run_instruction(&mut self, instruction: &NavigationInstruction) {
        match instruction {
            NavigationInstruction::Move(direction, value) => {
                let (pos_x, pos_y) = mem::replace(&mut self.waypoint, ((DirectionX::East, 0), (DirectionY::North, 0)));
                self.waypoint = move_object(pos_x, pos_y, direction, value);
            },
            NavigationInstruction::Turn(rotation, value) => {
                if *value == 90 || *value == 270 {
                    let waypoint_dir_y = self.waypoint.0.0.get_direction().rotate(rotation, value).get_direction_y();
                    let waypoint_dir_x = self.waypoint.1.0.get_direction().rotate(rotation, value).get_direction_x();
                    self.waypoint = (
                        (waypoint_dir_x, self.waypoint.1.1),
                        (waypoint_dir_y, self.waypoint.0.1),
                    );
                } else {
                    let waypoint_dir_x = self.waypoint.0.0.get_direction().rotate(rotation, value).get_direction_x();
                    let waypoint_dir_y = self.waypoint.1.0.get_direction().rotate(rotation, value).get_direction_y();
                    self.waypoint = (
                        (waypoint_dir_x, self.waypoint.0.1),
                        (waypoint_dir_y, self.waypoint.1.1),
                    );
                }
            },
            NavigationInstruction::GoForward(value) => {
                match self.waypoint.0.0 {
                    DirectionX::East => self.move_ship(&Direction::East, &(value * self.waypoint.0.1)),
                    DirectionX::West => self.move_ship(&Direction::West, &(value * self.waypoint.0.1)),
                }
                match self.waypoint.1.0 {
                    DirectionY::North => self.move_ship(&Direction::North, &(value * self.waypoint.1.1)),
                    DirectionY::South => self.move_ship(&Direction::South, &(value * self.waypoint.1.1)),
                }
            }
        }
    }
}

pub fn main () {
    let instructions = get_instructions();
    // println!("{:?}", instructions);
    // let mut ship = ShipPart1 {
    //     pos_x: (DirectionX::East, 0),
    //     pos_y: (DirectionY::North, 0),
    //     direction: Direction::East
    // };
    let mut ship = ShipPart2 {
        pos_x: (DirectionX::East, 0),
        pos_y: (DirectionY::North, 0),
        waypoint: ((DirectionX::East, 10), (DirectionY::North, 1)),
    };
    for i in instructions.iter() {
        ship.run_instruction(i);
        // println!("{:?} -> {:?}", i, ship)
    }
    println!("Final position: {:?} ; {:?}", ship.pos_x, ship.pos_y);
    println!("  Manhattan distance: {}", ship.pos_x.1 + ship.pos_y.1)
}
