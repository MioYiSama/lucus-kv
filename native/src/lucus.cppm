module;

#include <liburing.h>

export module lucus;

extern "C" int add(int x, int y) { return x + y; }
