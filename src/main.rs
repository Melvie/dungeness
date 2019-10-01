
use ndarray::prelude::*;
use ndarray::{s,array};
use std::fmt;
use std::collections::HashMap;
use pancurses::{initscr, endwin};
use rand::prelude::*;


#[derive(Debug)]
struct Map{
    layout: Array2<i32>
}

impl Map {
    fn map_from_file(x: &[u8]) -> Map {
        // make a border of 0 cells
        let mut grid = Array2::from_elem(((50 + 2), (100 + 2)), 0);
        let a = Array::from_iter(x.iter().filter_map(|&b| match b {
            b'1' => Some(1),
            b'0' => Some(0),
            _ => None,
        }));

        let a = a.into_shape((50, 100)).unwrap();
        grid.slice_mut(s![1..-1, 1..-1]).assign(&a);
        Map {layout: grid}
    }

    fn generate_walls(&mut self, number_of_walls: u8) {
        for _ in 0..number_of_walls{
            let rect: Rectangle = Rectangle::make_rand_rect();
            self.layout.slice_mut(s![rect.top_left_corner.x..rect.width+rect.top_left_corner.x+1,
                                    rect.top_left_corner.y..rect.length+rect.top_left_corner.y+1]
                                    ).assign(&rect.fill)
        }
    }
}


impl fmt::Display for Map{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f,"")?;
        let ascii_map = make_ascii_map();
        for row in self.layout.genrows() {
            for &value in row{
                match ascii_map.get(&value){
                    Some(&lookup) => write!(f,"{}", lookup)?,
                    _ => write!(f,"#")?,
                }
            }
            write!(f,"\n")?;
        }
        write!(f,"")
    }

}

struct Coordinate {
    x:usize,
    y:usize
}

impl Coordinate {
    pub fn new(xy:(i32, i32)) -> Coordinate {
        Coordinate{x:xy.1 as usize, y:xy.0 as usize }
    }
}

struct Rectangle {
    top_left_corner: Coordinate,
    length: usize,
    width: usize,
    fill: Array2<i32>
}

impl Rectangle {
    pub fn make_rand_rect() -> Rectangle {
        let mut rng = rand::thread_rng();
        let width = rng.gen_range(1,25);
        let length = rng.gen_range(1,25);
        let x = rng.gen_range(1,25);
        let y = rng.gen_range(1,75);
        let fill = Array2::ones((width+1,length+1));

        Rectangle {
            top_left_corner: Coordinate{x:x,y:y},
            length: length,
            width: width,
            fill: fill
        }
    }
}

struct Renderer{
    height_inv: Array2<usize>,
    dimensions: Coordinate,
    distances: Array2<usize>,
    ascii_map: HashMap<i32, char>,
    shades: usize,
    side_shades: usize,
    shade_diff: usize,
    const_vec: Array2<i32>,
    max_hops: usize
}

fn make_ascii_map() -> HashMap<i32, char> {
    let mut ascii_map = HashMap::new();
    for (i,thing) in " .,:;<+*LtCa4U80dQM@".chars().enumerate(){
        ascii_map.insert(i as i32,thing);
    }
    ascii_map
}

impl Renderer{
    pub fn new(dims:(i32, i32)) -> Renderer {
        let dimensions = Coordinate::new(dims);
        let height_inv = array!([0, 1/dimensions.y]); //might make weird things happen
        let distances =  Array::zeros((1,dimensions.x));
        let ascii_map = make_ascii_map();
        let shades = ascii_map.len() - 1;
        let side_shades = (shades +1)/5;
        Renderer {
            dimensions: dimensions,
            distances: distances,
            height_inv: height_inv,
            const_vec: array!([1, -1]),
            ascii_map: ascii_map,
            shades: shades,
            side_shades: side_shades,
            shade_diff:side_shades,
            max_hops: 60
        }
    }

}

fn main() {

    let mut room = Map::map_from_file(include_bytes!("room.txt"));
    room.generate_walls(10);

    let window = initscr();

    for _ in 0..10 {
        window.erase();
        let map_string = room.to_string();
        window.printw(map_string);
        window.getch();
        window.refresh();

    }
    endwin();
}
