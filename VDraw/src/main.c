#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <stdio.h>
#include <stdarg.h>
#include <math.h>
#include <complex.h>
#include <cairo/cairo.h>
#include <cairo/cairo-pdf.h>

// -lgsl -lgslcblas
// clang main.c -lcairo -lm 
/*
gcc main.c -lcairo -lm -Wall -O2
*/

#define _V_DEBUG_
#define _SUPPORT_UTF8_

typedef struct VDrawContext* VDrawContext;
typedef double complex vcomplex;
typedef float* RGBA;
typedef uint32_t VPolygon;
typedef uint32_t VAnnotation;

typedef double (*VFunction)(double);

typedef struct {
	float r, g, b, a;
} RGBA_t;

#define VDRAW_VEC(n) \
typedef double vvec##n[n]; \
 \
inline double vvec##n##_dot(const vvec##n u, const vvec##n v) \
{ \
	double dot = 0; \
	for (int i = 0; i < n; i++) \
		dot +=  u[i] * v[i]; \
	return dot; \
} \
\
inline double vvec##n##_len(const vvec##n u) \
{ \
	double inner = 0; \
	for (int i = 0; i < n; i++) \
		inner += pow(u[i], 2); \
	return sqrtf(inner); \
} \
 \
inline void vvec##n##_normalize(vvec##n u) \
{ \
	const double len = vvec##n##_len(u); \
	for (int i = 0; i < n; i++) \
		u[i] /= len; \
} \
 \
