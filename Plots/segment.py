import numpy as np
import matplotlib.pyplot as plt

plt.axis('off')
#data = random.random((5,5))
#img = plt.imshow(data, interpolation='nearest')
#img.set_cmap('hot')
x = np.arange(0.0, 5.0, 0.01)
y = (1/2)*x
s = plt.plot(x, y, lw=2)
plt.annotate('a', xy=(0,0))


#plt.savefig("test.png", bbox_inches='tight')
plt.savefig("test.pdf")
