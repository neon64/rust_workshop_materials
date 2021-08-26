#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>
#include <unistd.h>

void *counter(void *data) {
    usleep(1);
    int *x = data;
    for(int i = 0; i < 10000; i++) {
        *x += 1;
        //
    }
    
    return NULL;
}

// This program should compile with no warnings, no errors
// Yet is very incorrect.
int main() {
    int x = 0;
    pthread_t t;
    pthread_create(&t, NULL, &counter, &x);

    counter(&x);

    pthread_join(t, NULL);

    // at least on my computer, this gives either 10,000,
    // 20,000, or some number in between the two
    // classic data race
    printf("%d", x);
}