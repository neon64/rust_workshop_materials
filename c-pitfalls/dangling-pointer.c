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

    // In the real world, this call to `free` could be
    //  hidden away in some other function.
    free(data);

    printf("should print zero: %d\n", *dangling);

    return 0;
}