from wrappers import *

scene = PltScene(hscale=1.0, vscale=1.5)

rtri1 = PltPolygon(color=[0.8,0.0,0.2])
coords = [[0,0], [6,0], [3,4]]
rtri1.shape(coords)


scene.append_polygon(rtri1)
scene.plot()
PltScene.show()
