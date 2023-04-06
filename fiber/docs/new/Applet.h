#include <SDL.h>
#include <GL/glew.h>
#include <functional>


namespace fiber {

class GLBackend {
public:
	GLBackend();
	~GLBackend();
	void createVbo();
	GLuint createShader(const char *filename, GLenum shaderType);
	void destroyShader(GLuint shader);
private:
	int a;
};

class Applet {
public:
	using InitCallback = std::function<void(fiber::Applet applet)>;
	using CleanupCallback = std::function<void(fiber::Applet applet)>;

	Applet(InitCallback init, CleanupCallback callback);
	~Applet();


	/**
	 * generaly order of things:
	 * 1. create vertex buffer objects that store vertices in the gpu
	 * 2. create vertex array objects (vao) to store information about a set
	 *     of vbos (eg: one for vertices, one for texCoords, vertex normals, etc.
	 *     all of which make up a mesh
	 * 3. create vertex shaders which interpret where the vertices will end up on the display
	 * 4. create fragment shaders which colors the surfaces
	 * 5. create shader program which the gpu can execute (using all of the above)
	 */
	static inline GLBackend& get() { return backend; };
	void createWindow(const char *winTitle, const int height, const int width);
	void Applet::run(const GLuint shaderProgram);
	void run();

private:
	static GLBackend backend;
	SDL_Window *mWindow;
	SDL_GLContext mContext;
	bool mClosed;

};

}