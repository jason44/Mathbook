#include <stdint.h>
#include <stdlib.h>


typedef struct vvector VVector;

typedef struct vallocator VAllocator;


struct vallocator {
};

struct vvector {
	void *data;
	unsigned int size;
	unsigned int capacity;
	unsigned int type_size;
	unsigned int l;
};


// following C++ naming conventions
inline const size_t vvector_size(const struct vvector *vec)
{

}


struct vvector vvector_new(void *data, size_t data_size)
{
	
}

struct vvector vvector_cpy(void *data, size_t data_count, size_t data_size)
{
	
}

vvector_realloc(struct vvector *vec)
{

}