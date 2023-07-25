#include "blake2b_calculator.h"

extern "C" {
    void calculate_hash( const unsigned char* data, unsigned length, unsigned char* into )
    {
        BlakeHashCalculator hashCalculator;
        hashCalculator.update( data, length );
        hashCalculator.digest( into );
    }
};