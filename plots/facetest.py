import matplotlib.pyplot as plt

# Create the vertices of the triangle
x = [0, 1, 0.5]
y = [0, 0, 0.8]

# Set the background color to green
fig = plt.figure(facecolor='green')

# Create the triangle plot with red lines and green face
plt.fill(x, y, facecolor=(0,1,0, 0.5) , edgecolor='red', linewidth=2)

# Show the plot
plt.show()
