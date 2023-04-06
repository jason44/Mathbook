#include <SDL.h>
#include <GL/glew.h>
#include <functional>

#include "Applet.h"
#include "Utils.h"

#define WIDTH 1024
#define HEIGHT 640

int main(int argc, char **argv) {
	Applet app(nullptr, nullptr);
	SDL_Window *window = NULL;
	SDL_GLContext context = NULL;
	if (SDL_Init(SDL_INIT_VIDEO) < 0) {
		fprintf(stderr, "SDL could not initialize! SDL_Error: %s\n", SDL_GetError());
		return 10;
	}

	atexit(SDL_Quit);
	// Request OpenGL ES 3.0
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
	// enable double-buffering
	SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
	// Create the window
	window = SDL_CreateWindow("GL TUTORIAL", SDL_WINDOWPOS_UNDEFINED,
	SDL_WINDOWPOS_UNDEFINED, WIDTH, HEIGHT,
	SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN);

	if (!window) {
		SDL_ShowSimpleMessageBox(SDL_MESSAGEBOX_ERROR, "Error",
		"Couldn't create the main window.", NULL);
		return EXIT_FAILURE;
	}

	context = SDL_GL_CreateContext(window);
	if (!context) {
		SDL_ShowSimpleMessageBox(SDL_MESSAGEBOX_ERROR, "Error",
		"Couldn't create an OpenGL context.", NULL);
		return EXIT_FAILURE;
	}

	// tell GL to only draw onto a pixel if the shape is closer to the viewer
	glEnable(GL_DEPTH_TEST); // enable depth-testing
	glDepthFunc(GL_LESS); // depth-testing interprets a smaller value as "closer"

	// flat array of vec3
	float vertices[] = {
		0.0f,  0.5f,  0.0f,
		0.5f, -0.5f,  0.0f,
		-0.5f, -0.5f,  0.0f
	};

	GLuint vbo = 0;
	glGenBuffers(1, &vbo);
	glBindBuffer(GL_ARRAY_BUFFER, vbo);
	glBufferData(GL_ARRAY_BUFFER, 9* sizeof(float), vertices, GL_STATIC_DRAW);

	GLuint vao = 0;
	glGenVertexArrays(1, &vao);
	glBindVertexArray(vao);
	glEnableVertexAttribArray(0);
	glBindBuffer(GL_ARRAY_BUFFER, vbo);
	glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, NULL);
	GLuint vs = app.createShader("test1.vert", GL_VERTEX_SHADER);
	GLuint fs = app.createShader("test1.frag", GL_FRAGMENT_SHADER);
	
	GLuint shaderProgram = glCreateProgram();
	glAttachShader(shaderProgram, vs);
	glAttachShader(shaderProgram, fs);
	glLinkProgram(shaderProgram);
	app.destroyShader(vs);
	app.destroyShader(fs);

#ifndef NDEBUG
	GLint linkingSucceeded = GL_FALSE;
	glGetProgramiv(shaderProgram, GL_LINK_STATUS, &linkingSucceeded);
	if (!linkingSucceeded) {
		SDL_Log("Linking shader failed (vert.: %s, frag.: %s\n", vs, fs);
		GLint loglen = 0;
		glGetProgramiv(shaderProgram, GL_INFO_LOG_LENGTH, &loglen);
		GLchar *errLog = (GLchar*)malloc(loglen);
		if (errLog) {
			glGetProgramInfoLog(shaderProgram, loglen, &loglen, errLog);
			SDL_Log("%s\n", errLog);
			free(errLog);
		} else {
			SDL_Log("Couldn't get shader link log; out of memory\n");
			glDeleteProgram(shaderProgram);
		}
	} 
#endif
	
	SDL_GL_DeleteContext(context);
	return 0;
}

using namespace fiber;

void Applet::createVbo() {
}


GLuint Applet::createShader(const char *filename, GLenum shaderType) {
	FILE *file = fopen(filename, "r");
	if (!file) {
		SDL_Log("cannot open file: %s\n", filename);
		return 0;
	}

	size_t len = Utils::getFileLen(file);
	// +1 for \0
	GLchar *shaderSrc = (GLchar *)malloc(len + 1);
	if (!shaderSrc) {
		SDL_Log("Out of memory when reading file: %s\n", filename);
		fclose(file);
		return 0;
	}

	fread(shaderSrc, 1, len, file);

	// Create the shader
	GLuint shader = glCreateShader(shaderType);
	glShaderSource(shader, 1, (const GLchar**)&shaderSrc, NULL);
	glCompileShader(shader);

#ifndef NDEBUG
	GLint compileSucceeded = GL_FALSE;
	glGetShaderiv(shader, GL_COMPILE_STATUS, &compileSucceeded);
	if (!compileSucceeded) {
		SDL_Log("Compilation of shader %s failed:\n", filename);
		GLint loglen = 0;
		glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &loglen);
		GLchar *errLog = (GLchar *)malloc(loglen);
		if (errLog) {
			glGetShaderInfoLog(shader, loglen, &loglen, errLog);
			SDL_Log("%s\n", errLog);
			free(errLog);
		} else SDL_Log("Couldn't get shader log; out of memory\n");

		glDeleteShader(shader);
	}
#endif

	fclose(file);
	free(shaderSrc);
	return shader;
}

void Applet::destroyShader(GLuint shaderID) {
	glDeleteShader(shaderID);
}

void Applet::run() {
	SDL_EventState(SDL_DROPFILE, SDL_ENABLE);
	
}