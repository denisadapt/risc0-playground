#include <cstdlib>

void _add(int a, int b, int* c)
{
    int d = a + b;
   *c = d;
}

extern "C" {
    void addxx(int a, int b, int* c) {
        _add(a, b, c);
    }

    void subxxx(int a, int b, int* c) {
        _add(a, -b, c);
    }
}

