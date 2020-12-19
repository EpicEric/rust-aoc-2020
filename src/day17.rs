use std::{
    cmp,
    collections::{HashSet, HashMap},
    hash::{Hash, Hasher},
    iter::from_fn,
};

type Coordinates = (isize, isize, isize, isize);

// Part 1
fn iterate_over_coordinates_part_1<'a>(
    min_coordinates: &'a Coordinates,
    max_coordinates: &'a Coordinates
) -> impl Iterator<Item=Coordinates> + 'a {
    let mut pos = -1isize;
    let x_width = max_coordinates.0 - min_coordinates.0 + 1;
    let y_width = max_coordinates.1 - min_coordinates.1 + 1;
    let z_width = max_coordinates.2 - min_coordinates.2 + 1;
    let max_pos = x_width * y_width * z_width;
    from_fn(move || {
        loop {
            pos += 1;
            if pos == max_pos {
                return None
            } else {
                let z_diff = pos / (x_width * y_width);
                let y_diff = pos % (x_width * y_width) / x_width;
                let x_diff = pos % x_width;
                return Some((
                    min_coordinates.0 + x_diff,
                    min_coordinates.1 + y_diff,
                    min_coordinates.2 + z_diff,
                    0,
                ));
            }
        }
    })
}

// Part 2
fn iterate_over_coordinates_part_2<'a>(
    min_coordinates: &'a Coordinates,
    max_coordinates: &'a Coordinates
) -> impl Iterator<Item=Coordinates> + 'a {
    let mut pos = -1isize;
    let x_width = max_coordinates.0 - min_coordinates.0 + 1;
    let y_width = max_coordinates.1 - min_coordinates.1 + 1;
    let z_width = max_coordinates.2 - min_coordinates.2 + 1;
    let w_width = max_coordinates.3 - min_coordinates.3 + 1;
    let max_pos = x_width * y_width * z_width * w_width;
    from_fn(move || {
        loop {
            pos += 1;
            if pos == max_pos {
                return None
            } else {
                let w_diff = pos / (x_width * y_width * z_width);
                let z_diff = pos % (x_width * y_width * z_width) / (x_width * y_width);
                let y_diff = pos % (x_width * y_width) / x_width;
                let x_diff = pos % x_width;
                return Some((
                    min_coordinates.0 + x_diff,
                    min_coordinates.1 + y_diff,
                    min_coordinates.2 + z_diff,
                    min_coordinates.3 + w_diff,
                ));
            }
        }
    })
}

#[derive(Debug, Clone)]
struct ActiveCube {
    coordinates: Coordinates,
    neighbors: HashSet<Coordinates>,
}

impl Hash for ActiveCube {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinates.hash(state);
    }
}

impl PartialEq for ActiveCube {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

impl Eq for ActiveCube {}

impl ActiveCube {
    fn add_neighbor(&mut self, neighbor: Coordinates) {
        self.neighbors.insert(neighbor);
    }

    fn remove_neighbor(&mut self, neighbor: &Coordinates) {
        self.neighbors.remove(neighbor);
    }
}

#[derive(Debug, Clone)]
struct Dimension {
    min_coordinates: Coordinates,
    max_coordinates: Coordinates,
    active_cubes: HashMap<Coordinates, ActiveCube>,
}

impl Dimension {
    fn new() -> Self {
        Dimension {
            min_coordinates: (0, 0, 0, 0),
            max_coordinates: (0, 0, 0, 0),
            active_cubes: HashMap::new(),
        }
    }

    fn deactivate_cube(&mut self, coordinates: &Coordinates) {
        match self.active_cubes.remove(coordinates) {
            Some(cube) => {
                cube.neighbors.iter().for_each(|c| {
                    let c = self.active_cubes.get_mut(c).unwrap();
                    c.remove_neighbor(&cube.coordinates);
                });
            },
            None => (),
        };
    }

    fn activate_cube(&mut self, coordinates: &Coordinates) {
        let cube = ActiveCube {
            coordinates: *coordinates,
            neighbors: self.get_active_neighbors_iter(coordinates).iter().map(|c| c.coordinates).collect(),
        };
        self.for_each_active_neighbors(coordinates, |c| c.add_neighbor(*coordinates));
        self.active_cubes.insert(*coordinates, cube);
        self.min_coordinates = (
            cmp::min(coordinates.0, self.min_coordinates.0),
            cmp::min(coordinates.1, self.min_coordinates.1),
            cmp::min(coordinates.2, self.min_coordinates.2),
            cmp::min(coordinates.3, self.min_coordinates.3),
        );
        self.max_coordinates = (
            cmp::max(coordinates.0, self.max_coordinates.0),
            cmp::max(coordinates.1, self.max_coordinates.1),
            cmp::max(coordinates.2, self.max_coordinates.2),
            cmp::max(coordinates.3, self.max_coordinates.3),
        );
    }

