#include "blake2b_calculator.h"
#include "blake2b_rust.h"


BlakeHashCalculator::BlakeHashCalculator() {
    hash_state_ = blake_hash_calculator_create();
}

BlakeHashCalculator::~BlakeHashCalculator() {
    blake_hash_calculator_destroy(hash_state_);
}

void BlakeHashCalculator::update(const unsigned char* data, unsigned length) {
    blake_hash_update(hash_state_, data, length);
}

void BlakeHashCalculator::digest(unsigned char* into) {
    blake_hash_digest(hash_state_, into);
}

void BlakeHashCalculator::reset() {
    blake_hash_calculator_destroy(hash_state_);
    hash_state_ = blake_hash_calculator_create();
}