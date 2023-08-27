#!/bin/bash

time target/release/mandelbrot mandel_0.png 4000x3000 -1.20,0.35 -1,0.20 F

time target/release/mandelbrot mandel_1.png 4000x3000 -1.20,0.35 -1,0.20 T
