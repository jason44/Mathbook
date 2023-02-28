#include <complex.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <cairo/cairo.h>
#include <cairo/cairo-pdf.h>

// -lgsl -lgslcblas
// -lcairo -lm

typedef struct {
	float m[4];
} RGBA_t;

typedef float* RGBA;

typedef struct VDrawContext {
	cairo_surface_t *surface;
	cairo_t *cr;
	const char *filename;
	const double height;
	const double width;
} VDrawContext;

typedef struct vec2 {
	float x;
	float y;
} vec2;

typedef struct Edge {
	vec2 p;
	vec2 q;
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


void vdraw_create(VDrawContext *ctx)
{
	/*
	cairo_surface_t *surface;
	cairo_t *cr;
	const char *filename;
	const double height;
	const double width;
	*/
	ctx->surface = cairo_pdf_surface_create(ctx->filename, ctx->width, ctx->height);
	ctx->cr = cairo_create(ctx->surface);
	cairo_set_source_rgba(ctx->cr, 0.0, 0.0, 0.0, 1.0);
	cairo_paint(ctx->cr);	
}


inline void vdraw_destroy(VDrawContext *ctx)
{
	cairo_destroy(ctx->cr);	
}

/*
*/
		
void vdraw_polygon(VDrawContext *ctx, Polygon *poly)
{
	RGBA c = &poly->color;
	cairo_t *cr = ctx->cr;

	cairo_set_source_rgba(cr, c[0], c[1], c[2], c[3]);
	cairo_set_line_width(cr, poly->lw);

	if (poly->dash) {
		const double dashes[] = {10.0, 10.0};
		cairo_set_dash(cr, dashes, 2, -10.0);
	}

	vec2 *vertices = poly->vertices;
	size_t vertex_count = poly->vertex_count;

	// close the polygon even if its already closed
	vec2 vertices[vertex_count+1];

	if (vertices[0].x == vertices[vertex_count-1].x &&
		vertices[0].y == vertices[vertex_count-1].y) {
		// TODO: Check if realloc is faster than using memcpy 
		memcpy(vertices, poly->vertices, sizeof(vec2)*(vertex_count));
		vertices[vertex_count] = {vertices[0].x, vertices[0].y};
	}
	cairo_move_to(cr, vertices[0].x, vertices[0].y);
	for (size_t i = 1; i < poly->vertex_count; i++) {
		cairo_line_to(cr, vertices[i].x, vertices[i].y);			
	}
	cairo_stroke(cr);
	if (poly->fill) cairo_fill(cr);
	
}

inline double vec2_dot(vec2 u, vec2 v) 
{
	return (u.x * v.x) + (u.y * v.y);
}

/*********** FREE THE ALLOCATED ARRAY ***********/
void polygon_calculate_edges(Polygon *poly, Edge *edges)
{
	puts("Err");	
}

/*********** FREE THE ALLOCATED ARRAY ***********/
void polygon_calculate_edges_as_vectors(Polygon *poly, vec2 *edges)
{
	puts("Err");	
}

int main(int argc, char** argv) 
{
	size_t vertex_count = 3;
	vec2 vertices[vertex_count] = {
		{0.0, 0.0},
		{1.0, 1.0},
		{0.5, 2.5}
	};

	Polygon poly = {
		.vertices = vertices,
		.vertex_count = vertex_count,
		.lw = 1.0,
		.color = {1.0, 1.0, 1.0, 1.0}
	};
	
	VDrawContext ctx;
	vdraw_create(&ctx);
	
	vdraw_polygon(&ctx, &poly);

	vdraw_destroy(&ctx);
}
