
#include <GL/glew.h>
#include <SDL.h>
#include <cstdio>

namespace fiber {
namespace Utils {

static size_t getFileLen(FILE *file) {
	size_t len;
	size_t currPos = ftell(file);
	fseek(file, 0, SEEK_END);
	len = ftell(file);
	// Return the file to its previous position
	fseek(file, currPos, SEEK_SET);
	return len;
}

}