#pragma once

void _add(int a, int b, int* c);
void _sub(int a, int b, int* c);

class MyClass
{
    int m_a;
    int m_b;
public:
    MyClass(int a, int b);
    ~MyClass();
    void get_sum( int* result );
    void get_sub( int* result);
};