#!/bin/bash
if ! which convert > /dev/null
then
	echo "Please install ImageMagick, and try again."
	exit -1
fi

if [ ! -d camo-output ];
then
	mkdir camo-output
else
	rm camo-output/*
fi

if [ ! -d camo-backup ];
then
	mkdir camo-backup
fi

for i in *.png
do
	cp $i camo-backup
	cp $i camo-output
done
cd camo-output

for i in *.png
do
#	convert $i -alpha off -resize 1024x1024 -quality 100 -gravity center $i
	convert $i -fft ${i%.*}-fft.png
	rm $i
done

convert *-fft-0.png -evaluate-sequence mean fft-0-mean.png
rm *-fft-0.png

convert *-fft-1.png -evaluate-sequence mean fft-1-mean.png
rm *-fft-1.png

convert fft-0-mean.png fft-1-mean.png -ift ift.png
rm fft-?-mean.png

convert ift.png -colors 8 -quantize OHTA -dither Riemersma camo.png
rm ift.png

echo "DONE"
