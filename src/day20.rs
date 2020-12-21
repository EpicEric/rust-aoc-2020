use std::{
    cmp,
    collections::{HashMap, HashSet},
};
use num_integer::Roots;
use regex::Regex;

#[derive(Debug, Clone)]
struct TileBorder {
    hash: usize,
}

impl TileBorder {
    fn new(hash: usize) -> Self {
        Self {
            hash: hash,
        }
    }

    fn flip(&self) -> Self {
        let mut new_hash = 0usize;
        for i in 0..10 {
            new_hash <<= 1;
            new_hash |= ((1 << i) & self.hash) >> i;
        }
        Self {
            hash: new_hash,
        }
    }
}

#[derive(Debug, Clone)]
struct Camera {
    id: usize,
    north: TileBorder,
    east: TileBorder,
    south: TileBorder,
    west: TileBorder,
    photo: Vec<Vec<bool>>
}

impl Camera {
    fn new(id: usize, photo: Vec<Vec<bool>>) -> Self {
        let north = photo[0].iter().fold(0usize, |acc, &x| (acc << 1) + x as usize);
        let east = photo.iter().map(|l| l.last().unwrap_or(&false)).fold(0usize, |acc, &x| (acc << 1) + x as usize);
        let south = photo.last().unwrap_or(&vec![]).iter().fold(0usize, |acc, &x| (acc << 1) + x as usize);
        let west = photo.iter().map(|l| l[0]).fold(0usize, |acc, x| (acc << 1) + x as usize);
        Self {
            id: id,
            north: TileBorder::new(north),
            east: TileBorder::new(east),
            south: TileBorder::new(south),
            west: TileBorder::new(west),
            photo: photo,
        }
    }

    fn all_possible_borders(&self) -> Vec<TileBorder> {
        vec![
            self.north.clone(),
            self.east.clone(),
            self.south.clone(),
            self.west.clone(),
            self.north.flip(),
            self.east.flip(),
            self.south.flip(),
            self.west.flip(),
        ]
    }

    fn get_trimmed_photo(&self) -> Vec<Vec<bool>> {
        self.photo[1..self.photo.len() - 1].iter().map(|line| line[1..line.len() - 1].to_vec()).collect()
    }

    // fn rotations(&self) -> Vec<Self> {
    //     vec![
    //         self.clone(),
    //         Self {
    //             id: self.id,
    //             north: self.east.clone(),
    //             east: self.south.clone(),
    //             south: self.west.clone(),
    //             west: self.north.clone(),
    //         },
    //         Self {
    //             id: self.id,
    //             north: self.south.clone(),
    //             east: self.west.clone(),
    //             south: self.north.clone(),
    //             west: self.east.clone(),
    //         },
    //         Self {
    //             id: self.id,
    //             north: self.west.clone(),
    //             east: self.north.clone(),
    //             south: self.east.clone(),
    //             west: self.south.clone(),
    //         },
    //     ]
    // }

    // fn horizontal_flip(&self) -> Self {
    //     Self {
    //         id: self.id,
    //         north: self.north.flip(),
    //         east: self.west.clone(),
    //         south: self.south.flip(),
    //         west: self.east.clone(),
    //     }
    // }

    // fn vertical_flip(&self) -> Self {
    //     Self {
    //         id: self.id,
    //         north: self.south.clone(),
    //         east: self.east.flip(),
    //         south: self.north.clone(),
    //         west: self.west.flip(),
    //     }
    // }

    // fn hv_flip(&self) -> Self {
    //     Self {
    //         id: self.id,
    //         north: self.south.flip(),
    //         east: self.west.flip(),
    //         south: self.north.flip(),
    //         west: self.east.flip(),
    //     }
    // }
}

