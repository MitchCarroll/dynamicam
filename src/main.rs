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

//Stores the size of a layer
struct Dimension {
    x: u32,
    y: u32,
}

//Stores a layer
struct Layer {
    size: Dimension,
    data: Vec<f32>,
}

impl Layer {
    //Checks that the coordinates are valid for the layer (i.e.: in bounds)
    fn checkindex(&self, x: u32, y: u32) {
        if !( x < self.size.x && y < self.size.y ) {
            panic!("Layer coordinate out of bounds ({}, {})",x,y);
        }
    }

    //Returns the value at the given coordinates
    fn get(&self, x: u32, y: u32) -> f32 {
        &self.checkindex(x,y);
        self.data[ (y * self.size.x + x) as usize ]
    }

    //Sets the value at the given coordinates
    fn set(&mut self, x: u32, y: u32, val: f32) {
        self.checkindex(x,y);
        self.data[ (y * self.size.x + x) as usize ] = val;
    }

    //Constructor: initialized a new Layer of given size
    fn new(x: u32, y: u32) -> Layer {
        Layer {
            size: Dimension {
                x: x,
                y: y
            },
            data: vec![0.0f32; (x * y) as usize]
        }
    }
}

fn main()
{
    //Get command line arguments
    let mut args = vec![];
    for (_,a) in env::args().enumerate() {
        args.push(a);
    }
    
    //parse the command line arguments, and show usage error if wrong
    check_args(&args);

    //open the input file
    let mut infile = match File::open(&args[1]){
        Ok(file) => file,
        Err(why) => panic!("File not found: {} - {}",&args[1],why),
    };

    //split the infile up into tokens by whitespace
    let mut in_data = String::new();
    let _ = infile.read_to_string(&mut in_data);
    let mut split = in_data.split_whitespace();

    //the first 2 tokens are the output image size
    let output_size = Dimension {
                        x: split.next().unwrap().to_string().parse().unwrap(),
                        y: split.next().unwrap().to_string().parse().unwrap(),
    };

    //the next 2 tokens are the number of layers and number of colors
    let num_layers: u8 = split.next().unwrap().to_string().parse().unwrap();
    let num_colors: u8 = split.next().unwrap().to_string().parse().unwrap();

    //the next token is the random number seed
    let seed: &[_] = &[split.next().unwrap().to_string().parse().unwrap()];

    //Initialize the random number generator
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    
    //Print some output for user confirmation
    println!("Input file: {}", &args[1]);
    println!("Output file: {}", &args[2]);
    println!("Output size: {}X{}",output_size.x,output_size.y);
    println!("{} Layers, {} Colors",num_layers,num_colors);
    println!("Random Seed: {}",seed[0]);

    //Initialize and populate the layer_dimensions vector from the input file
    let mut layer_dimensions = vec![];
    for _ in 0 .. num_layers {
        layer_dimensions.push(read_layer_dimension(&mut split));
    }
    
    //Initialize and populate the colors vector from values in the input file
    let mut colors = vec![];
    for _ in 0 .. num_colors {
        colors.push(split.next().unwrap());
    }
    //Sort the colors vector by total value (brightness)
    colors.sort_by(|a, b| quantify_color(a.to_lowercase())
                            .partial_cmp(&quantify_color(b.to_lowercase()))
                            .unwrap());

    //Initialize the layers, and populate them with random values in each texel
    let mut layers = vec![];
    for i in 0 .. num_layers as usize {
        layers.push(Layer::new(layer_dimensions[i].x, layer_dimensions[i].y));
        for y in 0 .. layers[i].size.y {
            for x in 0 .. layers[i].size.x {
                layers[i].set(x, y, rng.next_f32());
            }
        }
    }

    //Initialize the output layer
    let mut output_image = Layer::new(output_size.x, output_size.y);

    for layer in layers {
        let scale = Dimension {
            x: output_size.x / layer.size.x,
            y: output_size.y / layer.size.y
        };
        
        for y in 0 .. output_size.y {
            for x in 0..output_size.x {
                let c = output_image.get(x,y);
                output_image.set(x, y, c + layer.get(x / scale.x, y / scale.y)); //accumulate layer texels into output layer texel
            }
        }
    }

    for y in 0 .. output_size.y {
        for x in 0 .. output_size.x {
            let n = ((output_image.get(x, y) / num_layers as f32) * num_colors as f32).round(); //calculate pixel color
            output_image.set(x, y, n);
        }
    }

    //Open (creating, if necessary) an output file (named by command line arg)
    let mut outfile: File = match File::create(&args[2]){
        Ok(file) => file,
        Err(why) => panic!("File not found: {} - {}",&args[2],why),
    };
    
    //Write XPM header to output file
    let _ = outfile.write_fmt(format_args!("/* XPM */\n"));
    let _ = outfile.write_fmt(format_args!("static char * camo_xpm[] = {{\n"));
    let _ = outfile.write_fmt(format_args!("\"{} {} {} 1\",\n", output_size.x, output_size.y, num_colors));

    //Write color codes to output file using color_code()
    for i in 0 .. num_colors {
        let _ = outfile.write_fmt(format_args!("\"{}\tc #{}\",\n", color_code(i as f32), colors[i as usize])); 
    }

    //Write pixels to output file
    for y in 0 .. output_size.y {
        let _ = outfile.write_fmt(format_args!("\""));
        for x in 0 .. output_size.x {
            let _ = outfile.write_fmt(format_args!("{}",color_code(output_image.get(x,y))));
        }
        let _ = outfile.write_fmt(format_args!("\",\n"));
    }

    let _ = outfile.write_fmt(format_args!("}};"));

    //Flush output data stream to guarantee data is properly saved
    let _ = outfile.flush();
}

//Checks command line arguments to make sure there are the proper number.
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

//Read a line from the input file, and determine the dimensions given for its layer
fn read_layer_dimension(split: &mut SplitWhitespace) -> Dimension
{
    Dimension{
        x: split.next().unwrap().parse().unwrap(), 
        y: split.next().unwrap().parse().unwrap()
    }
}

//Takes a number, and converts it to an XPM color code
fn color_code(n: f32) -> char
{
    let mut n: u8 = n as u8 + 32;
    if n >= 34 { n += 1 }
    if n >= 92 { n += 1 }
    let c = (n as u8) as char;
    if n > 95 || n < 32 { panic!("{} out of range for color code", n); }

    //skip invalid characters
    if n == 34 || n == 92 { panic!("Color code {} produces invalid character: {}", n, c); }
    
    c //return the computed character
}

//returns the total value of the given hex code (for sorting)
fn quantify_color(color: String) -> f32
{
    ((i32::from_str_radix(&color[0..1],16).unwrap() 
        + i32::from_str_radix(&color[2..3],16).unwrap() 
        + i32::from_str_radix(&color[4..5],16).unwrap()) as f32) 
    / (i32::from_str_radix("ffffff",16).unwrap() as f32)
}

