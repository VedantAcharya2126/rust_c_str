#ifndef CONCAT_H
#define CONCAT_H

// Concatenates "Hello, " with the input string.
char* concat_string(const char* name);

// Frees the memory allocated by concat_string.
void free_string(char* s);

#endif