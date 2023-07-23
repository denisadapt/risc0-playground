#include <cstdlib>
#include "mylib.h"

extern "C" {
    void addxx(int a, int b, int* c) {
        _add(a, b, c);
    }

    void subxxx(int a, int b, int* c) {
        _sub(a, b, c);
    }
}

