from wrappers import *



scene = PltGridScene((-10, 10), (-10, 10), grid_linewidth=0.5, ticks_visible=False, tick_labels=False)
for i in range (1, 11):
    for j in range(1, 11):
        scene.dot(i, j, size=3)
        
#PltGridScene.show()
scene.save("test.pdf")

