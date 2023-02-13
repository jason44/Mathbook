import numpy as np
import matplotlib.pyplot as plt
import matplotlib.axes as axes
plt.figure(figsize=(5, 5))
plt.axis('off')
#data = random.random((5,5))
#img = plt.imshow(data, interpolation='nearest')
#img.set_cmap('hot')

coordt1 = [[0,0], [6,0], [0, 4]]
coordt1.append(coordt1[0]) #repeat the first point to create a 'closed loop'

xt1, yt1 = zip(*coordt1) #create lists of x and y values

coordt2 = [[0,4], [0,10], [4, 10]]
coordt2.append(coordt2[0]) 

xt2, yt2 = zip(*coordt2)

coordt3 = [[6,0], [10,0], [10, 6]]
coordt3.append(coordt3[0]) 

xt3, yt3 = zip(*coordt3)

coordt4 = [[4,10], [10,10], [10, 6]]
coordt4.append(coordt4[0]) 

xt4, yt4 = zip(*coordt4)

plt.annotate('A', xy=(0.0, -0.50), fontsize=13, weight='bold', va='center', ha='center')
plt.annotate('B', xy=(6.0, -0.50), fontsize=13, weight='bold', va='center', ha='center')
plt.annotate('C', xy=(-0.5, 4), fontsize=13, weight='bold', va='center', ha='center')
#plt.annotate('D', xy=(9.15, 3.15), fontsize=13, weight='bold', va='center', ha='center')

plt.plot(xt1, yt1, marker='o', markerfacecolor='r', markeredgecolor='r', lw=2)
plt.plot(xt2, yt2, marker='o', markerfacecolor='r', markeredgecolor='r', lw=2)
plt.plot(xt3, yt3, marker='o', markerfacecolor='r', markeredgecolor='r', lw=2)
plt.plot(xt4, yt4, marker='o', markerfacecolor='r', markeredgecolor='r', lw=2)
#plt.show()



#plt.savefig("test.png", bbox_inches='tight')
plt.savefig("pythagorean.pdf", bbox_inches='tight')
