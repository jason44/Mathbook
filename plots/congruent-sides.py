from wrappers import *

scene = PltScene(hscale=1.0, vscale=0.9)

rtri1 = PltPolygon(color=[0.8,0.0,0.2])
coords = [[0,0], [6,0], [3,4]]
rtri1.shape(coords)
rtri1.annotate_point('P', vertex=0, offset=LEFT+DOWN,)
rtri1.annotate_point('Q', vertex=1, offset=RIGHT+DOWN)
rtri1.annotate_point('R', vertex=2, offset=UP)

line = PltPolygon(color=[0.7,0.0,0.3])
coords2 = [[3,0], [3,4]]
line.shape(coords2)
line.annotate_point('M', vertex=0, offset=DOWN*1.5)

rangle1 = PltPolygon(color=[0.1,0.7,0.2], fill=False)
coords3 = [[3,0], [3,0.4], [2.5, 0.4], [2.5, 0]]
rangle1.shape(coords3)

rangle2 = PltPolygon(color=[0.1,0.7,0.2], fill=False)
coords4 = [[3,0], [3,0.4], [3.5, 0.4], [3.5, 0]]
rangle2.shape(coords4)

scene.append_polygon(rangle1)
scene.append_polygon(rangle2)
scene.append_polygon(rtri1)
scene.append_polygon(line)
scene.plot()
#PltScene.show()
scene.save("congruent_triangles.pdf")
