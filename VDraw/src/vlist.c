#include <stdlib.h>
#include <stdio.h>

// it might be better to keep size for easier interfacing????

typedef struct vlist {
	struct vlist *next;
	void *data;
} vlist;

typedef struct vlist_head_t {
	struct vlist *next;
	size_t size;
};

inline struct vlist_head_t vlist_new() 
{
	return (struct vlist_head_t){
		next = NULL;
	};
}

// data must be typecasted before being accessed for all data pointers
void vlist_append(struct vlist_head_t *list, void *data)
{
	struct vlist *cur = list->next;
	for (size_t i = 0; i < list->size-1; i++) {
		cur = cur->next;
	}
	cur->next = (struct vlist *)malloc(sizeof(struct vlist));
	cur->next->data = data;
	list->size++;
}

void vlist_destroy(struct vlist_head_t *list)
{
	if (list->size > 0) {
		struct vlist *cur = vlist->next;
		for (size_t i = 0; i < list->size-2; i++) {
			struct vlist *temp = cur->next;
			free(cur);
			cur = temp;
		}
	}
}


int main()
{
	int a[] = {1, 2, 3, 4, 5};
	vlist_head_t head = vlist_new();
	for (int i = 0; i < 5; i++) {
		vlist_append(head, (void *)a+i);	
	}
	for (int i = 0; i < 5; i++) {
		vlist_head_tkk	
	}
}
