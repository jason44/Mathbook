import numpy as np
import matplotlib.pyplot as plt
import matplotlib.axes as axes
import matplotlib.patches as patches

RIGHT = 2.0 + 0j
LEFT = -2.0 + 0j
UP = 0 + 2.0j
DOWN = 0 - 2.0j


class PltPolygon:
	def __init__(self, color=[0, 0, 0], fill=True, fill_alpha=0.2, lw=1.0):
		self.color = color
		self.fill = fill
		self.fill_alpha = fill_alpha
		self.lw = lw
		self.annotations = []
	"""
 	coordinates: list(list(x, y))
	"""

	def shape(self, coordinates):
		# close the shape just in case it is not yet closed
		coordinates.append(coordinates[0])
		self.xs, self.ys = zip(*coordinates)

	# specify the vertice by index
	# add direction enums together to produce the desired offset
	# _scaling may need to be adjusted depending on the figure
	def annotate_point(self, text, vertex, offset=RIGHT+LEFT, _scaling=0.1, fontsize=12, weight='bold'):
		if vertex > (len(self.xs) - 1):
			return ValueError
		real = offset.real
		imag_coeff = offset.imag
		print(f"{text}, real: {real} imag: {imag_coeff}")
		pos = (self.xs[vertex]+(real*_scaling),
			   self.ys[vertex]+(imag_coeff*_scaling))
		self.annotations.append((text, pos, fontsize, weight))


class PltRectangle(PltPolygon):
	def __init__(self, center=(0, 0), x=1, y=1, color=[0, 0, 0], fill=True, fill_alpha=0.5):
		super().__init__(color, fill, fill_alpha)
		coordinates = [[center[0]-x, center[y]-y],
					   [center[0]+x, center[1]-y], [center[0]+x], center[1]+y]
		coordinates.append(coordinates[0])
		xs, ys = zip(*coordinates)
		self.shape(xs, ys)

# borders and figure scaline are broken until we can make


class PltScene:
	def __init__(self, xrange=(0, 10), yrange=(0, 10), border_color='#ffffff', border_linewidth=0.0,
			hscale=1.0, vscale=1.0):
		plt.axis('off')
		self.border_color = border_color
		self.border_linewidth = border_color
		self.hscale = hscale
		self.vscale = vscale
		self.polygons = []
		self.xrange = xrange
		self.yrange = yrange
		self.fig = plt.figure(figsize=(3*self.hscale, 3*self.vscale),
							  linewidth=border_linewidth)
		self.local_ax = self.fig.add_subplot(1, 1, 1)
		#self.local_ax.set_xlim([xrange[0], xrange[1]+0.5])
		#self.local_ax.set_ylim([yrange[0], yrange[1]+0.5])
		self.local_ax.axis('off')

	def limit_canvas(self, xrange=None, yrange=None):
		if xrange is None:
			xrange = self.xrange
		if yrange is None:
			yrange = self.yrange
		self.local_ax.set_xlim([xrange[0], xrange[1]+0.5])
		self.local_ax.set_ylim([yrange[0], yrange[1]+0.5])
		plt.xlim([xrange[0], xrange[1]+0.5])
		plt.ylim([yrange[0], yrange[1]+0.5])

	def origin(self):
		return (self.canvas[0]/2, self.canvas[1]/2)

	def append_polygon(self, p):
		self.polygons.append(p)

	def plot(self):
		for polygon in self.polygons:
			plt.plot(polygon.xs, polygon.ys, lw=polygon.lw,
					 color=(polygon.color[0], polygon.color[1], polygon.color[2]))
			if polygon.fill:
				plt.fill(polygon.xs, polygon.ys, facecolor=(polygon.color[0],
					polygon.color[1], polygon.color[2], polygon.fill_alpha))
			for annotation in polygon.annotations:
				text = annotation[0]
				pos = annotation[1]
				fontsize = annotation[2]
				weight = annotation[3]
				# https://stackoverflow.com/questions/11545062/matplotlib-autoscale-axes-to-include-annotations	
				"""
				pyplot doesn't know where the text will be until after its drawn. Therefore, we need to draw the 
				annotation, then resize the axis given the new information. Then, redraw with autoscale()
   				"""
				label = plt.annotate(text, xy=pos, fontsize=fontsize,
					weight=weight, va='center', ha='center')
				plt.draw()
				# returns bbox bounding the text
				bbox = label.get_window_extent()
				# .transformed constructs a bbox using the data coordinates from the parameter
				# axes.transData is the display coordinates of the axis
				# .inverted() will take the display coordinates and turn them into data coordinates
				bbox_data = bbox.transformed(self.local_ax.transData.inverted())
				# update the xlim and ylim with the corners of the label bbox if the bbox is outside the current datalim
				self.local_ax.update_datalim(bbox_data.corners())
				plt.autoscale()

	def dot(self, x, y, color='r', size=5.0):
		plt.plot(x, y, marker='o', markerfacecolor=color,
				 markeredgecolor=color, markersize=size)

	def circle(self, center=(0, 0), r=1, color=(0, 0, 1), face=True):
		c = plt.Circle(center, r, edgecolor=color, facecolor=(
			color[0], color[1], color[2], 0.3))
		self.local_ax.add_patch(c)

	def save(self, name):
		#plt.savefig(name, bbox_inches='tight', pad_inches=0)
		plt.tight_layout()
		plt.savefig(name, dpi=160)

	def show():
		plt.show()


class PltGridScene(PltScene):
	def __init__(self, xrange=(-10, 10), yrange=(-10, 10), grid_linewidth=1.0, grid_color='b', grid_alpha=0.3,
				 tick_steps=1, ticks_visible=True, tick_labels=True, borders=True):
		if borders:
			super().__init__(xrange, yrange, '#363636', 2, 2.0, 2.0)
		else:
			super().__init__(xrange, yrange)
		self.ax = self.fig.add_subplot(1, 1, 1)
		self.ax.spines['left'].set_position('zero')
		self.ax.spines['bottom'].set_position('zero')
		self.ax.spines['right'].set_color('none')
		self.ax.spines['top'].set_color('none')
		self.ax.set_xlim([xrange[0], xrange[1]+0.5])
		self.ax.set_ylim([yrange[0], yrange[1]+0.5])
		self.ax.grid(True, color=grid_color, alpha=grid_alpha,
					 linewidth=grid_linewidth)
		self.ax.set_xticks(np.arange(xrange[0], xrange[1]+1, step=tick_steps))
		self.ax.set_yticks(np.arange(yrange[0], yrange[1]+1, step=tick_steps))
		# self.ax.set_yticklabels()
		if ticks_visible:
			self.ax.tick_params(axis='both', which='major', labelsize=8)
		else:
			self.ax.tick_params(axis='both', which='major', bottom=False, top=False,
								right=False, left=False, labelbottom=False, labelleft=False)
		if not tick_labels:
			self.ax.xaxis.set_ticklabels([])
			self.ax.yaxis.set_ticklabels([])
