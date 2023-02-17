import numpy as np
import matplotlib.pyplot as plt
import matplotlib.axes as axes
plt.figure(figsize=(8, 5))
plt.axis('off')
#data = random.random((5,5))
#img = plt.imshow(data, interpolation='nearest')
#img.set_cmap('hot')

t1 = [[0,0], [5,0], [5,3]]
t1.append(t1[0]) #repeat the first point to create a 'closed loop'

xs, ys = zip(*t1) #create lists of x and y values

t2 = [[0,0], [0,3], [5,3]]
t2.append(t2[0])
xt, yt = zip(*t2)

#plt.annotate('A', xy=(-0.15, -0.15), fontsize=10, weight='bold', 
#             arrowprops=dict(arrowstyle="->", color='r'))
#plt.annotate('a', xy=(-0.15, -0.15))
plt.annotate('A', xy=(-0.15, -0.15), fontsize=13, weight='bold', va='center', ha='center')
plt.annotate('B', xy=(5.15, -0.15), fontsize=13, weight='bold', va='center', ha='center')
plt.annotate('C', xy=(-0.15, 3.15), fontsize=13, weight='bold', va='center', ha='center')
plt.annotate('D', xy=(5.15, 3.15), fontsize=13, weight='bold', va='center', ha='center')

plt.plot(xs, ys, marker='o', markerfacecolor='r', markeredgecolor='r', lw=3, color=(1,0,0))
plt.fill(xs, ys, facecolor=(1,0,0,0.2))

plt.plot(xt, yt, marker='o', markerfacecolor='r', markeredgecolor='r', lw=3, color=(0,0,1))
plt.fill(xt, yt, facecolor=(0,0,1,0.2))
#plt.show()



#plt.savefig("test.png", bbox_inches='tight')
plt.savefig("rect.pdf", bbox_inches='tight')
