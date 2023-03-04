#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <stdio.h>
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
#define IMAGE_COORDINATTES

#define VDRAW_STYLE_POLYGON (VDrawSettings){}

#define V_I (vec2){1.0, 0.0}
#define V_J (vec2){0.0, 1.0}

#define VRIGHT CMPLX(1.0f, 0.0f)
#define VLEFT CMPLX(-1.0f, 0.0f)
#define VUP CMPLX(0.0f, 1.0f)
#define VDOWN CMPLX(0.0f, -1.0f)

typedef double complex vcomplex;
typedef struct Edge Line;
typedef float* RGBA;

typedef struct {
	float r, g, b, a;
} RGBA_t;

typedef struct VDrawSettings {
	cairo_fill_rule_t fill;
	cairo_line_cap_t linecap;
	cairo_line_join_t linejoin;
	bool dash;
	bool fill;
	double lw;
	RGBA_t color;	
} VDrawSettings;

typedef struct VDrawContext {
	cairo_surface_t *surface;
	cairo_t *cr;
	const char *filename;
	const double height;
	const double width;
	bool image_coordinates;
} VDrawContext;

typedef struct vec2 {
	float x, y;
} vec2;

typedef struct Edge {
	vec2 p, q;
} Edge;

/****************************
Defining a polygon:

vertex_count = 3;
vec2 vertices[vertex_count] = {
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

dash and fill are optional
****************************/
typedef struct Polygon {
	vec2 *vertices;
	size_t vertex_count;
	VDrawSettings settings;
} Polygon;


inline double vec2_dot(const vec2 u, const vec2 v) 
{
	return (u.x * v.x) + (u.y * v.y);
}

inline double vec2_length(const vec2 u)
{
	return sqrt(pow(u.x, 2) + pow(u.y, 2));
}

/*********************************************
determinant is calculated with a 2x2 matrix where
|u_x, v_x|
|u_y, v_y}

if det(u, v) is positive, then v is to the right hand of u
if det(u, v) is negative, then v is to the left hand of u
*********************************************/
inline double vec2_det(const vec2 u, const vec2 v)
{
	return (u.x * v.y) - (u.y * v.x);
}

inline double edge_length(const Edge edge)
{
	return vec2_length((vec2){edge.p.x - edge.q.x, edge.p.y - edge.q.y});
}

#ifdef IMAGE_COORDINATTES
void vec2_to_image_coordinates(double image_width, double image_height, 
	vec2 *verticies, const size_t vertex_count)	
{
	image_width /= 2.0;
	image_height /= 2.0;
	for (size_t i = 0; i < vertex_count; i++) {
		verticies[i].x = verticies[i].x + image_width;
		verticies[i].y = (verticies[i].y + image_height) -1;
	}
}
#endif

/*********** FREE THE ALLOCATED ARRAY ***********/
void polygon_calculate_edges(Polygon *poly, Edge *edges)
{
	assert(poly->vertices[1].x);
	for (size_t i = 1; i < poly->vertex_count; i ++) {
		edges[i] = (Edge){poly->vertices[i], poly->vertices[i-1]
		};	
	}
}

/*********** FREE THE ALLOCATED ARRAY ***********/
size_t polygon_calculate_edges_as_vectors(Polygon *poly, size_t *edge_count)
{
	assert(poly->vertices[1].x);

	// close the polygon unless it is already closed
	vec2 *vertices = poly->vertices;
	size_t vertex_count = poly->vertex_count;
	vec2 buf[vertex_count+1];
	if (vertices[0].x != vertices[vertex_count-1].x &&
		vertices[0].y != vertices[vertex_count-1].y) {
		memcpy(buf, vertices, sizeof(vec2)*(vertex_count));
		buf[vertex_count] = (vec2){vertices[0].x, vertices[0].y};
		vertices = buf;
		++vertex_count;
	}

	vec2 *edges = malloc(sizeof(vec2)*(vertex_count-1));
	for (size_t i = 1; i < poly->vertex_count; i++) {
		edges[i] = (vec2){poly->vertices[i].x - poly->vertices[i-1].x, 
			poly->vertices[i].y - poly->vertices[i-1].y
		};	
	}
	*edge_count = vertex_count-1;
	return edges;
}

