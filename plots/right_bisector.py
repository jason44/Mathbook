from wrappers import *

### SMART IDEA BIG GUY: GET RID OF shape()

scene = PltScene(hscale=1.0, vscale=1.0)

rtri1 = PltPolygon(color=[0.8,0.0,0.2])
coords = [[0,0], [0,6], [8,6]]
rtri1.shape(coords)
rtri1.annotate_point('P', vertex=0, offset=DOWN*1.57)
rtri1.annotate_point('Q', vertex=1, offset=LEFT+UP)
rtri1.annotate_point('R', vertex=2, offset=(RIGHT+(UP*0.9))*0.9)

slope = 0.5 / -0.65
# y = mx  to  x = 8
line = PltPolygon(color=[0.8,0.0,0.2], ls='--')
coords2 = [[0,6], [4, 2*slope]]
line.shape(coords2)
line.annotate_point('M', vertex=1, offset=(RIGHT+DOWN)*1.2)
#line.annotate_point('M', vertex=0, offset=DOWN*1.5)

rangle1 = PltPolygon(color=[0.1,0.7,0.2], fill=False)
coords3 = [[0,6], [0.65, 6], [0.65, 5.5], [0, 5.5]]
rangle1.shape(coords3)

scene.append_polygon(rangle1)
scene.append_polygon(rtri1)
scene.append_polygon(line)
scene.plot()
#PltScene.show()
scene.save("right_bisector.pdf")
