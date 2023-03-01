#include <complex.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <stdio.h>
#include <math.h>
#include <cairo/cairo.h>
#include <cairo/cairo-pdf.h>

// -lgsl -lgslcblas
// clang main.c -lcairo -lm 
/*
gcc main.c -lcairo -lm -Wall -O2
*/

#define IMAGE_COORDINATTES

typedef struct {
	float r, g, b, a;
} RGBA_t;

#define V_I (vec2){1.0, 0.0}
#define V_J (vec2){0.0, 1.0}

typedef float* RGBA;

struct VDrawSettings {
	cairo_fill_rule_t fill;
	cairo_line_cap_t linecap;
	cairo_line_join_t linejoin;
};

typedef struct VDrawContext {
	cairo_surface_t *surface;
	cairo_t *cr;
	const char *filename;
	const double height;
	const double width;
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
	double lw;
	RGBA_t color;	
	bool dash;
	bool fill;
} Polygon;


inline double vec2_dot(const vec2 u, const vec2 v) 
{
	return (u.x * v.x) + (u.y * v.y);
}

#ifdef IMAGE_COORDINATTES
// TODO: create a coordinate system centered around the origin of the image
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

void vdraw_create(VDrawContext *ctx)
{
	/*
	cairo_surface_t *surface;
	cairo_t *cr;
	const char *filename;
	const double height;
	const double width;
	*/
	assert(ctx->filename);
	assert(ctx->width);
	assert(ctx->height);
	ctx->surface = cairo_pdf_surface_create(ctx->filename, ctx->width, ctx->height);
	ctx->cr = cairo_create(ctx->surface);
	cairo_set_source_rgba(ctx->cr, 1.0, 1.0, 1.0, 1.0);
	//cairo_set_line_cap(ctx->cr, CAIRO_LINE_CAP_ROUND);
	cairo_set_line_cap(ctx->cr, CAIRO_LINE_CAP_ROUND);
	cairo_set_line_join(ctx->cr, CAIRO_LINE_JOIN_ROUND);
	cairo_set_fill_rule(ctx->cr, CAIRO_FILL_RULE_WINDING);
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

#define VDRAW_STYLE_POLYGON CAIRO_FILL_RULE_WINDING,CAIRO_LINE_CAP_ROUND,CAIRO_LINE_JOIN_ROUND
void vdraw_set_style(VDrawContext *ctx, const cairo_fill_rule_t fill, 
	const cairo_line_cap_t linecap, const cairo_line_join_t linejoin)
{
	cairo_set_fill_rule(ctx->cr, fill);
	cairo_set_line_cap(ctx->cr, linecap);
	cairo_set_line_join(ctx->cr, linejoin);
}

void vdraw_polygon(VDrawContext *ctx, Polygon *poly)
{
	if (!poly->vertices || !poly->vertex_count) goto ERROR_EXIT;

	if (poly->vertices[0].x != poly->vertices[poly->vertex_count-1].x || 
		poly->vertices[0].y != poly->vertices[poly->vertex_count-1].y) {
		puts("Polygon shape is not closed");
		goto ERROR_EXIT;
	}

	cairo_t *cr = ctx->cr;
	RGBA c;
	if (poly->color.r) c = &poly->color;
	else c = (RGBA)(&(RGBA_t){0.0, 0.0, 0.0, 1.0});
	cairo_set_source_rgba(cr, c[0], c[1], c[2], c[3]);

	double lw;
	if (poly->lw) lw = poly->lw;
	else lw = 0.15;
	cairo_set_line_width(cr, lw);

	if (poly->dash) {
		const double dashes[] = {10.0, 10.0};
		cairo_set_dash(cr, dashes, 2, -10.0);
	}

	vec2_to_image_coordinates(ctx->width, ctx->height, poly->vertices, poly->vertex_count);

	/*
	// close the polygon unless it is already closed
	vec2 buf[vertex_count+1];
	if (vertices[0].x != vertices[vertex_count-1].x &&
		vertices[0].y != vertices[vertex_count-1].y) {
		// TODO: Check if realloc is faster than using memcpy 
		printf("vertex_count: %lu\n",vertex_count);
		memcpy(buf, vertices, sizeof(vec2)*(vertex_count));
		buf[vertex_count] = (vec2){vertices[0].x, vertices[0].y};
		vertices = buf;
		++vertex_count;
	}
	*/
	vec2 *vertices = poly->vertices;
	size_t vertex_count = poly->vertex_count;

	for (size_t k = 0; k < vertex_count; k++) printf("(%f, %f)\n", vertices[k].x, vertices[k].y);	
	puts("--------------");

	cairo_move_to(cr, vertices[0].x, vertices[0].y);
	for (size_t i = 1; i < vertex_count; i++) {
		cairo_line_to(cr, vertices[i].x, vertices[i].y);			
	}

	// preserve the path so cairo knows what to fill
	if (poly->fill) {
		cairo_stroke_preserve(cr);
		cairo_set_source_rgba(cr, c[0], c[1], c[2], 0.2);
		cairo_fill(cr);
	} else cairo_stroke(cr);

	return;

	ERROR_EXIT: {
		puts("EXIT_ERROR");
		vdraw_destroy(ctx);
		exit(EXIT_FAILURE);
	}
}

// specify vertex by index
void vdraw_polygon_angle(VDrawContext *ctx, Polygon *poly, const int vertex) {
	if (!poly->vertices || !poly->vertex_count) goto ERROR_EXIT;

	if (poly->vertices[0].x != poly->vertices[poly->vertex_count-1].x || 
		poly->vertices[0].y != poly->vertices[poly->vertex_count-1].y) {
		puts("Polygon shape is not closed");
		goto ERROR_EXIT;
	}

	vdraw_set_style(ctx, VDRAW_STYLE_POLYGON);

	// modify the color values
	cairo_t *cr = ctx->cr;
	RGBA c;
	if (poly->color.r) c = &poly->color;
	else c = (RGBA)(&(RGBA_t){0.0, 0.0, 0.0, 1.0});
	cairo_set_source_rgba(cr, c[0], c[1], c[2], c[3]);

	double lw;
	if (poly->lw) lw = poly->lw * 0.30;
	else lw = 0.09;
	cairo_set_line_width(cr, lw);

	vec2 R = poly->vertices[vertex];
	// signed integers are very nice nice nice
	vec2 P = poly->vertices[(vertex - 1) % (int)(poly->vertex_count)];
	vec2 Q = poly->vertices[(vertex + 1) % (int)(poly->vertex_count-1)];
	vec2 u = {P.x-R.x, P.y-R.y};
	vec2 v = {Q.x-R.x, Q.y-R.y};

	double dot_uv = vec2_dot(u, v);
	double angle_uv = acos(dot_uv / ((sqrt(pow(u.x, 2) + pow(u.y, 2))) * (sqrt(pow(v.x, 2) + pow(v.y, 2)))));
	printf("angle between u and x: %f\n", angle_uv);
	if (dot_uv != 0) {
		// angle between u and v in the right hand coordinate 
		// system is less than 180
		double dot_ux = vec2_dot(u, V_I);
		double dot_vx = vec2_dot(v, V_I);

		if (dot_ux < dot_vx) {
			double proj_ux_m = vec2_dot(u, V_I);
			vec2 proj_ux = {proj_ux_m*V_I.x, proj_ux_m*V_I.y};
			double angle_ux = acos(dot_ux / ((sqrt(pow(u.x, 2) + pow(u.y, 2))) * (sqrt(pow(proj_ux.x, 2) + pow(proj_ux.y, 2)))));
			printf("angle between u and x: %f\n", angle_ux);
			cairo_arc(cr, R.x, R.y, (ctx->width+ctx->height)*0.03, M_PI_2 + angle_ux, M_PI_2 + angle_ux+angle_uv);
		} else {
			double proj_vx_m = vec2_dot(v, V_I);
			vec2 proj_vx = {proj_vx_m*V_I.x, proj_vx_m*V_I.y};
			double angle_vx = acos(dot_vx / ((sqrt(pow(v.x, 2) + pow(v.y, 2))) * (sqrt(pow(proj_vx.x, 2) + pow(proj_vx.y, 2)))));
			printf("angle between u and x: %f\n", angle_vx);
			cairo_arc(cr, R.x, R.y, (ctx->width+ctx->height)*0.03, M_PI_2 + angle_vx, M_PI_2 + angle_vx-angle_uv);
		}
	} else {
		// perpendicular (right angle)
		puts("I haven't defined this yet sir");
	}  


	cairo_stroke(cr);
	return;

	ERROR_EXIT: {
		puts("OUTPUT_ERROR");
		vdraw_destroy(ctx);
		exit(EXIT_FAILURE);
	}
}

int main(int argc, char** argv) 
{
	const size_t vertex_count = 4;
	vec2 vertices[] = {
		{0.0, 0.0},
		{4.0, 4.0},
		{4.0, -2.0},
		{0.0, 0.0}
	};

	Polygon poly = {
		.vertices = vertices,
		.vertex_count = vertex_count,
		.lw = 0.1,
		.color = {0.1, 0.1, 0.1, 1.0},
		.fill = true
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
	vdraw_polygon(&ctx, &poly);
	puts("POLYGON");
	vdraw_polygon_angle(&ctx, &poly, 1);
	vdraw_polygon_angle(&ctx, &poly, 0);
	vdraw_polygon_angle(&ctx, &poly, 2);
	puts("ANGLE");
	vdraw_save(&ctx);
	puts("SAVE");
	vdraw_destroy(&ctx);
	puts("DESTROY");

	return 0;
}

/*
 * TODO: keep track of the vertices that extend the farthest past the origin 
 * so we can crop the final image with that information
 */
