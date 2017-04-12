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

#include <iostream>
#include <fstream>
#include <string>
#include <cstdlib>
#include <cmath>
using namespace std;

const string pal[]={
  "0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f",
  "g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v",
  "w","x","y","z","A","B","C","D","E","F","G","H","I","J","K","L",
  "M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","`","~",
  ",","<",".",">","/","?",";",":","'","\"","!","@","#","$","%","^",
  "&","*","(",")","-","_","=","+","[","{","]","}","\\","|"};
//FIXME: load palette from file. Maybe not? This way, it should be baked long into
//       the binary, so it's probably just about optimal, unless we are going
//       for a small memory footprlong int, though this is pretty dang small to begin
//       with...

int main(int argc, char **argv)
{
  if(argc <= 1) {
      cout << "please provide an input file in the following format:" << endl
           << "SIZE_X SIZE_Y" << endl
           << "LAYERS COLORS" << endl
           << "SEED" << endl
           << "L1X L1Y" << endl
           << "L2X L2Y" << endl
           << "L3X L4Y" << endl
           << "... ..." << endl
           << "RRGGBB" << endl
           << "RRGGBB" << endl
           << "RRGGBB" << endl
           << "......" << endl
           << endl << "output is in XPM format" << endl;
  }
  ifstream infile; //the input file stream
  ofstream outfile;//the file our program will generate;
  long int size_x, size_y, num_colors, num_layers; //the number of colors and 
                                              //layers to create, and the 
                                              //final image size
  int seed;
  
  string *colors; //an array of colors
  long int **dimensions; //an array of the dimensions of each layer
  long int ***layers; //the container for all the layers
  long int **image; //final image array


  if(argc >= 2) 
    infile.open(argv[1]); //FIXME: use command line parsing, e.g. -in=
  else {
    cout << "no input file given" << endl
	 << "USAGE: dynamicamo <infile> <outfile>" << endl;
    exit(1);
  }
  
  infile >> size_x; //the first two items are the image size
  infile >> size_y;
  infile >> num_layers; //read number of layers
  infile >> num_colors; //read number of colors
  infile >> seed; //read the random seed
  layers = new long int**[num_layers]; //initialize the first dimension
  dimensions = new long int*[num_layers];
  for(long int l=0;l<num_layers;l++)
    dimensions[l]=new long int[2]; //dimensions array allocated
  for (long int layer=0;layer<num_layers;layer++) { //for each layer
    infile >> dimensions[layer][0]; //read the dimensions of the layer from
    infile >> dimensions[layer][1]; //...the file
    layers[layer] = new long int*[dimensions[layer][0]]; //dimension 2
    for(long int x=0;x<dimensions[layer][0];x++) //dimension 3
      layers[layer][x] = new long int[dimensions[layer][1]];
  } //layer arrays initialized
  colors = new string[num_colors]; //initialize color pallette array
  for(long int c=0;c<num_colors;c++) //for each color
    infile >> colors[c]; //read color string from file
  infile.close(); //we really shouldn't need it anymore

  //sort colors by value
  
  srandom(seed);  
  for(long int l=0;l<num_layers;l++) 
    for(long int x=0;x<dimensions[l][0];x++)
      for(long int y=0;y<dimensions[l][1];y++) 
	layers[l][x][y]=random()%(num_colors+1);
  image=new long int*[size_x];
  for(long int x=0;x<size_x;x++)
    image[x]=new long int[size_y];//the array for the final image data
  for(long int x=0;x<size_x;x++)
    for(long int y=0;y<size_y;y++)
      image[x][y]=0; //initialize all elements of the image to 0
  //Scale each layer to the final size, and add the layers
  for(long int l=0;l<num_layers;l++) 
    for(long int x=0;x<size_x;x++) 
      for(long int y=0;y<size_y;y++) 
	image[x][y]+=layers[l]
                       [x/(size_x/dimensions[l][0])]
                       [y/(size_y/dimensions[l][1])]; 
  //Average the values of the final array
  for(long int x=0;x<size_x;x++) 
    for(long int y=0;y<size_y;y++) 
      image[x][y]=image[x][y]/num_layers;
  //The pattern has now been generated. All that remains is to write it
  // to a file, and delete our structures.
  cout << argc << endl;
  for(long int a=0;a<argc;a++)
    cout << argv[a] << endl;
  cout << "opening outfile\n";
  if(argc >= 3)
    outfile.open(argv[2]); //FIXME: use command line parsing, e.g. --out=
  else {
    cout << "no output file given" << endl
	 << "USAGE: dynamicamo <infile> <outfile>" << endl;
    exit(1);
  }

  //Initialize file header
  outfile << "/* XPM */" << endl
	  << "static char * camo_xpm[] = {" << endl
	  << "\"" << size_x << " " << size_y << " " 
	  << num_colors << " 1\"," << endl;
  for(long int c=0;c<num_colors;c++)
    outfile << "\"" << pal[c] << " 	c #" << colors[c] << "\"," << endl;

  for(long int x=0;x<size_x;x++) {
    outfile << "\"";
    for(long int y=0;y<size_y;y++)
      outfile << pal[image[x][y]];
    outfile << "\"," << endl;
  }
  outfile << "};" << endl;
  outfile.close();
  //  delete layers,colors,dimensions, and image
  for(long int l=0;l<num_layers;l++) {
    for(long int x=0;x<dimensions[l][0];x++) 
      delete [] layers[l][x];
    delete [] layers[l];
  }
  delete [] layers;
  for(long int l=0;l<num_layers;l++) 
    delete [] dimensions[l];

  delete [] dimensions;

  delete [] colors;

  for(long int x=0;x<size_x;x++) 
    delete [] image[x];

  delete [] image;

  return 0;
}
