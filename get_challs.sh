#!/bin/bash

for i in {1..100} ; do 
    wget -O challs/chall$i "https://decodingchallenge.org/Challenges/SD/SD_${i}0_0"
done
