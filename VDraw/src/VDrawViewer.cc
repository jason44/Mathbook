/*
clang VDrawViewer.cc -lpoppler -Ipoppler/cpp/ -Wall 
*/
#include <iostream>

#include <poppler/cpp/poppler-document.h>
#include <poppler/cpp/poppler-destination.h>
#include <poppler/cpp/poppler-page.h>
#include <poppler/cpp/poppler-page-renderer.h>
#include <poppler/cpp/poppler-rectangle.h>

/*
#include <poppler-document.h>
#include <poppler-destination.h>
#include <poppler-page.h>
#include <poppler-page-renderer.h>
#include <poppler-rectangle.h>
*/

class VDrawViewer {
public:
	VDrawViewer::VDrawViewer(const std::string &filename);
	VDrawViewer::~VDrawViewer(); 
private:
	poppler::document *doc;
	std::string filename;
};


VDrawViewer::VDrawViewer(const std::string &filename)
: filename(filename)
{
	doc = poppler::document::load_from_file(filename);
	if (!doc) exit(EXIT_FAILURE);

	

}

VDrawViewer::~VDrawViewer() 
{

}


int main(int argc, char **argv)
{
	std::cout << argv[1] << '\n';

	if (argv[1]) const std::string filename(argv[1]);
	else puts("no pdf document specified\n pass --help for additioanl options");

	return 0;
}
