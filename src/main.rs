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
    x: u32,
    y: u32,
}

struct Layer {
    size: Dimension,
    data: Vec<u8>,
}

impl Layer {
    fn checkindex(&self, x: u32, y: u32) {
        if !( x < self.size.x && y < self.size.y ) {
            panic!("Layer coordinate out of bounds ({}, {})",x,y);
        }
    }

    fn get(&self, x: u32, y: u32) -> u8 {
        &self.checkindex(x,y);

        self.data[ (y * self.size.x + x) as usize ]
    }

    fn set(&mut self, x: u32, y: u32, val: u8) {
        self.checkindex(x,y);
        
        self.data[ (y * self.size.x + x) as usize ] = val;
    }

    fn new(x: u32, y: u32) -> Layer {
        Layer {
            size: Dimension {
                x: x,
                y: y
            },
            data: vec![0u8; (x * y) as usize]
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
    for _ in 0 .. num_layers {
        layer_dimensions.push(read_layer_dimension(&mut split));
    }
    
    let mut colors = vec![];
    for _ in 0 .. num_colors {
        colors.push(split.next().unwrap());
    }

    let mut layers = vec![];
    for i in 0 .. num_layers as usize {
        layers.push(Layer::new(layer_dimensions[i].x, layer_dimensions[i].y));
        for y in 0 .. layers[i].size.y {
            for x in 0 .. layers[i].size.x {
                layers[i].set(x, y, (rng.next_u32() % (num_colors as u32)) as u8);
            }
        }
    }

    let mut output_image = Layer::new(output_size.x, output_size.y);

    for layer in layers {
        let scale = Dimension {
            x: output_size.x / layer.size.x,
            y: output_size.y / layer.size.y
        };
        
        for y in 0 .. output_size.y {
            for x in 0..output_size.x {
                let c = output_image.get(x,y);
                output_image.set(x, y, c + layer.get(x / scale.x, y / scale.y));
            }
        }
    }

    for y in 0 .. output_size.y {
        for x in 0 .. output_size.x {
            let n = output_image.get(x, y) / num_layers;
            output_image.set(x, y, n);
        }
    }

    let mut outfile: File = match File::create(&args[2]){
        Ok(file) => file,
        Err(why) => panic!("File not found: {} - {}",&args[2],why),
    };
    
    let _ = outfile.write_fmt(format_args!("/* XPM */\n"));
    let _ = outfile.write_fmt(format_args!("static char * camo_xpm[] = {{\n"));
    let _ = outfile.write_fmt(format_args!("\"{} {} {} 1\",\n", output_size.x, output_size.y, num_colors));
    for i in 0 .. num_colors {
        let _ = outfile.write_fmt(format_args!("\"{}\tc #{}\",\n", color_code(i), colors[i as usize])); 
    }

    for y in 0 .. output_size.y {
        let _ = outfile.write_fmt(format_args!("\""));
        for x in 0 .. output_size.x {
            let _ = outfile.write_fmt(format_args!("{}",color_code(output_image.get(x,y))));
        }
        let _ = outfile.write_fmt(format_args!("\",\n"));
    }

    let _ = outfile.write_fmt(format_args!("}};"));

    let _ = outfile.flush();
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

fn color_code(n: u8) -> char
{
    let mut n = n + 32;
    if n >= 34 { n += 1 }
    if n >= 92 { n += 1 }
    let c = (n as u8) as char;
    if n > 95 || n < 32 { panic!("{} out of range for color code", n); }
    if n == 34 || n == 92 { panic!("Color code {} produces invalid character: {}", n, c); }
    
    c
}
