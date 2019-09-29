
use ndarray::prelude::*;
use ndarray::array;
use std::fmt;
use std::collections::HashMap;
use pancurses::{initscr, endwin};
use rand::prelude::*;
use std::io::BufReader;
use std::fs::File;
use serde::Deserialize;
use std::path::Path;
use std::error::Error;


#[derive(Debug)]
struct Map{
    layout: Array2<i32>
}

pub fn map_from_file<P: AsRef<Path>>(path: P) -> Result<Map, Box<Error>>{
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
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

    let grid = map_from_file("map2d.json");

    let window = initscr();
    let mut rng = rand::thread_rng();
    // let grid = array![[1,1,1],[1,0,1],[1,1,1]];
    println!("{:#?}", grid.unwrap());
    // println!("{},{}",window.get_max_yx());
//     // let mut map: Map = Map {layout: grid };
//     let renderer = Renderer::new(window.get_max_yx());

//     for _ in 0..100 {
//         window.erase();
//         map.layout[[1,1]] = rng.gen_range(0,10);
//         // map.layout[[1,1]] = ' '.bytes();
//         let map_string = map.to_string();
//         window.printw(map_string);
//         window.getch();
//         window.refresh();

//     }
//     endwin();
}
