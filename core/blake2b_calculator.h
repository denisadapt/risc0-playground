#pragma once
#include <cstdint>

class BlakeHashCalculator{
public:
    BlakeHashCalculator();
    ~BlakeHashCalculator();
    void update(const unsigned char* data, unsigned length);
    void digest(unsigned char* into);
    void reset();

private:
    // Use an opaque pointer to represent the Rust Hasher
    void* hash_state_;
};