inline void vvec##n##_copy(vvec##n src, vvec##n dst) \
{ \
	memcpy(dst, src, sizeof(vvec##n)); \
} \

#define VDRAW_MAX_ANNOTATIONS 20
#define VDRAW_MAX_POLYGONS 10
#define VDRAW_STYLE_POLYGON (VDrawStyleInfo){}

#define V_RIGHT CMPLX(1.0f, 0.0f)
#define V_LEFT CMPLX(-1.0f, 0.0f)
#define V_UP CMPLX(0.0f, 1.0f)
#define V_DOWN CMPLX(0.0f, -1.0f)

#define V_I (vvec2){1.0, 0.0}
#define V_J (vvec2){0.0, 1.0}

#ifdef _SUPPORT_UTF8_
#define V_THETA "\xCE\xB8"
#define V_PHI "\xCF\x95"
#define V_SUB_ONE "\xE2\x82\x81"
#define V_SUP_ONE "\xC2\xB9"
#define V_INTEGRAL "\xE2\x88\xAB"
#endif

VDRAW_VEC(2)
VDRAW_VEC(3)
#define VVEC2(v) (vvec2){v[0], v[1]}
#define VVEC3(v) (vvec3){v[0], v[1], v[2]}

typedef vvec2 VPoint;
typedef vvec2 vmat2[2]; 
typedef vmat2 VEdge;	
typedef VEdge VLine;

/****************************
Defining a polygon:

vertex_count = 3;
vvec2 vertices[vertex_count] = {
	{0.0, 0.0},
	{1.0, 1.0},
	{0.5, 2.5}
};

Polygon poly = {
	.vertices = vertices,
	.vertex_count = vertex_count,
	.lw = 1.0,
	.color = [1.0, 1.0, 1.0, 1.0],
};
*******************************/

/* vdraw_set_style defaults if VDrawStyleInfo is empty */
typedef struct VDrawStyleInfo {
	cairo_fill_rule_t fillrule;
	cairo_line_cap_t linecap;
	cairo_line_join_t linejoin;
	bool dash;
	bool fill;
	double lw;
	RGBA_t color;	
} VDrawStyleInfo;

typedef struct VAnnotationInfo {
	double size;
	/* CAIRO_FONT_SLANT_NORMAL
	 * CAIRO_FONT_SLANT_OBLIQUE
	 * CAIRO_FONT_SLANT_ITALIC */
	cairo_font_slant_t slant;
	/* CAIRO_FONT_WEIGHT_NORMAL
	 * CAIRO_FONT_WEIGHT_BOLD */
	cairo_font_weight_t weight;
	const char *face;
} VAnnotationInfo;

struct VAnnotationInterface {
	const char *label;
	VAnnotationInfo style;
};

typedef struct VDrawCreateInfo {
	const char *filename;
	const double height;
	const double width;
} VDrawCreateInfo;

struct VDrawContext {
	cairo_surface_t *surface;
	cairo_t *cr;
	const char *filename;
	double height;
	double width;
};

struct VPolygonInterface {
	vvec2 *vertices;
	size_t vertex_count;
	VDrawStyleInfo style;
};

struct VPolygonInterface _VDRAW_POLYGONS[VDRAW_MAX_POLYGONS];
size_t _VDRAW_POLYGONS_SIZE = 0;

struct VAnnotationInterface _VDRAW_ANNOTATIONS[VDRAW_MAX_ANNOTATIONS];
size_t _VDRAW_ANNOTATIONS_SIZE = 0;

void vdraw_set_style(VDrawContext ctx, VDrawStyleInfo *style);

VAnnotation vannotation_create(const char *label, const VAnnotationInfo *info)
{
	assert(_VDRAW_ANNOTATIONS_SIZE < VDRAW_MAX_ANNOTATIONS);
	_VDRAW_ANNOTATIONS[_VDRAW_ANNOTATIONS_SIZE] = (struct VAnnotationInterface){
		.label = label,
		.style = *info
	};
	return _VDRAW_ANNOTATIONS_SIZE++;
}

void vannotation_set_style(VDrawContext ctx, VAnnotation annotation)
{
	struct VAnnotationInterface *anno = &_VDRAW_ANNOTATIONS[annotation];

	VDrawStyleInfo styleinfo = {};	
	vdraw_set_style(ctx, &styleinfo);

	if (anno->style.size) cairo_set_font_size(ctx->cr, anno->style.size);
	else cairo_set_font_size(ctx->cr, (ctx->width+ctx->height)*0.02);

	cairo_font_slant_t slant;
	if (anno->style.slant) slant = anno->style.slant;
	else slant = CAIRO_FONT_SLANT_NORMAL;

	cairo_font_weight_t weight;
	if (anno->style.weight) weight = anno->style.weight;
	else weight = CAIRO_FONT_WEIGHT_BOLD;

	/*("serif", "sans-serif", "cursive", "fantasy", "monospace")*/
	if (anno->style.face) cairo_select_font_face(ctx->cr, anno->style.face, slant, weight);
	else cairo_select_font_face(ctx->cr, "sans-serif", slant, weight);
}

/*********************************************
determinant is calculated with a 2x2 matrix where
|u_x, v_x|
|u_y, v_y}

if det(u, v) is positive, then v is to the right hand of u
if det(u, v) is negative, then v is to the left hand of u
*********************************************/
inline double vvec2_det(const vvec2 u, const vvec2 v)
{
	return (u[0] * v[1]) - (u[1] * v[0]);
}

inline void vvec2_flip_horizontal(vvec2 *vertices, size_t vertex_count)
{
	for (size_t i = 0; i < vertex_count; i++) 
		vertices[i][1] *= -1;
}

inline void vvec2_flip_vertical(vvec2 *vertices, size_t vertex_count)
{
	for (size_t i = 0; i < vertex_count; i++) 
		vertices[i][0] *= -1;
}

void vvec2_annotate(VDrawContext ctx, const vvec2 vertex, VAnnotation annotation)
{
	// cairo_text_path() <- text to path which can be filled 
	//	move_to reference point before turning text into path (you can't move it afterwords)
	// cairo_show_text or cairo_show_glyph <- faster than cairo_text_path()]
	struct VAnnotationInterface *anno = &_VDRAW_ANNOTATIONS[annotation];
	vannotation_set_style(ctx, annotation);
	cairo_text_extents_t te;
	cairo_text_extents(ctx->cr, anno->label, &te); 
    cairo_move_to (ctx->cr, vertex[0] - (te.width/2) , vertex[1] + (te.height/2));
	cairo_show_text(ctx->cr, anno->label);
	cairo_fill(ctx->cr);
}

/* keep it compatible for all structs that use vvec2 */ 
void vvec2_to_image_coordinates(VDrawContext ctx, 
	vvec2 *vertices, const size_t vertex_count)	 
{ \
	const double image_width = ctx->width / 2.0; 
	const double image_height = ctx->height / 2.0; 
	for (size_t i = 0; i < vertex_count; i++) { 
		vertices[i][0] += image_width; 
		vertices[i][1] += image_height; 
	} 
} 

void vvec2_annotate_to(VDrawContext ctx, const vvec2 vertex, 
	VAnnotation annotation, const double complex direction)
{
	double i = cimag(direction);
	double r = creal(direction);
	
	struct VAnnotationInterface *anno = &_VDRAW_ANNOTATIONS[annotation];
	vannotation_set_style(ctx, annotation);
	cairo_text_extents_t te;
	cairo_text_extents(ctx->cr, anno->label, &te); 
    cairo_move_to (ctx->cr, vertex[0] - (te.width/2) + (r*0.25), 
		vertex[1] + (te.height/2) - (i*0.25));
	cairo_show_text(ctx->cr, anno->label);
	cairo_fill(ctx->cr);
}

void vvec2_annotate_label(const char* label, VDrawContext ctx, const vvec2 vertex, 
	const VAnnotationInfo *info)
{
	const VAnnotationInfo *_info;
	const VAnnotationInfo temp_info = {};
	if (info) _info = info;
	else _info = &temp_info;
	VAnnotation anno = vannotation_create(label, _info);
	vvec2_annotate(ctx, vertex, anno);
}

void vvec2_annotate_label_to(const char* label, VDrawContext ctx, const vvec2 vertex, 
	const VAnnotationInfo *info, const double complex direction)
{
	const VAnnotationInfo *_info;
	const VAnnotationInfo temp_info = {};
	if (info) _info = info;
	else _info = &temp_info;
	VAnnotation anno = vannotation_create(label, _info);
	vvec2_annotate_to(ctx, vertex, anno, direction);
}

inline void vvec3_cross(vvec3 u, vvec3 v, vvec3 w)
{
	w[0] = (u[1]*v[2]) - (u[2]*v[1]);
	w[1] = ((u[0]*v[2]) - (u[2]*v[0])) * -1;
	w[2] = (u[0]*v[1])-(u[1]*v[2]);
}

#define vline_len vedge_len
inline double vedge_len(const VEdge edge)
{
	return vvec2_len((vvec2){edge[0][0] - edge[1][0], edge[0][1] - edge[1][1]});
}

#define vedge_from_vertices vline_from_point
inline void vline_from_point(const vvec2 u, const vvec2 v, VLine out)
{
	memcpy(out[0], u, sizeof(vvec2));
	memcpy(out[1], v, sizeof(vvec2));
}

inline VFunction vline_point(VDrawContext ctx, const VLine line, const double ratio_from_end)
{	
	// subtract origin from center to get distance from origin to the center of the segment
	const vvec2 origin = {ctx->width/2.0, ctx->height/2.0};
	const vvec2 center = {(line[0][0] + line[1][0]) / 2.0, (line[0][1] + line[1][1]) / 2.0};
	const vvec2 offset = {center[0] - origin[0], center[1] - origin[1]};

	const double slope = (line[0][1]-line[1][1])/(line[0][0]-line[1][0]);
	double xdist = (line[0][0] - line[1][0]) * ratio_from_end;
	vvec2 rpoint;
	if (line[1][1] > line[0][1]) {
		vvec2_copy((vvec2){line[0][0]+xdist, line[0][0]+(slope*xdist)}, rpoint);
	} else {
		vvec2_copy((vvec2){line[0][0]+xdist, line[0][0]-(slope*xdist)}, rpoint);
	}
	vdraw_dot(ctx , VVEC2(rpoint), NULL);
}

VPolygon vpolygon_create(vvec2 *vertices, const size_t vertex_count, 
	const VDrawStyleInfo *info)
{
	assert(_VDRAW_POLYGONS_SIZE < VDRAW_MAX_POLYGONS);
	_VDRAW_POLYGONS[_VDRAW_POLYGONS_SIZE] = (struct VPolygonInterface){
		.vertices = vertices,
		.vertex_count = vertex_count,
		.style = *info
	};
	return _VDRAW_POLYGONS_SIZE++;
}

void vpolygon_remove_closing_point(VPolygon polygon) {
	struct VPolygonInterface *poly = &_VDRAW_POLYGONS[polygon];
	if (poly->vertices[0][0] == poly->vertices[poly->vertex_count-1][0] &&
		poly->vertices[0][1] == poly->vertices[poly->vertex_count-1][1]) {
		--poly->vertex_count;
	}
}

VDrawContext vdraw_create(VDrawCreateInfo *info)
{
	assert(info->filename);
	assert(info->width);
	assert(info->height);
	VDrawContext ctx = (VDrawContext)malloc(sizeof(*ctx));
	assert(ctx);
	ctx->surface = cairo_pdf_surface_create(info->filename, info->width, info->height);
	ctx->cr = cairo_create(ctx->surface);
	ctx->filename = info->filename;
	ctx->width = info->width;
	ctx->height = info->height;
	cairo_set_source_rgba(ctx->cr, 1.0, 1.0, 1.0, 1.0);
	cairo_paint(ctx->cr);	
	return ctx;
}

void vdraw_destroy(VDrawContext ctx)
{
	cairo_destroy(ctx->cr);	
	cairo_surface_destroy(ctx->surface);
	free(ctx);
}

inline void vdraw_save(VDrawContext ctx) 
{
	cairo_show_page(ctx->cr);
}

void vdraw_set_style(VDrawContext ctx, VDrawStyleInfo *style)
{
	/*
	cairo_fill_rule_t fill;
	cairo_line_cap_t linecap;
	cairo_line_join_t linejoin;
	bool dash;
	bool fill;
	double lw;
	RGBA_t color;	
	*/
	RGBA_t c;
	if (style->color.r) c = style->color;
	else c = (RGBA_t){0.0, 0.0, 0.0, 1.0};
	cairo_set_source_rgba(ctx->cr, c.r, c.g, c.b, c.a);

	double lw;
	if (style->lw) lw = style->lw;
	else lw = 0.15;
	cairo_set_line_width(ctx->cr, lw);

	if (style->dash) {
		const double dashes[] = {10.0, 10.0};
		cairo_set_dash(ctx->cr, dashes, 2, -10.0);
	}
	
	if (style->fillrule) cairo_set_fill_rule(ctx->cr, style->fillrule);
	else cairo_set_fill_rule(ctx->cr, CAIRO_FILL_RULE_WINDING);
	
	if (style->linecap) cairo_set_line_cap(ctx->cr, style->linecap);
	else cairo_set_line_cap(ctx->cr, CAIRO_LINE_CAP_ROUND);
	
	if (style->linejoin) cairo_set_line_join(ctx->cr, style->linejoin);
	else cairo_set_line_join(ctx->cr, CAIRO_LINE_JOIN_ROUND);

}

void vdraw_dot(VDrawContext ctx, const vvec2 point, VDrawStyleInfo *style)
{
	vdraw_set_style(ctx, style);
	cairo_arc(ctx->cr, point[0], point[1], style->lw*1.6, 0.0, 2.0*M_PI);
	// swap red and blue for contrast
	cairo_set_source_rgba(ctx->cr, style->color.r, style->color.g, style->color.b, 1.0);
	cairo_fill(ctx->cr);
}

void vdraw_line(VDrawContext ctx, VLine line, VDrawStyleInfo *style)
{
	if (line[0][0] == line[1][0] && line[0][1] == line[1][1]) goto ERROR_EXIT;
	vdraw_set_style(ctx, style);
	cairo_move_to(ctx->cr, line[0][0], line[0][1]);
	cairo_line_to(ctx->cr, line[1][0], line[1][1]);
	cairo_stroke(ctx->cr);

	vdraw_dot(ctx, line[0], style);
	vdraw_dot(ctx, line[0], style);
	vdraw_dot(ctx, line[1], style);
	return;

	ERROR_EXIT: {
		vdraw_destroy(ctx);
		exit(EXIT_FAILURE);
	}
}

void vdraw_perpendicular_bisector(VDrawContext ctx, VLine line,
	const double distance, VDrawStyleInfo *style)
{
	const vvec2 bisector = {(line[0][0] + line[1][0]) / 2.0, (line[0][1] + line[1][1]) / 2.0};
	vvec2 u = {(line[0][0]-line[1][0])*-1, line[0][1]-line[1][1]};
	vvec2_normalize(u);
	VLine pb;
	vline_from_point(bisector, (vvec2){(u[0]+bisector[0])+distance-1, (u[1]+bisector[1])-distance+1}, pb);
	vdraw_line(ctx, pb, style);
}

void vdraw_polygon(VDrawContext ctx, VPolygon polygon)
{
	struct VPolygonInterface *poly = &_VDRAW_POLYGONS[polygon];
	if (!poly->vertices || !poly->vertex_count) goto ERROR_EXIT;

	vpolygon_remove_closing_point(polygon);
	vdraw_set_style(ctx, &poly->style);

	vvec2 *vertices = poly->vertices;
	size_t vertex_count = poly->vertex_count;

	for (size_t k = 0; k < vertex_count; k++) 
		printf("(%f, %f)\n", vertices[k][0], vertices[k][1]);	
	puts("--------------");

	cairo_move_to(ctx->cr, vertices[0][0], vertices[0][1]);
	for (size_t i = 1; i < vertex_count; i++) {
		cairo_line_to(ctx->cr, vertices[i][0], vertices[i][1]);			
	}
	// close the polygon 
	cairo_line_to(ctx->cr, vertices[0][0], vertices[0][1]);

	// preserve the path so cairo knows what to fill
	if (poly->style.fill) {
		RGBA_t c = poly->style.color;
		cairo_stroke_preserve(ctx->cr);
		cairo_set_source_rgba(ctx->cr, c.r, c.g, c.b, 0.15);
		cairo_fill(ctx->cr);
	} else cairo_stroke(ctx->cr);

	return;

	ERROR_EXIT: {
		vdraw_destroy(ctx);
		exit(EXIT_FAILURE);
	}
}

// specify vertex by index
void vdraw_polygon_angle(VDrawContext ctx, VPolygon polygon, const int vertex) 
{
	struct VPolygonInterface *poly = &_VDRAW_POLYGONS[polygon];
	if (!poly->vertices || !poly->vertex_count) goto ERROR_EXIT;
	if (vertex >= poly->vertex_count) goto ERROR_EXIT;

	vpolygon_remove_closing_point(polygon);
	vdraw_set_style(ctx, &poly->style);

	RGBA_t c;
	if (poly->style.color.r) c = poly->style.color;
	else c = (RGBA_t){0.0, 0.0, 0.0, 1.0};
	// inverse R and B for angles only
	cairo_set_source_rgba(ctx->cr, c.b, c.g, c.r, c.a);

	vvec2 R;
	vvec2_copy(poly->vertices[vertex], R);
	// signed integers are very nice nice nice
	vvec2 P;
	if (vertex-1 < 0) vvec2_copy(poly->vertices[poly->vertex_count-1], P);
	else vvec2_copy(poly->vertices[vertex-1], P);
	vvec2 Q;
	if (vertex+1 == poly->vertex_count) vvec2_copy(poly->vertices[0], Q);
	else vvec2_copy(poly->vertices[vertex+1], Q);
	//vvec2 P = poly->vertices[(vertex - 1) & (int)(poly->vertex_count)-1];
	//vvec2 Q = poly->vertices[(vertex + 1) % (int)(poly->vertex_count)];
	vvec2 u = {P[0]-R[0], P[1]-R[1]};
	vvec2 v = {Q[0]-R[0], Q[1]-R[1]};

	#ifdef _V_DEBUG_
	puts("_____________________");
	printf("INDEX: %i | (%f, %f)\n", vertex, R[0], R[1]);
	for (size_t i = 0; i < poly->vertex_count; i++) {
		printf("vertex %lu: (%f, %f)\n", i, poly->vertices[i][0], poly->vertices[i][1]);
	}
	puts("_____________________");
	printf("R(%f, %f) | P(%f, %f) | Q(%f, %f)\n", R[0], R[1], P[0], P[1], Q[0], Q[1]);
	#endif

	//cairo_new_path(cr);
	//cairo_move_to(cr, R[0], R[1]);

	double dot_uv = vvec2_dot(u, v);
	double angle_uv = acos(dot_uv/(vvec2_len(u)*vvec2_len(v)));
	printf("angle between u and v: %f\n", angle_uv);
	
	// NOTE: acos only gives angles between 0 and PI
	if (dot_uv != 0) {
		// dot(u, v)//||u||||v|| always returns the smallest angle between the two 
		double det_xv = vvec2_det(V_I, v);
		double det_xu = vvec2_det(V_I, u);
		double dot_ux = vvec2_dot(u, V_I);
		double dot_vx = vvec2_dot(v, V_I);

		#ifdef _V_DEBUG_
		printf("u = (%f, %f) v = (%f, %f)\n", u[0], u[1], v[0], v[1]);
		printf("dot_uv/(vvec2_len(u)*vvec2_len(v)): %f\n", 
			dot_uv/(vvec2_len(u)*vvec2_len(v)));
		printf("dot_ux/(vvec2_len(u)*vvec2_len(V_I): %f\n", 
			dot_ux/(vvec2_len(u)*vvec2_len(V_I)));
		printf("dot_vx/(vvec2_len(v)*vvec2_len(V_I): %f\n", 
			dot_vx/(vvec2_len(v)*vvec2_len(V_I)));
		#endif

		double angle_from_x = 0;
		if (det_xu >= 0) {
			// u is to the right hand of x
			double det_uv = vvec2_det(u, v);
			if (det_uv > 0) 
				// v is to the right hand of u
				//angle_from_x = acos(dot_ux/(vvec2_len(u)*vvec2_len(V_I))); 
				angle_from_x = acos(dot_ux/(vvec2_len(u))); 
			else if (det_uv < 0 && det_xv >= 0) 
				// v is to the left hand of u but to the right hand of x
				//angle_from_x = acos(dot_vx/(vvec2_len(v)*vvec2_len(V_I))); 
				angle_from_x = acos(dot_vx/(vvec2_len(v))); 
			else if (det_uv < 0 && det_xv < 0) 
				// v is to the left hand of u and x
				//angle_from_x = acos(dot_vx/(vvec2_len(v)*vvec2_len(V_I)))*-1; 
				angle_from_x = acos(dot_vx/(vvec2_len(v)))*-1; 
			else { // NOTE: remove block where u and v are parallel
				// u and v are parallel 
				if (dot_uv < 0) 
					// angle between u and v is 180
					//angle_from_x = acos(dot_ux/(vvec2_len(u)*vvec2_len(V_I))); 
					angle_from_x = acos(dot_ux/(vvec2_len(u))); 
				else return; // angle between u and v is 0
			}
			printf("POSITIVE: angle1: %f. angle2: %f\n", 
				angle_from_x, angle_from_x + angle_uv);
			cairo_arc(ctx->cr, R[0], R[1], (ctx->width+ctx->height)*0.025, 
				angle_from_x, angle_from_x + angle_uv);
			cairo_stroke(ctx->cr);
		} else if (det_xu < 0) {
			// u is to the left hand of x
			double det_vu = vvec2_det(v, u);
			if (det_vu < 0 && det_xv < 0) 
				// u is to the left hand of v and v is to the left hand of x
				//angle_from_x = acos(dot_vx/(vvec2_len(v)*vvec2_len(V_I)))*-1; 
				angle_from_x = acos(dot_vx/(vvec2_len(v)))*-1; 
			else if (det_vu < 0 && det_xv >= 0) 
				// u is to the left hand of v and v is to the right hand of x
				//angle_from_x = acos(dot_vx/(vvec2_len(v)*vvec2_len(V_I))); 
				angle_from_x = acos(dot_vx/(vvec2_len(v))); 
			else if (det_vu > 0 && det_xu < 0) 
				// u is to the right hand of v and left hand of x
				//angle_from_x = acos(dot_ux/(vvec2_len(u)*vvec2_len(V_I)))*-1; 
				angle_from_x = acos(dot_ux/(vvec2_len(u)))*-1; 
			else if (det_vu > 0 && det_xu >= 0) 
				// u is to the right hand of v and right hand of x
				//angle_from_x = acos(dot_ux/(vvec2_len(u)*vvec2_len(V_I))); 
				angle_from_x = acos(dot_ux/(vvec2_len(u))); 
			else { // NOTE: remove block where u and v are parallel because that is not possible in a valid polygon
				// u and v are parallel
				if (dot_uv < 0) 
					// angle between u and v is 180
					//angle_from_x = acos(dot_ux/(vvec2_len(u)*vvec2_len(V_I)))*-1; 
					angle_from_x = acos(dot_ux/(vvec2_len(u)))*-1; 
				else return; // angle between u and v is 0
			}
			printf("NEGATIVE: angle1: %f. angle2: %f\n",
				angle_from_x, angle_from_x - angle_uv);
			cairo_arc_negative(ctx->cr, R[0], R[1], (ctx->width+ctx->height)*0.025, 
				angle_from_x, angle_from_x - angle_uv);
			cairo_stroke(ctx->cr);
		}
	} else {
		// u and v intersect at a right angle  	
		vvec2 u_n = {u[0]/vvec2_len(u), u[1]/vvec2_len(u)};
		vvec2 v_n = {v[0]/vvec2_len(v), v[1]/vvec2_len(v)};
		const double imgr = (ctx->width + ctx->height)*0.025;
		const size_t angle_vcount = 5;
		vvec2 angle_vertices[5] = {
			{R[0], R[1]}, 
			{R[0]+(u_n[0]*imgr), R[1]+(u_n[1]*imgr)}, 
			{R[0]+((u_n[0]+v_n[0])*imgr), R[1]+((u_n[1]+v_n[1])*imgr)}, 
			{R[0]+(v_n[0]*imgr), R[1]+(v_n[1]*imgr)}, 
			{R[0], R[1]}
		};

		#ifdef _V_DEBUG_
		for (size_t i = 0; i < angle_vcount; i++) 
			printf("right_angles vertex: [%f, %f]\n", 
				angle_vertices[i][0], angle_vertices[i][1]);
		#endif

		VDrawStyleInfo style = {
			.lw = 0.05,
			.color = (RGBA_t){c.b, c.b, c.r, c.a},
			.fill = false
		};
		VPolygon angle_poly = vpolygon_create(angle_vertices, 
			angle_vcount, &style);
		vdraw_polygon(ctx, angle_poly);
	}	
	return;

	ERROR_EXIT: {
		puts("OUTPUT ERROR");
		vdraw_destroy(ctx);
		exit(EXIT_FAILURE);
	}
}

int main(int argc, char* argv[])
{
	VDrawCreateInfo info = {
		.filename = "test.pdf",
		.width = 10.0,
		.height = 10.0
	};
	VDrawContext ctx = vdraw_create(&info);


	VDrawStyleInfo style = {
		.lw = 0.05,
		.color = {0.75, 0.2, 0.3, 1.0},
		.fill = true
	};

	/*("serif", "sans-serif", "cursive", "fantasy", "monospace")*/
	VAnnotationInfo annoinfo1 = {};
	VAnnotationInfo annoinfo2 = {
		.size = (ctx->width+ctx->height)*0.015,
		.face = "serif"
	};
	/*
	const size_t vertex_count = 4;
	vvec2 vertices[] = {
		{4.0, 4.0},
		{0.0, 4.0},
		{0.0, -4.0},
		{4.0, 4.0}
	};
	vvec2_to_image_coordinates(ctx, vertices, vertex_count);

	VPolygon poly = vpolygon_create(vertices, vertex_count, &style);
	vdraw_polygon_angle(ctx, poly, 1);
	vdraw_polygon_angle(ctx, poly, 0);
	vdraw_polygon_angle(ctx, poly, 2);
	vdraw_polygon(ctx, poly);	
	//VAnnotation annotationd = vannotation_create("D", &annoinfo1);
	//vvec2_annotate_to(ctx, vertices[0], annotationd, (V_DOWN+V_RIGHT)*2.5);

	const size_t vertex_count2 = 4;
	vvec2 vertices2[] = {
		{-4.0, 4.0},
		{0.0, 4.0},
		{0.0, -4.0},
		{-4.0, 4.0}
	};
	vvec2_to_image_coordinates(ctx, vertices2, vertex_count2);

	VPolygon poly2 = vpolygon_create(vertices2, vertex_count2, &style);
	vdraw_polygon_angle(ctx, poly2, 1);
	vdraw_polygon_angle(ctx, poly2, 0);
	vdraw_polygon_angle(ctx, poly2, 2);
	vdraw_polygon(ctx, poly2);
	VAnnotation annotation1 = vannotation_create("h", &annoinfo1);
	puts("LIKE");
	VAnnotation annotation3 = vannotation_create(V_THETA, &annoinfo1);
	VAnnotation annotation4 = vannotation_create("b2", &annoinfo1);
	//vvec2_annotate(ctx, vertices2[2], annotation1);
	// TODO: fix indices changing after removing the endpoint (if endpoint is a closing point)
	vvec2_annotate_to(ctx, vertices2[1], annotation1, (V_UP*11.0)+(V_LEFT*1.3));
	vvec2_annotate_to(ctx, vertices2[1], annotation3, (V_DOWN*1.5)+(V_RIGHT*6.5));
	vvec2_annotate_to(ctx, vertices2[1], annotation4, (V_DOWN*1.5)+(V_LEFT*6.5));
	*/
	VLine line = {{-3.95, -3.95}, {3.95, 3.95}};
	vvec2_to_image_coordinates(ctx, (vvec2 *)&line, 2);
	vdraw_line(ctx, line, &style);
	VLine line2 = {{-2.56, 2.45}, {3.95, 3.95}};
	vvec2_to_image_coordinates(ctx, (vvec2 *)&line2, 2);
	vdraw_line(ctx, line2, &style);
	vdraw_perpendicular_bisector(ctx, line, 2.5, &style);
	vdraw_dot(ctx, (vvec2){0.0, 0.0}, &style);
	vline_point(ctx, line2, 0.25);
	vdraw_save(ctx);
	vdraw_destroy(ctx);
	
	return 0;
}

/*
 * TODO: keep track of the vertices that extend the farthest past the origin 
 * so we can crop the final image with that information
 */
