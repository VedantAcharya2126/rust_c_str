#include "concat-string.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* concat_string(const char* name) {
    const char* prefix = "Hello, ";

    // size_t is capable of representing the maximum size of any object that can be allocated in memory on a given system.
    size_t len_prefix = strlen(prefix);
    size_t len_name = strlen(name);
    size_t total_len = len_prefix + len_name;

    // Allocate memory for the new string (+1 for the null terminator)
    char* result = (char*)malloc(total_len + 1);

    if (result == NULL) {
        return NULL;
    }

    strcpy(result, prefix);
    strcat(result, name);

    return result;
}

void free_string(char* s) {
    if (s) {
        free(s);
        s = NULL;  // Avoiding dangling pointers
    }
}
