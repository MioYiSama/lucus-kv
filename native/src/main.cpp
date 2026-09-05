#include <liburing.h>

#define LUCUS_API extern "C"

LUCUS_API int add(int x, int y) { return x + y; }

void f() {}
