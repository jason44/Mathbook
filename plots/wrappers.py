import numpy as np
import matplotlib.pyplot as plt
import matplotlib.axes as axes


class PltPolygon:
	def __init__(self, color=[0,0,0], fill=True, fill_alpha=0.5):
		self.color = color
		self.fill = fill
		self.fill_alpha = fill_alpha
  
	def shape(self, xs, ys):
		self.xs = xs
		self.ys = ys

class PltRectangle(PltPolygon):
	def __init__(self, center=(0, 0), x=1, y=1, color=[0,0,0], fill=True, fill_alpha=0.5):
		super().__init__(color, fill, fill_alpha)
		coordinates = [[center[0]-x, center[y]-y], [center[0]+x, center[1]-y], [center[0]+x], center[1]+y]
		coordinates.append(coordinates[0])
		xs, ys = zip(*coordinates)
		self.shape(xs, ys)

	
# borders and figure scaline are broken until we can make 
class PltScene:	
	def __init__(self, xrange=(0, 10), yrange=(0, 10), border_color='#ffffff', border_linewidth=0.0, 
    			hscale=1.0, vscale=1.0):
		self.border_color = border_color
		self.border_linewidth = border_color
		self.hscale = hscale
		self.vscale = vscale
		self.polygons = []
		self.xrange = xrange
		self.yrange = yrange
		self.fig = plt.figure(figsize=(3*self.hscale, 3*self.vscale), 
			linewidth=border_linewidth)
		#plt.xlim(self.xrange[0], self.xrange[1])
		#plt.ylim(self.yrange[0], self.yrange[1])
	
	def origin(self):
		return (self.canvas[0]/2, self.canvas[1]/2)
 
	def append_polygon(self, p):
		self.polygons.append(p)  
	
	def plot(self):
		for polygon in self.polygons:
			plt.plot(polygon.xs, polygon.ys, color=(polygon.color[0], polygon.color[1], polygon.color[2]))
			if polygon.fill:
				plt.fill(polygon.xs, polygon.ys, facecolor=(polygon.color[0], 
					polygon.color[1], polygon.color[2], polygon.fill_alpha))
	
	def dot(self, x, y, color='r', size=5.0):
		plt.plot(x, y, marker='o', markerfacecolor=color, markeredgecolor=color, markersize=size)

	def save(self, name):
		plt.savefig(name, bbox_inches='tight', pad_inches=0, 
			edgecolor=self.border_color)
  
	def show():
		plt.show()

  
class PltGridScene(PltScene):
	def __init__(self, xrange=(-10, 10), yrange=(-10, 10), grid_linewidth=1.0, grid_color='b', grid_alpha=0.3, 
        		tick_steps=1, ticks_visible=True, tick_labels=True, borders=True):
		if borders:
			super().__init__(xrange, yrange, '#363636', 2, 2.0, 2.0)
		else:
			super().__init__(xrange, yrange)
		self.ax = self.fig.add_subplot(1,1,1)
		self.ax.spines['left'].set_position('zero')
		self.ax.spines['bottom'].set_position('zero')
		self.ax.spines['right'].set_color('none')
		self.ax.spines['top'].set_color('none')
		self.ax.set_xlim([xrange[0], xrange[1]+0.5])
		self.ax.set_ylim([yrange[0], yrange[1]+0.5])
		self.ax.grid(True, color=grid_color, alpha=grid_alpha, linewidth=grid_linewidth)
		self.ax.set_xticks(np.arange(xrange[0], xrange[1]+1, step=tick_steps))
		self.ax.set_yticks(np.arange(yrange[0], yrange[1]+1, step=tick_steps))
		#self.ax.set_yticklabels()
		if ticks_visible:
			self.ax.tick_params(axis='both', which='major', labelsize=8)
		else:	
			self.ax.tick_params(axis='both', which='major', bottom=False, top=False, 
                       			labelbottom=False, labelleft=False)
		if not tick_labels:
			print("HEY")
			self.ax.xaxis.set_ticklabels([])
			self.ax.yaxis.set_ticklabels([])
	
	
