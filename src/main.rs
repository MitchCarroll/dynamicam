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
use std::io;
use std::io::prelude::*;
use std::fs::File;
//use std::io::prelude::*;
//use std::path::Path;
//use std::error::Error;
//use rand::{Rng, SeedableRng, StdRng};

fn main()
{
    let mut args: env::Args = env::args();
    check_args(&args);

    let mut infile = match File::open(args.nth(1).unwrap_or_default()) {
        Ok(file) => file,
        Err(why) => panic!(why),
    };
    let mut in_data = String::new();

    infile.read_to_string(&mut in_data);
    let mut split = in_data.split_whitespace();
    let size_x: i16 = split.next().unwrap().to_string().parse().unwrap();
    let size_y: i16 = split.next().unwrap().to_string().parse().unwrap();
    let num_layers: i8 = split.next().unwrap().to_string().parse().unwrap();
    let num_colors: i8 = split.next().unwrap().to_string().parse().unwrap();
    let seed: i64 = split.next().unwrap().to_string().parse().unwrap();

    
    println!("{},{}",size_x,size_y);
    println!("{},{}",num_layers,num_colors);
    println!("{}",seed);

}

fn check_args(args: &env::Args)
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
        println!("USAGE: dynamicamo <infile> <outfile>");
        std::process::exit(1);
    }
}
