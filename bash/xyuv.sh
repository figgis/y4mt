#!/bin/bash

# Extract various frames from YCbCr 4:2:0 input
# using the mighty dd-command
#
# Frame numbers are zero-indexed.
#
# Usage:
#
#    ./xyuv.sh n input frame"
#    ./xyuv.sh n input WxH frame"
#    ./xyuv.sh nn input start stop"
#    ./xyuv.sh nn input WxH start stop"
#
# Example:
#    Extract frame 10 from a 720p file:
#    $ ./xyuv.sh n input_1280x720.yuv 10
#    $ ./xyuv.sh n input.yuv 1280x720 10
#
#    Extract frame 10-20 (inclusive) from a 1080p file:
#    $ ./xyuv.sh nn input_1920x1080.yuv 10 20
#    $ ./xyuv.sh nn input.yuv 1920x1080 10 20
#
# This can easily be adopted to suppport 4:2:2 or 4:4:4

usage () {
	cat <<- EOF
	Extract various frames from YCbCr 4:2:0 input
	Frame numbers are zero-indexed
	
	Usage:
	$0 n input frame
	$0 n input WxH frame
	$0 nn input start stop
	$0 nn input WxH start stop
	EOF
	exit 1
}

# Parse width and height from string
get_dim () {
	if [[ "$1" =~ ([0-9]+)x([0-9]+) ]]
	then
		WIDTH=${BASH_REMATCH[1]}
		HEIGHT=${BASH_REMATCH[2]}
	else
		echo "No dimension found..."
		usage
	fi
}

# Using input file name, generate output string
outn () {
	fname=$(basename "$IN")
	ext="${fname##*.}"
	fname="${fname%.*}"
	OUT=${fname}_${FRAME}.$ext
}

# Using input file name, generate output string
outnn () {
	fname=$(basename "$IN")
	ext="${fname##*.}"
	fname="${fname%.*}"
	OUT=${fname}_${START}-${STOP}.$ext
}

# Check input params
if [ $# -lt 3 ] || [ $# -gt 5 ]
then
	usage
fi

# n with dimension in filename
if [ "$1" == "n" ] && [ $# == 3 ]
then
	IN=$2
	FRAME=$3
	get_dim "$IN"
	outn
fi

# n with dimension specified
if [ "$1" == "n" ] && [ $# == 4 ]
then
	IN=$2
	get_dim "$3"
	FRAME=$4
	outn
fi

# nn with dimension in filename
if [ "$1" == "nn" ] && [ $# == 4 ]
then
	IN=$2
	START=$3
	STOP=$4
	get_dim "$IN"
	outnn
fi

# nn with dimension specified
if [ "$1" == "nn" ] && [ $# == 5 ]
then
	IN=$2
	get_dim "$3"
	START=$4
	STOP=$5
	outnn
fi

case "$1" in
	# n-th frame
	n)
		dd if="$IN" bs=$((WIDTH*HEIGHT*3/2)) count=1 skip="$FRAME" of="$OUT"
		;;
	nn)
		dd if="$2" bs=$((WIDTH*HEIGHT*3/2)) count=$((STOP-START+1)) skip="$START" of="$OUT"
		;;
	*)
		usage
esac
