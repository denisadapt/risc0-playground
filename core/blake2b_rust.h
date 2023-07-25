#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations for Rust Hasher type
typedef struct blake2b_simd_hasher blake2b_simd_Hasher;

// Function to create and return a pointer to the Rust Hasher object
void* blake_hash_calculator_create();

// Function to update the hash state with data
void blake_hash_update(void* hash_state, const unsigned char* data, size_t length);

// Function to obtain the final hash digest
void blake_hash_digest(void* hash_state, unsigned char* into);

// Function to destroy the Rust Hasher object
void blake_hash_calculator_destroy(void* hash_state);

#ifdef __cplusplus
}
#endif
