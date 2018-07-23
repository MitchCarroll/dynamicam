#!/bin/bash
if ! which convert > /dev/null
then
	echo "Please install ImageMagick, and try again."
	exit -1
fi

if [ ! -d camo-output ];
then
	echo "creating camo-output directory"
	mkdir camo-output
else
	echo "clearing camo-output directory"
	rm camo-output/*
fi

if [ ! -d camo-backup ];
then
	echo "creating camo-backup directory"
	mkdir camo-backup
fi

echo "copying input files to output and backup directories"
for i in input*.*
do
	echo $i
	cp $i camo-backup
	cp $i camo-output
done

echo "changing working directory to camo-output"
cd camo-output

echo "converting files to 1024x1024 PNG files, and running FFT"
for i in *.*
do
	echo $i
	echo "resizing..."
	convert $i -alpha off -resize 1024x1024 -quality 100 -gravity center \
		${i%.*}-conv.png
	rm $i

	echo "running FFT..."
	convert ${i%.*}-conv.png -fft ${i%.*}-fft.png
	rm ${i%.*}-conv.png
done

# rm *fft-1.png
echo "generating average of FFT-0 images"
convert *-fft-0.png -evaluate-sequence mean fft-0-mean.png
rm *-fft-0.png

echo "creating grey image"
convert -size 1024x1024 xc:#808080 grey.png

echo "creating mask"
convert -size 1024x1024 xc:white -fill none -stroke xc:black +antialias \
	-strokewidth 1 -draw "circle 512,512 512,1024" circle.png

echo "applying mask"
convert fft-0-mean.png circle.png -compose Multiply -composite fft-0-mean.png
# convert grey.png circle.png -compose Multiply -composite fft-0-mean.png

rm circle.png

# echo "generating average of FFT-1 images"
# convert *-fft-1.png -evaluate-sequence mean fft-1-mean.png
# rm *-fft-1.png

echo "creating perlin layers"
for o in `seq 2 8`
do
	export i=$[2**$o]
	echo "$i"x"$i"
#	convert -size 1024x1024 xc:black -fill xc:white +antialias \
#		-draw "circle 512,512 512,$[512+i]" circle-$i.png
#		-draw "point 512,512" -blur 0x$i -contrast-stretch 0 circle-$i.png
#	convert fft-0-mean.png circle-$i.png -compose Multiply -composite \
#		fft-0-circle-$i.png
#	convert fft-1-mean.png circle-$i.png -compose Multiply -composite \
#		fft-1-circle-$i.png

#	rm circle-$i.png
	
	cp fft-0-mean.png fft-0-circle-$i.png

	echo "running IFT..."
	convert fft-0-circle-$i.png grey.png -ift ift-0-$i.png
	# convert grey.png fft-1-circle-$i.png -ift ift-1-$i.png
	# convert fft-0-circle-$i.png fft-1-circle-$i.png -ift ift-$i.png
	rm fft-?-circle-$i.png

	echo "scaling down..."
	convert ift-0-$i.png -resize "$i"x"$i" -quality 100 -gravity center \
		ift-0-$i.png

	echo "scaling up..."
	convert ift-0-$i.png -scale 1024x1024 ift-0-$i.png

	echo "dithering colors..."
	convert ift-0-$i.png +dither -colors 4 -quantize OHTA \
		ift-0-$i-dither.png
done
unset i

echo "cleaning up grey image"
rm grey.png

echo "averaging IFT images"
convert ift-0-*.png -evaluate-sequence mean ift.png
# convert ift-0-*-dither.png -evaluate-sequence mean ift-dither.png

echo "dithering colors"
convert ift.png -colors 4 -quantize OHTA -dither none camo.png
# convert ift.png -colors 4 -quantize OHTA -dither Riemersma camo.png
# convert ift-dither.png -colors 4 -quantize OHTA -dither Riemersma camo-dither.png
# rm ift.png

echo "DONE"