void polygon_remove_closing_point(Polygon *poly) {
	if (poly->vertices[0].x == poly->vertices[poly->vertex_count-1].x &&
		poly->vertices[0].y == poly->vertices[poly->vertex_count-1].y) {
		--poly->vertex_count;
	}
}

void vdraw_create(VDrawContext *ctx)
{
	assert(ctx->filename);
	assert(ctx->width);
	assert(ctx->height);
	ctx->surface = cairo_pdf_surface_create(ctx->filename, ctx->width, ctx->height);
	ctx->cr = cairo_create(ctx->surface);
	cairo_paint(ctx->cr);	
}

void vdraw_destroy(VDrawContext *ctx)
{
	cairo_destroy(ctx->cr);	
	cairo_surface_destroy(ctx->surface);
}

inline void vdraw_save(VDrawContext *ctx) 
{
	cairo_show_page(ctx->cr);
}

void vdraw_set_style(VDrawContext *ctx, VDrawSettings *settings)
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
	cairo_t *cr = ctx->cr;
	RGBA c;
	if (settings->color.r) c = &settings->color;
	else c = (RGBA)(&(RGBA_t){0.0, 0.0, 0.0, 1.0});
	cairo_set_source_rgba(cr, c[0], c[1], c[2], c[3]);

	double lw;
	if (settings->lw) lw = settings->lw;
	else lw = 0.15;
	cairo_set_line_width(cr, lw);

	if (settings->dash) {
		const double dashes[] = {10.0, 10.0};
		cairo_set_dash(cr, dashes, 2, -10.0);
	}
	
	if (settings->fill) cairo_set_fill_rule(ctx->cr, settings->fill);
	else cairo_set_fill_rule(ctx->cr, CAIRO_FILL_RULE_WINDING);
	
	if (settings->linecap) cairo_set_line_cap(ctx->cr, settings->linecap);
	else cairo_set_line_cap(ctx->cr, CAIRO_LINE_CAP_ROUND);
	
	if (settings->linejoin) cairo_set_line_join(ctx->cr, settings->linejoin);
	else cairo_set_line_join(ctx->cr, CAIRO_LINE_JOIN_ROUND);

}

void vdraw_line(VDrawContext *ctx, Line line, VDrawSettings *settings );
{
	if (edge.p == edge.q) goto ERROR_EXIT;
	vdraw_set_style(ctx, settings);
	cairo_move_to(ctx->cr, edge.p.x, edge.p.y);
	cairo_line_to(ctx->cr, edge.q.x, edge.q.y);
	cairo_stroke(cr);
	return;

	ERROR_EXIT: {
		vdraw_destroy(ctx);
		exit(EXIT_FAILURE);
	}
}

void vdraw_polygon(VDrawContext *ctx, Polygon *poly)
{
	if (!poly->vertices || !poly->vertex_count) goto ERROR_EXIT;

	polygon_remove_closing_point(poly);
	vdraw_set_style(ctx, &poly->settings);

	if (!ctx->image_coordinates) {
		vec2_to_image_coordinates(ctx->width, ctx->height, 
			poly->vertices, poly->vertex_count);
		ctx->image_coordinates = true;
	}

	vec2 *vertices = poly->vertices;
	size_t vertex_count = poly->vertex_count;

	for (size_t k = 0; k < vertex_count; k++) printf("(%f, %f)\n", 
		vertices[k].x, vertices[k].y);	
	puts("--------------");

	cairo_move_to(ctx->cr, vertices[0].x, vertices[0].y);
	for (size_t i = 1; i < vertex_count; i++) {
		cairo_line_to(ctx->cr, vertices[i].x, vertices[i].y);			
	}
	// close the polygon 
	cairo_line_to(ctx->cr, vertices[0].x, vertices[0].y);

	// preserve the path so cairo knows what to fill
	if (poly->settings.fill) {
		cairo_stroke_preserve(ctx->cr);
		cairo_set_source_rgba(ctx->cr, c[0], c[1], c[2], 0.2);
		cairo_fill(ctx->cr);
	} else cairo_stroke(ctx->cr);

	return;

	ERROR_EXIT: {
		vdraw_destroy(ctx);
		exit(EXIT_FAILURE);
	}
}

