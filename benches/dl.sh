#!/bin/sh

# Simple script to fetch some **large** images to test with

out="$(dirname "$(cargo locate-project | jq -r .root)")/target/benchimg"

image1="356036"
image2="1146708"
image3="1567069"
image4="1089194"

[ ! -d "$out" ] && mkdir -p "$out"

curl "https://images.pexels.com/photos/$image1/pexels-photo-$image1.jpeg" --output "$out/pexels-photo-$image1.jpeg"
curl "https://images.pexels.com/photos/$image2/pexels-photo-$image2.jpeg" --output "$out/pexels-photo-$image2.jpeg"
curl "https://images.pexels.com/photos/$image3/pexels-photo-$image3.jpeg" --output "$out/pexels-photo-$image3.jpeg"
curl "https://images.pexels.com/photos/$image4/pexels-photo-$image4.jpeg" --output "$out/pexels-photo-$image4.jpeg"
