set title "Relation of Batch Size to Allocation Time"

set key off
set rmargin 5
set grid ytics noxtics nocbtics back
set border 3 back lw 2 lc rgbcolor "#222222"

set xlabel "Thread Spawned in Batch"
set xtics nomirror out
set xrange [0 to 10000]

set ylabel "Time for Batch to Return (ms)"
set yrange [10 to 400]
set ytics nomirror out

set terminal png size 4000,3000 enhanced font "default,20"
set output 'output_spin.png'

plot "spin.tsv" with points \
    pointtype 6 \
    pointsize 1.25 \
    linecolor rgbcolor "#22dd3131"
