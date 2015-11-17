/*
* Print the gettimeofday() clock resolution for this machine.
*
* Code taken from Iozone. Iozone Filesystem Benchmark
* Author: Don Capps
*
*/
#include <sys/time.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <assert.h>

double time_res, delay;
void get_resolution();       /* Works with most compilers */
static double time_so_far(); /* Works with most compilers */

/*
* Measure and print the gettimeofday() resolution.
*/
int main() {
    printf("Measuring the gettimeofday() resolution\n");
    get_resolution();
    printf("Time resolution of gettimeofday() = %f seconds\n", time_res);
    printf("Time resolution of gettimeofday() = %f milli seconds\n", time_res * 1000);
    printf("Time resolution of gettimeofday() = %f micro seconds\n", time_res * (1000 * 1000));
}

void get_resolution() {
    double starttime, finishtime;
    long j;
again:
    finishtime = time_so_far(); /* Warm up the instruction cache */
    starttime = time_so_far();  /* Warm up the instruction cache */
    delay = j = 0;              /* Warm up the data cache */
    while (1) {
        starttime = time_so_far();
        for (j = 0; j < delay; j++)
            ;
        finishtime = time_so_far();
        if (starttime == finishtime)
            delay++;
        else
            break;
    }
    time_res = (finishtime - starttime) / 1000000.0;
}

/************************************************************************/
/* Time measurement routines. */
/* Return time in microseconds */
/************************************************************************/
static double time_so_far() {
    struct timeval tp;
    int res = gettimeofday(&tp, (struct timezone *)NULL);
    assert(res == 0);
    return ((double)(tp.tv_sec) * 1000000.0) + (((double)tp.tv_usec));
}