// specify vertex by index
void vdraw_polygon_angle(VDrawContext *ctx, Polygon *poly, const int vertex) 
{
	if (!poly->vertices || !poly->vertex_count) goto ERROR_EXIT;
	if (vertex >= poly->vertex_count) goto ERROR_EXIT;

	polygon_remove_closing_point(poly);
	vdraw_set_style(ctx, &poly->settings);

	if (!ctx->image_coordinates) {
		vec2_to_image_coordinates(ctx->width, ctx->height, 
			poly->vertices, poly->vertex_count);
		ctx->image_coordinates = true;
	}

	// modify the color values
	cairo_t *cr = ctx->cr;
	RGBA c;
	if (poly->settings.color.r) c = &poly->settings.color;
	else c = (RGBA)(&(RGBA_t){0.0, 0.0, 0.0, 1.0});
	// inverse R and B for angles only
	cairo_set_source_rgba(cr, c[2], c[1], c[0], c[3]);


	vec2 R = poly->vertices[vertex];
	// signed integers are very nice nice nice
	vec2 P;
	if (vertex-1 < 0) P = poly->vertices[poly->vertex_count-1];
	else P = poly->vertices[vertex-1];
	vec2 Q;
	if (vertex+1 == poly->vertex_count) Q = poly->vertices[0];
	else Q = poly->vertices[vertex+1];
	//vec2 P = poly->vertices[(vertex - 1) & (int)(poly->vertex_count)-1];
	//vec2 Q = poly->vertices[(vertex + 1) % (int)(poly->vertex_count)];
	vec2 u = {P.x-R.x, P.y-R.y};
	vec2 v = {Q.x-R.x, Q.y-R.y};

#ifdef _V_DEBUG_
	puts("_____________________");
	printf("INDEX: %i | (%f, %f)\n", vertex, R.x, R.y);
	for (size_t i = 0; i < poly->vertex_count; i++) {
		printf("vertex %lu: (%f, %f)\n", i, poly->vertices[i].x, poly->vertices[i].y);
	}
	puts("_____________________");
	printf("R(%f, %f) | P(%f, %f) | Q(%f, %f)\n", R.x, R.y, P.x, P.y, Q.x, Q.y);
#endif

	//cairo_new_path(cr);
	//cairo_move_to(cr, R.x, R.y);

	double dot_uv = vec2_dot(u, v);
	double angle_uv = acos(dot_uv/(vec2_length(u)*vec2_length(v)));
	printf("angle between u and v: %f\n", angle_uv);
	
	// NOTE: acos only gives angles between 0 and PI
	if (dot_uv != 0) {
		// dot(u, v)//||u||||v|| always returns the smallest angle between the two 
		double det_xv = vec2_det(V_I, v);
		double det_xu = vec2_det(V_I, u);
		double dot_ux = vec2_dot(u, V_I);
		double dot_vx = vec2_dot(v, V_I);

	#ifdef _V_DEBUG_
		printf("u = (%f, %f) v = (%f, %f)\n", u.x, u.y, v.x, v.y);
		printf("dot_uv/(vec2_length(u)*vec2_length(v)): %f\n", 
			dot_uv/(vec2_length(u)*vec2_length(v)));
		printf("dot_ux/(vec2_length(u)*vec2_length(V_I): %f\n", 
			dot_ux/(vec2_length(u)*vec2_length(V_I)));
		printf("dot_vx/(vec2_length(v)*vec2_length(V_I): %f\n", 
			dot_vx/(vec2_length(v)*vec2_length(V_I)));
	#endif

		double angle_from_x = 0;
		if (det_xu >= 0) {
			// u is to the right hand of x
			double det_uv = vec2_det(u, v);
			if (det_uv > 0) 
				// v is to the right hand of u
				angle_from_x = acos(dot_ux/(vec2_length(u)*vec2_length(V_I))); 
			else if (det_uv < 0 && det_xv >= 0) 
				// v is to the left hand of u but to the right hand of x
				angle_from_x = acos(dot_vx/(vec2_length(v)*vec2_length(V_I))); 
			else if (det_uv < 0 && det_xv < 0) 
				// v is to the left hand of u and x
				angle_from_x = acos(dot_vx/(vec2_length(v)*vec2_length(V_I)))*-1; 
			else { 
				// u and v are parallel 
				if (dot_uv < 0) 
					// angle between u and v is 180
					angle_from_x = acos(dot_ux/(vec2_length(u)*vec2_length(V_I))); 
				else return; // angle between u and v is 0
			}
			printf("POSITIVE: angle1: %f. angle2: %f\n", 
				angle_from_x, angle_from_x + angle_uv);
			cairo_arc(cr, R.x, R.y, (ctx->width+ctx->height)*0.03, angle_from_x, 
				angle_from_x + angle_uv);
			cairo_stroke(cr);
		} else if (det_xu < 0) {
			// u is to the left hand of x
			double det_vu = vec2_det(v, u);
			if (det_vu < 0 && det_xv < 0) 
				// u is to the left hand of v and v is to the left hand of x
				angle_from_x = acos(dot_vx/(vec2_length(v)*vec2_length(V_I)))*-1; 
			else if (det_vu < 0 && det_xv >= 0) 
				// u is to the left hand of v and v is to the right hand of x
				angle_from_x = acos(dot_vx/(vec2_length(v)*vec2_length(V_I))); 
			else if (det_vu > 0 && det_xu < 0) 
				angle_from_x = acos(dot_ux/(vec2_length(u)*vec2_length(V_I)))*-1; 
				// u is to the right hand of v and left hand of x
			else if (det_vu > 0 && det_xu >= 0) 
				// u is to the right hand of v and right hand of x
				angle_from_x = acos(dot_ux/(vec2_length(u)*vec2_length(V_I))); 
			else {
				// u and v are parallel
				if (dot_uv < 0) 
					// angle between u and v is 180
					angle_from_x = acos(dot_ux/(vec2_length(u)*vec2_length(V_I)))*-1; 
				else return; // angle between u and v is 0
			}
			printf("NEGATIVE: angle1: %f. angle2: %f\n",
				angle_from_x, angle_from_x - angle_uv);
			cairo_arc_negative(cr, R.x, R.y, (ctx->width+ctx->height)*0.03, 
				angle_from_x, angle_from_x - angle_uv);
			cairo_stroke(cr);
		}
	} else {
		// u and v intersect at a right angle  	
		size_t angle_vcount = 5;
		vec2 angle_vertices[] = {
			R, {P.x*0.1, P.y*0.1}, {(P.x+Q.x)*0.1, (P.y+Q.y)*0.1}, {Q.x*0.1, Q.y*0.1}, R
		};
		VDrawSettings settings = {
			.lw = 0.05,
			.color = {0.1, 0.1, 0.1, 1.0},
			.fill = false
		};
		Polygon angle_poly = {
			.vertices = angle_vertices,
			.vertex_count = angle_vcount,
			.settings = settings
		};
		vdraw_polygon(ctx, &angle_poly);
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
	const size_t vertex_count = 4;
	vec2 vertices[] = {
		{0.0, 0.0},
		{4.0, 4.0},
		{4.0, -2.0},
		{0.0, 0.0}
	};
	VDrawSettings settings = {
		.lw = 0.05,
		.color = {0.1, 0.1, 0.1, 1.0},
		.fill = false
	};
	Polygon poly = {
		.vertices = vertices,
		.vertex_count = vertex_count,
		.settings = settings
	};
	VDrawContext ctx = {
		.filename = "test.pdf",
		.width = 10.0,
		.height = 10.0
	};
	vdraw_create(&ctx);
	/*
	size_t edge_count;
	vdraw_calculate_edges_as_vectors(&poly, &edge_count);
	*/
	vdraw_polygon_angle(&ctx, &poly, 1);
	vdraw_polygon_angle(&ctx, &poly, 0);
	vdraw_polygon_angle(&ctx, &poly, 2);
	vdraw_polygon(&ctx, &poly);	
	
	vdraw_save(&ctx);
	vdraw_destroy(&ctx);
	
	return 0;
}

/*
 * TODO: keep track of the vertices that extend the farthest past the origin 
 * so we can crop the final image with that information
 */
