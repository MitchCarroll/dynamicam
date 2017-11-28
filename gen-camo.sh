#!/bin/bash
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
	convert $i -resize 256x256 -quality 100 -gravity center $i
	convert $i -fft ${i%.*}-fft.png
	rm $i
done

rm *-fft-1.png

convert -size 256x256 xc:#808080 grey.png

convert *-fft-0.png -evaluate-sequence mean fft-0-mean.png
rm *-fft-0.png

convert fft-0-mean.png grey.png -ift mean-ift-0.png
rm fft-0-mean.png

convert mean-ift-0.png -colors 5 -quantize YUV -dither Riemersma camo-out.png
rm mean-ift-0.png
rm grey.png

echo "DONE"
