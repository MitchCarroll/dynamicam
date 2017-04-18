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
use rand::{Rng, StdRng, SeedableRng};

struct Dimension {
    x: u16,
    y: u16,
}

struct Layer {
    size: Dimension,
    data: Vec<u8>,
}

impl Layer {
    fn checkindex(&self, x: u16, y: u16) {
        if !( x < self.size.x && y < self.size.y ) {
            panic!("Layer coordinate out of bounds ({}, {})",x,y);
        }
    }

    fn get(&self, x: u16, y: u16) -> u8 {
        &self.checkindex(x,y);

        self.data[ (y * self.size.x + x) as usize ]
    }

    fn set(&mut self, x: u16, y: u16, val: u8) {
        self.checkindex(x,y);
        
        self.data[ (y * self.size.x + x) as usize ] = val;
    }

    fn new(x: u16, y: u16) -> Layer {
        Layer {
            size: Dimension {
                x: x,
                y: y
            },
            data: vec![0u8;(x*y) as usize]
        }
    }
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
        Err(why) => panic!("File not found: {} - {}",&args[1],why),
    };
    let mut in_data = String::new();

    let _ = infile.read_to_string(&mut in_data);
    let mut split = in_data.split_whitespace();
    let output_size = Dimension {
                        x: split.next().unwrap().to_string().parse().unwrap(),
                        y: split.next().unwrap().to_string().parse().unwrap(),
    };
    let num_layers: u8 = split.next().unwrap().to_string().parse().unwrap();
    let num_colors: u8 = split.next().unwrap().to_string().parse().unwrap();
    let seed: &[_] = &[split.next().unwrap().to_string().parse().unwrap()];

    let mut rng: StdRng = SeedableRng::from_seed(seed);
    
    println!("Input file: {}", &args[1]);
    println!("Output file: {}", &args[2]);
    println!("Output size: {}X{}",output_size.x,output_size.y);
    println!("{} Layers, {} Colors",num_layers,num_colors);
    println!("Random Seed: {}",seed[0]);

    let mut layer_dimensions = vec![];
    for _ in 0..num_layers {
        layer_dimensions.push(read_layer_dimension(&mut split));
    }
    
    let mut colors = vec![];
    for _ in 0..num_colors {
        colors.push(split.next().unwrap());
    }

    let mut layers = vec![];
    for i in 0..num_layers as usize {
        layers.push(Layer::new(layer_dimensions[i].x, layer_dimensions[i].y));
        for a in 0..layers[i].size.x {
            for b in 0..layers[i].size.y {
                layers[i].set(a,b,(rng.next_u32() % num_colors as u32) as u8);
            }
        }
    }

    let mut output_image = Layer::new(output_size.x, output_size.y);

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
    Dimension{
        x: split.next().unwrap().parse().unwrap(), 
        y: split.next().unwrap().parse().unwrap()
    }
}
