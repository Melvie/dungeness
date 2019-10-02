
use ndarray::prelude::*;
use ndarray::{s,array};
use std::fmt;
use std::collections::HashMap;
use pancurses::{initscr, endwin};


mod map;

struct Renderer{
    height_inv: Array2<usize>,
    dimensions: map::Coordinate,
    distances: Array2<usize>,
    ascii_map: HashMap<i32, char>,
    shades: usize,
    side_shades: usize,
    shade_diff: usize,
    const_vec: Array2<i32>,
    max_hops: usize
}

impl Renderer{
    pub fn new(dims:(i32, i32)) -> Renderer {
        let dimensions = map::Coordinate::new(dims);
        let height_inv = array!([0, 1/dimensions.y]); //might make weird things happen
        let distances =  Array::zeros((1,dimensions.x));
        let ascii_map = map::make_ascii_map();
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

    let mut room = map::Map::map_from_file(include_bytes!("room.txt"));
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
