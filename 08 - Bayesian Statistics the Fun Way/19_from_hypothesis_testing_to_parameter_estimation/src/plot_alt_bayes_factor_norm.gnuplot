################################################################################
#                               Plot Bayes Factor                              #
################################################################################

# The graphic title
set title "Normalised Bayes Factor For Various Prize Probabilities (Alternative Hypothesis)"

# Hide the legend
set nokey

# Define the axis labels
set ylabel "Normalised Bayes Factor"
set xtics nomirror out

set xlabel "Hypothesis Probability"
set xtics nomirror out

# Set the output file
set terminal png size 4000,3000 enhanced font "default,20"
set output './plots/alt_bayes_factor_norm.png'

plot "./data/alt_hyp_1000_norm.tsv" with lines \
    linewidth 3 \
    linecolor rgbcolor "#22dd3131"
