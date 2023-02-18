
# Create a figure and set the size
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.axes as axes

fig = plt.figure(figsize=(6,6), linewidth=2)
#plt.grid(color='b', linestyle='-', linewidth=1, alpha=0.5)
# Create a plot with the four quadrants
ax = fig.add_subplot(1,1,1)
ax.spines['left'].set_position('zero')
ax.spines['bottom'].set_position('zero')
ax.spines['right'].set_color('none')
ax.spines['top'].set_color('none')
ax.set_xlim([-10, 10.5])
ax.set_ylim([-10, 10.5])

# Add grid lines
ax.grid(True, color='b', alpha=0.3)
ax.set_xticks(np.arange(-10, 11, step=1))
ax.set_yticks(np.arange(-10, 11, step=1))
#ax.set_yticklabels()
ax.tick_params(axis='both', which='major', labelsize=8)
ax.xaxis.set_ticklabels([])
ax.yaxis.set_ticklabels([])

for i in range (1, 11):
    for j in range(1, 11):
        plt.plot(i, j, marker='o', markerfacecolor='r', markeredgecolor='r', lw=1)
# Add axis labels
#ax.set_xlabel('X-axis')
#ax.set_ylabel('Y-axis')

# Add a title
#ax.set_title('Four Quadrant Grid')

# Show the plot
#plt.show()
plt.savefig("nxn.pdf", bbox_inches='tight', pad_inches=0, edgecolor='#ffffff')