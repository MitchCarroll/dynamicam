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
		-extent 1024x1024 ${i%.*}-square.png
	rm $i

	echo "running FFT..."
	convert ${i%.*}-square.png -fft ${i%.*}-fft.png
#	convert -size 1024x1024 xc:#808080 ${i%.*}-fft-0.png
	rm ${i%.*}-square.png
done

echo "generating average of FFT-0 images"
convert *-fft-0.png -evaluate-sequence mean fft-0-mean.png
rm *-fft-0.png

echo "generating average of FFT-1 images"
convert *-fft-1.png -evaluate-sequence mean fft-1-mean.png
 rm *-fft-1.png

echo "creating masks"
for i in 2 4 8 16 32; do
	convert -size 1024x1024 xc:black -fill xc:white +antialias \
		-draw "circle 512,512 512,$[512+i]" circle-"$i".png

#	convert circle-"$i".png -blur 0x"$[i/4]" circle-"$i".png
	convert circle-"$i".png -blur 0x32 circle-"$i".png

	convert circle-"$i".png -contrast-stretch 1 circle-$i.png
done

echo "applying masks"
for i in 2 4 8 16 32; do
	convert fft-0-mean.png circle-"$i".png \
		-compose Multiply -composite \
		fft-0-mask-"$i".png
	
	convert fft-1-mean.png circle-"$i".png \
		-compose Multiply -composite \
		fft-1-mask-"$i".png
#	cp fft-1-mean.png fft-1-mask-"$i".png

#	convert fft-0-mask-"$i".png -contrast-stretch 1 fft-0-mask-"$i".png
	convert fft-1-mask-"$i".png -contrast-stretch 1 fft-1-mask-"$i".png
done

echo "removing masks"
rm circle-*.png
rm fft-?-mean.png

echo "running inverse transforms"
for i in 2 4 8 16 32; do
	convert fft-0-mask-"$i".png fft-1-mask-"$i".png \
		-ift ift-"$i".png
	rm fft-0-mask-"$i".png
	rm fft-1-mask-"$i".png
done

echo "generating Perlin layers"
for i in 2 4 8 16 32; do
	convert ift-"$i".png -resize "$[i*i]"x"$[i*i]" layer-"$i".png
	convert layer-"$i".png -scale 1024x1024 layer-"$i".png
	rm ift-"$i".png
done

echo "averaging Perlin layers"
convert layer-*.png -evaluate-sequence mean pattern.png
rm layer-*.png

echo "quantizing colors"
convert pattern.png \
	-colors 4 \
	-dither FloydSteinberg \
	-quantize sRGB \
	camo.png
#	-quantize CMY \
#	-quantize sRGB \
#	-quantize GRAY \
#	-quantize XYZ \
#	-quantize LAB \
#	-quantize LUV \
#	-quantize HSL \
#	-quantize HSB \
#	-quantize HWB \
#	-quantize YIQ \
#	-quantize YUV \
#	-quantize OHTA \
	
#	-dither Riemersma \
#	-dither None \

	rm pattern.png

echo "DONE"
