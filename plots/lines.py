dotted_line = plt.Line2D((2, 8), (4, 4), lw=5., 
                         ls='-.', marker='.', 
                         markersize=50, 
                         markerfacecolor='r', 
                         markeredgecolor='r', 
                         alpha=0.5)

line = plt.Line2D((2, 8), (6, 6), lw=2.5)
plt.gca().add_line(line)

