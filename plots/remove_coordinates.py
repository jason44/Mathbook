from numpy import random
import matplotlib.pyplot as plt

data = random.random((5,5))
img = plt.imshow(data, interpolation='nearest')
img.set_cmap('hot')
plt.axis('off')
plt.savefig("test.png", bbox_inches='tight')
