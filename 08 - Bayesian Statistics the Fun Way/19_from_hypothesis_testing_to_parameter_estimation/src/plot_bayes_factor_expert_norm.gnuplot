################################################################################
#                               Plot Bayes Factor                              #
################################################################################

# The graphic title
set title "Bayes Factor For Various Prize Probabilities (Expert Data Normalised)"

# Hide the legend
set nokey

# Define the axis labels
set ylabel "Bayes Factor (normalised)"
set xtics nomirror out

set xlabel "Hypothesis Probability"
set xtics nomirror out

# Set the output file
set terminal png size 4000,3000 enhanced font "default,20"
set output './plots/bayes_factor_expert_norm.png'

plot "./data/hyp_1000_expert_norm.tsv" with lines \
    linewidth 3 \
    linecolor rgbcolor "#22dd3131"