fn get_data() -> Vec<Camera> {
    let mut lines = super::file::read_file("./inputs/day20.txt");
    let mut cameras = Vec::<Camera>::new();
    loop {
        match lines.next() {
            Some(line) => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^Tile (\d+):$").expect("invalid tile number regex");
                }
                let caps = RE.captures(&line).expect("line did not match tile number regex");
                let id: usize = caps[1].parse().expect("invalid tile ID");
                let mut photo: Vec<Vec<bool>> = Vec::with_capacity(10);
                for _ in 0..10 {
                    let line = lines.next().unwrap();
                    photo.push(line.chars().map(|c| {
                        match c {
                            '#' => true,
                            '.' => false,
                            _ => panic!("invalid character in photo"),
                        }
                    }).collect());
                }
                cameras.push(Camera::new(id, photo));
                match lines.next() {
                    None => break,
                    _ => (),
                }
            },
            None => break,
        };
    }
    cameras
}

// Part 1
fn get_product_of_corners(cameras: &Vec<Camera>) -> usize {
    let mut matching_borders_with_cameras: HashMap<usize, HashSet<usize>> = HashMap::new();
    cameras.iter().map(|camera| (camera.id, camera.all_possible_borders())).for_each(|(id, borders)| {
        borders.iter().for_each(|border| {
            matching_borders_with_cameras.entry(cmp::min(border.hash, border.flip().hash)).or_insert(HashSet::new()).insert(id);
        })
    });
    // println!("{:?}", matching_borders_with_cameras);
    let mut product = 1usize;
    let mut count = 0usize;
    for camera in cameras.iter() {
        if matching_borders_with_cameras.values().filter(|set| set.iter().collect::<Vec<&usize>>() == vec![&camera.id]).count() == 2 {
            // println!("Possible border camera: {}", camera.id);
            product *= camera.id;
            count += 1;
        }
    }
    if count > 4 {
        panic!("Unexpected format")
    }
    product
}

