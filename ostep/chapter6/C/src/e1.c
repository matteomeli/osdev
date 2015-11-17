#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <sys/time.h>
#include <assert.h>

/* Calculate nanoseconds in a timeval structure */
long nanosec(struct timeval t) { return ((t.tv_sec * 1000000 + t.tv_usec) * 1000); }

// Calculate the cost of a system call
// NB: gettimeofday() function has microsecond resolution
int main(int argc, char const *argv[]) {
    const long iterations = 1000000;
    double system_call_cost;
    struct timeval t1, t2;
    int res;

    /*
    // Calculate cost for read()
    res = gettimeofday(&t1, NULL);
    assert(res == 0);

    // 0-byte read call
    res = read(0, NULL, 0);
    assert(res >= 0);

    res = gettimeofday(&t2, NULL);
    assert(res == 0);

    system_call_cost = (double)nanosec(t2) - (double)nanosec(t1);

    printf("Total time for a 0-byte read() call: %f ns = %f μs = %f ms\n", system_call_cost,
           system_call_cost / 1000, system_call_cost / (1000 * 1000));

    */

    // Calculate average now
    res = gettimeofday(&t1, NULL);
    assert(res == 0);

    for (int i = 0; i < 1000000; i++) {
        res = read(0, NULL, 0);
        assert(res >= 0);
    }

    res = gettimeofday(&t2, NULL);
    assert(res == 0);

    system_call_cost = (nanosec(t2) - nanosec(t1)) / (iterations * 1.0);

    printf("Avg time for a 0-byte read() call over %ld iterations: %f ns = %f μs = %f ms\n",
           iterations, system_call_cost, system_call_cost / 1000,
           system_call_cost / (1000 * 1000));

    /*
    // Calculate cost for getpid()
    res = gettimeofday(&t1, NULL);
    assert(res == 0);

    res = getpid();

    res = gettimeofday(&t2, NULL);
    assert(res == 0);

    system_call_cost = (double)nanosec(t2) - (double)nanosec(t1);

    printf("Total time for a getpid() call: %f ns = %f μs = %f ms\n", system_call_cost,
           system_call_cost / 1000, system_call_cost / (1000 * 1000));
    */

    // Calculate average now for getpid()
    res = gettimeofday(&t1, NULL);
    assert(res == 0);

    for (int i = 0; i < 1000000; i++) {
        res = getpid();
    }

    res = gettimeofday(&t2, NULL);
    assert(res == 0);

    system_call_cost = (nanosec(t2) - nanosec(t1)) / (iterations * 1.0);

    printf("Avg time for a getpid() call over %ld iterations: %f ns = %f μs = %f ms\n", iterations,
           system_call_cost, system_call_cost / 1000, system_call_cost / (1000 * 1000));
}

