#!/bin/bash

for f in *.png;
do 
#	convert -monitor $f -strip -depth 32 -colorspace sRGB \
#		-color-matrix  '1.0 	 0.0 	0.0 
#				0.494207 0.0 	1.24827 
#				0.0 	 0.0 	1.0' \
#		+set profile "converted-$f";
	convert -monitor $f -strip -depth 32 -colorspace sRGB \
		-color-matrix  '0.5 	 0.5 	0.0 
				0.5	 0.5 	0.0 
				0.0 	 0.0 	2.0' \
		+set profile "converted-$f";
	rm $f;
done 
rename 's/converted-//g' *.png;
