#include "mylib.h"

void _add(int a, int b, int* c)
{
   *c = a + b;
}

void _sub(int a, int b, int* c)
{
   *c = a - b;
}

MyClass::MyClass(int a, int b) : m_a(a), m_b(b) {}
MyClass::~MyClass() {}
void MyClass::get_sum(int* result) { *result = m_a + m_b; }
void MyClass::get_sub(int* result) { *result = m_a - m_b; }