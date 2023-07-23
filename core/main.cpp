#include <new>
#include "mylib.h"

extern "C" {
    void addxx(void* mem, int a, int b, int* c) {
        _add(a, b, c);
    }

    void subxxx(int a, int b, int* c) {
        _sub(a, b, c);
    }

    void addzz( void* mem, int a, int b, int* result )
    {
        auto s = (MyClass*) mem;
        new(s) MyClass( a, b );
        s->get_sum( result );
    }
}

