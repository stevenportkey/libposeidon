#ifndef _POSEIDON_H_
#define _POSEIDON_H_

/**
 * Creates a hash of multiple Fr elements.
 * @param inputs Array of Fr elements.
 * @param buf The buffer to store the output value.
 * @param max_len The maximum length of the buffer.
 * @return A positive value if the hash is created successfully and negative values if there are errors. The positive value is the size of the output.
 */
int poseidon_hash(const char* input, char* buf, int max_len);

#endif