// Part 2
fn get_image(cameras: &Vec<Camera>) {
    let size = cameras.len().sqrt();
    assert_eq!(size * size, cameras.len());
    // let layers = (size / 2) + (size % 2);

    // Find connecting cameras
    let mut matching_borders_with_cameras: HashMap<usize, HashSet<usize>> = HashMap::new();
    cameras.iter().map(|camera| (camera.id, camera.all_possible_borders())).for_each(|(id, borders)| {
        borders.iter().for_each(|border| {
            matching_borders_with_cameras.entry(cmp::min(border.hash, border.flip().hash)).or_insert(HashSet::new()).insert(id);
        })
    });

    // Separate cameras by class (4 corners, all borders, and innermost)
    let mut corner_cameras = cameras.iter().filter(|camera| {
        matching_borders_with_cameras.values()
            // Find all borders of the camera not shared with any other camera
            .filter(|set| set.iter().collect::<Vec<&usize>>() == vec![&camera.id])
            // If it equals 2, this is a corner camera
            .count() == 2
    }).collect::<Vec<_>>();
    let border_cameras = cameras.iter().filter(|camera| {
        matching_borders_with_cameras.values()
            // Find all borders of the camera not shared with any other camera
            .filter(|set| set.iter().collect::<Vec<&usize>>() == vec![&camera.id])
            // If it is greater than or equal to, this is a border camera
            .count() >= 1
    }).collect::<Vec<_>>();
    let inner_cameras = cameras.iter().filter(|camera| {
        matching_borders_with_cameras.values()
            // Find all borders of the camera not shared with any other camera
            .filter(|set| set.iter().collect::<Vec<&usize>>() == vec![&camera.id])
            // If it is greater than or equal to, this is a border camera
            .count() == 0
    }).collect::<Vec<_>>();

    // Choose one of the four corners randomly and place into first diagonal (i.e. [0][0])
    let mut camera_array: Vec<Vec<Camera>> = vec![vec![Camera::new(0, vec![vec![false]]); size]; size];
    camera_array[0][0] = corner_cameras.pop().unwrap().clone();
    let mut consumed_ids = HashSet::<usize>::new();
    consumed_ids.insert(camera_array[0][0].id);

    // Build the second diagonal
    let mut second_diagonal_cameras = cameras.iter().map(|camera| {
        let matches_border = matching_borders_with_cameras.values()
            // Find all borders of the camera shared with the cornermost camera
            .filter(|set| set.contains(&camera_array[0][0].id) && set.contains(&camera.id))
            .count() == 1;
        if matches_border {
            Some(camera.clone())
        } else {
            None
        }
    }).flatten().collect::<Vec<_>>();
    assert_eq!(second_diagonal_cameras.len(), 2);
    camera_array[0][1] = second_diagonal_cameras.pop().unwrap();
    camera_array[1][0] = second_diagonal_cameras.pop().unwrap();
    consumed_ids.insert(camera_array[0][1].id);
    consumed_ids.insert(camera_array[1][0].id);

    // Build the remaining increasing diagonals
    for i in 2..size {
        for j in 0..i + 1 {
            let mut diagonal_camera_vec = (if j == 0 || j == i { border_cameras.iter() } else { inner_cameras.iter() }).filter(|camera| !consumed_ids.contains(&camera.id)).map(|camera| {
                let matches_border = matching_borders_with_cameras.values()
                    // Find all borders of the camera not shared with any other camera
                    .filter(|set| {
                        if j == 0 {
                            let neighbor_id = &camera_array[j][i - j - 1].id;
                            set.contains(neighbor_id) && set.contains(&camera.id)
                        } else if j == i {
                            let neighbor_id = &camera_array[j - 1][i - j].id;
                            set.contains(neighbor_id) && set.contains(&camera.id)
                        } else {
                            let first_neighbor_id = &camera_array[j][i - j - 1].id;
                            let second_neighbor_id = &camera_array[j - 1][i - j].id;
                            (set.contains(first_neighbor_id) || set.contains(second_neighbor_id)) && set.contains(&camera.id)
                        }
                    })
                    .count() == (if j == 0 || j == i { 1 } else { 2 });
                if matches_border {
                    Some(camera.clone())
                } else {
                    None
                }
            }).flatten().collect::<Vec<_>>();
            // println!("{:?}", diagonal_camera_vec.iter().map(|c| c.id).collect::<Vec<_>>());
            assert_eq!(diagonal_camera_vec.len(), 1);
            camera_array[j][i - j] = diagonal_camera_vec.pop().unwrap().clone();
            consumed_ids.insert(camera_array[j][i - j].id);
        }
    }

    // Build the remaining decreasing diagonals
    for i in size..(size * 2) - 1 {
        for j in i - size + 1..size {
            let mut diagonal_camera_vec = (if j == i - size + 1 || j == size - 1 { border_cameras.iter() } else { inner_cameras.iter() }).filter(|camera| !consumed_ids.contains(&camera.id)).map(|camera| {
                let matches_border = matching_borders_with_cameras.values()
                    // Find all borders of the camera not shared with any other camera
                    .filter(|set| {
                        if j == i - size + 1 {
                            let neighbor_id = &camera_array[i - j][j - 1].id;
                            set.contains(neighbor_id) && set.contains(&camera.id)
                        } else if j == size - 1 {
                            let neighbor_id = &camera_array[i - j - 1][j].id;
                            set.contains(neighbor_id) && set.contains(&camera.id)
                        } else {
                            let first_neighbor_id = &camera_array[i - j][j - 1].id;
                            let second_neighbor_id = &camera_array[i - j - 1][j].id;
                            (set.contains(first_neighbor_id) || set.contains(second_neighbor_id)) && set.contains(&camera.id)
                        }
                    })
                    .count() == (if j == i - size + 1 || j == size - 1 { 1 } else { 2 });
                if matches_border {
                    Some(camera.clone())
                } else {
                    None
                }
            }).flatten().collect::<Vec<_>>();
            // println!("{:?}", diagonal_camera_vec.iter().map(|c| c.id).collect::<Vec<_>>());
            assert_eq!(diagonal_camera_vec.len(), 1);
            println!("{} {}", i, j);
            camera_array[i - j][j] = diagonal_camera_vec.pop().unwrap().clone();
            consumed_ids.insert(camera_array[i - j][j].id);
        }
    }

    camera_array.iter().for_each(|line| {
        println!("{:?}", line.iter().map(|c| c.id).collect::<Vec<_>>());
    });

    // Align cameras properly and generate full image
    // TODO
}

pub fn main() {
    let data = get_data();
    
    // Part 1
    // let product = get_product_of_corners(&data);
    // println!("Product: {}", product);

    // Part 2
    get_image(&data);
}