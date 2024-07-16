# libposeidon

This library is a wrapper of the rust implementation of Poseidon hash algorithm on Bn254. It allows other language to make use of the library.


The API:
```C
/**
 * Creates a hash of multiple Fr elements.
 * @param inputs Array of Fr elements.
 * @param buf The buffer to store the output value.
 * @param max_len The maximum length of the buffer.
 * @return A positive value if the hash is created successfully and negative values if there are errors. The positive value is the size of the output.
 */
int poseidon_hash(const char* input, char* buf, int max_len);
```

For example, input `["123", "456"]` will generate a Poseidon hash value of `19620391833206800292073497099357851348339828238212863168390691880932172496143`.
