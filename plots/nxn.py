import numpy as np
import matplotlib.pyplot as plt
import matplotlib.axes as axes

plt.grid(color='b', linestyle='-', linewidth=2, alpha=0.5)
plt.xticks(np.arange(1, 11, step=1))
plt.yticks(np.arange(1, 11, step=1))

for i in range (1, 11):
    for j in range(1, 11):
        plt.plot(i, j, marker='o', markerfacecolor='r', markeredgecolor='r', lw=2)


#plt.show()
plt.savefig("nxn.pdf", bbox_inches='tight')