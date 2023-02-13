import numpy as np
import matplotlib.pyplot as plt
import matplotlib.axes as axes
plt.figure(figsize=(8, 5))
plt.axis('off')
#data = random.random((5,5))
#img = plt.imshow(data, interpolation='nearest')
#img.set_cmap('hot')

coord = [[0,0], [9,0], [9, 3], [0, 3]]
coord.append(coord[0]) #repeat the first point to create a 'closed loop'

xs, ys = zip(*coord) #create lists of x and y values

dividingline = [[0,0], [9, 3]]
xt, yt = zip(*dividingline)
#plt.annotate('A', xy=(-0.15, -0.15), fontsize=10, weight='bold', 
#             arrowprops=dict(arrowstyle="->", color='r'))
#plt.annotate('a', xy=(-0.15, -0.15))
plt.annotate('A', xy=(-0.15, -0.15), fontsize=13, weight='bold', va='center', ha='center')
plt.annotate('B', xy=(9.15, -0.15), fontsize=13, weight='bold', va='center', ha='center')
plt.annotate('C', xy=(-0.15, 3.15), fontsize=13, weight='bold', va='center', ha='center')
plt.annotate('D', xy=(9.15, 3.15), fontsize=13, weight='bold', va='center', ha='center')
plt.plot(xs, ys, marker='o', markerfacecolor='r', markeredgecolor='r', lw=3)
plt.plot(xt, yt, marker='o', markerfacecolor='r', markeredgecolor='r', lw=3)
#plt.show()



#plt.savefig("test.png", bbox_inches='tight')
plt.savefig("rect.pdf", bbox_inches='tight')
