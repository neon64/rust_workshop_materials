#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int x;
    int y;
} useful_data_t;

int main() {
    useful_data_t* data = malloc(sizeof(useful_data_t));
    data->x = 3;
    data->y = 5;

    int *dangling = &data->x;

    // we're done with `data` now... right? RIGHT?
    free(data);

    // WRONG: `dangling` refers to memory owned by the allocation of `data`.
    // This probably won't even crash. Instead it will print GARBAGE, potentially corrupting rest of program.
    printf("should print zero: %d\n", *dangling);

    return 0;
}