    fn get_active_coordinates(&self) -> impl Iterator<Item=&Coordinates> {
        self.active_cubes.iter().map(|(k, _)| k)
    }

    // TODO: Figure out how to return Iterator instead of Vec
    fn get_active_neighbors_iter(&self, coordinates: &Coordinates) -> Vec<&ActiveCube> {
        // Part 1
        // iterate_over_coordinates_part_1(
        //     &(coordinates.0 - 1, coordinates.1 - 1, coordinates.2 - 1, 0),
        //     &(coordinates.0 + 1, coordinates.1 + 1, coordinates.2 + 1, 0),
        // Part 2
        iterate_over_coordinates_part_2(
            &(coordinates.0 - 1, coordinates.1 - 1, coordinates.2 - 1, coordinates.3 - 1),
            &(coordinates.0 + 1, coordinates.1 + 1, coordinates.2 + 1, coordinates.3 + 1),

        ).filter(|c| c != coordinates).map(|c| self.active_cubes.get(&c)).flatten().collect()
    }

    fn for_each_active_neighbors<F>(&mut self, coordinates: &Coordinates, f: F) where
        F: Fn(&mut ActiveCube)
    {
        // Part 1
        // iterate_over_coordinates_part_1(
        //     &(coordinates.0 - 1, coordinates.1 - 1, coordinates.2 - 1, 0),
        //     &(coordinates.0 + 1, coordinates.1 + 1, coordinates.2 + 1, 0),
        // Part 2
        iterate_over_coordinates_part_2(
            &(coordinates.0 - 1, coordinates.1 - 1, coordinates.2 - 1, coordinates.3 - 1),
            &(coordinates.0 + 1, coordinates.1 + 1, coordinates.2 + 1, coordinates.3 + 1),
        ).filter(|c| c != coordinates).for_each(|c| match self.active_cubes.get_mut(&c) {
            Some(cube) => f(cube),
            _ => (),
        });
    }

    fn run_cycle(&mut self) {
        let mut cubes_to_deactivate: Vec<Coordinates> = Vec::new();
        let mut cubes_to_activate: Vec<Coordinates> = Vec::new();

        // Part 1
        // for coordinate in iterate_over_coordinates_part_1(
        //     &(self.min_coordinates.0 - 1, self.min_coordinates.1 - 1, self.min_coordinates.2 - 1, 0),
        //     &(self.max_coordinates.0 + 1, self.max_coordinates.1 + 1, self.max_coordinates.2 + 1, 0),
        // Part 2
        for coordinate in iterate_over_coordinates_part_2(
            &(self.min_coordinates.0 - 1, self.min_coordinates.1 - 1, self.min_coordinates.2 - 1, self.min_coordinates.3 - 1),
            &(self.max_coordinates.0 + 1, self.max_coordinates.1 + 1, self.max_coordinates.2 + 1, self.max_coordinates.3 + 1),
        ) {
            match (self.active_cubes.contains_key(&coordinate), self.get_active_neighbors_iter(&coordinate).len()) {
                (true, x) if x < 2 || x > 3 => cubes_to_deactivate.push(coordinate),
                (false, 3) => cubes_to_activate.push(coordinate),
                _ => (),
            }
        }

        cubes_to_deactivate.iter().for_each(|c| self.deactivate_cube(c));
        cubes_to_activate.iter().for_each(|c| self.activate_cube(c));
    }
}

fn get_initial_state() -> Dimension {
    let mut dimension = Dimension::new();
    for (y, line) in super::file::read_file("./inputs/day17.txt").enumerate() {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
            dimension.activate_cube(&(x as isize, y as isize, 0, 0));
        }
    }
    dimension
}

pub fn main () {
    let mut data = get_initial_state();
    let total_cycles = 6;

    println!("Initial state: {} active cubes", data.active_cubes.len());

    for i in 1..total_cycles + 1 {
        data.run_cycle();
        println!("{}) {} active cubes", i, data.active_cubes.len());
    }
}
