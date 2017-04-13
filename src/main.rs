/******************************************************************\
*   Copyright: C. Mitch Carroll                                    *
*      All rights reserved.                                        *
\******************************************************************/

/* 
dynamically creates a digital camoflage pattern in XPM format
based on a text input file containing the necessary parameters:
  Number of layers
  Dimensions of each layer
  Number of colors
  Random seed
  RGB values for each color
  Format as follows:

  SIZE_X SIZE_Y
  LAYERS COLORS
  SEED
  L1X L1Y
  L2X L2Y
  L3X L4Y
  ... ...
  RRGGBB
  RRGGBB
  RRGGBB
  ......

*/
extern crate rand;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::str::SplitWhitespace;
//use rand::{Rng, SeedableRng, StdRng};

struct Dimension {
    x: i16,
    y: i16,
}

fn main()
{
    let mut args = vec![];
    for (_,a) in env::args().enumerate() {
        args.push(a);
    }
    
    check_args(&args);

    let mut infile = match File::open(&args[1]){
        Ok(file) => file,
        Err(why) => panic!(why),
    };
    let mut in_data = String::new();

    infile.read_to_string(&mut in_data);
    let mut split = in_data.split_whitespace();
    let size_x: u16 = split.next().unwrap().to_string().parse().unwrap();
    let size_y: u16 = split.next().unwrap().to_string().parse().unwrap();
    let num_layers: u8 = split.next().unwrap().to_string().parse().unwrap();
    let num_colors: u8 = split.next().unwrap().to_string().parse().unwrap();
    let seed: u64 = split.next().unwrap().to_string().parse().unwrap();

    println!("Input file: {}",&args[1]); 
    println!("Output size: {}X{}",size_x,size_y);
    println!("{} Layers, {} Colors",num_layers,num_colors);
    println!("Random Seed: {}",seed);

    let mut layer_dimensions = vec![];
    for i in 0..num_layers {
        layer_dimensions.push(read_layer_dimension(&mut split));
    }
    
    for i in layer_dimensions {
        println!("{}, {}", i.x, i.y);
    }

}

fn check_args(args: &Vec<String>)
{
    if args.len() < 3 {
        println!("please provide an input file in the following format:");
        println!("SIZE_X SIZE_Y");
        println!("LAYERS COLORS" );
        println!("SEED");
        println!("L1X L1Y" );
        println!("L2X L2Y" );
        println!("L3X L4Y" );
        println!("... ..." );
        println!("RRGGBB");
        println!("RRGGBB" );
        println!("RRGGBB" );
        println!("......" );
        println!();
        println!("output is in XPM format" );
        println!("USAGE: dynamicam <infile> <outfile>");
        std::process::exit(1);
    }
}

fn read_layer_dimension(split: &mut SplitWhitespace) -> Dimension
{
    Dimension{x: split.next().unwrap().parse().unwrap(), y: split.next().unwrap().parse().unwrap()}
}
