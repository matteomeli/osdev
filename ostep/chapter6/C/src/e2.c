#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <assert.h>
#include <sched.h>

#define MAX_BUF 1024

// NB: Won't work on OSX
/*
int cpu_bind(const unsigned short cpu) {
    cpu_set_t mask;
    int ret;

    CPU_ZERO(&mask);
    CPU_SET((int)cpu, &mask);
    ret = sched_setaffinity(0, sizeof mask, &mask);

    return ret;
}
*/

int main(int argc, char const *argv[]) {
    int fd1[2];
    int fd2[2];
    int child1, child2, res;

    res = pipe(fd1);
    assert(res == 0 && "pipe() failed");

    res = pipe(fd2);
    assert(res == 0 && "pipe() failed");

    child1 = fork();
    assert(child1 >= 0 && "fork() failed");

    if (child1 == 0) {
        // 1st child
        // cpu_bind(0);

        char buf[MAX_BUF];

        close(fd1[0]);
        close(fd2[1]);

        while (1) {
            write(fd1[1], "ping", sizeof("ping"));
            read(fd2[0], buf, MAX_BUF);
            printf("%d: %s\n", (int)getpid(), buf);

            sleep(1);
        }
    } else {
        // parent
        child2 = fork();
        assert(child2 >= 0 && "fork() failed");

        if (child2 == 0) {
            // 2nd child
            // cpu_bind(0);

            char buf[MAX_BUF];

            close(fd1[1]);
            close(fd2[0]);

            while (1) {
                read(fd1[0], buf, MAX_BUF);
                printf("%d: %s\n", (int)getpid(), buf);
                write(fd2[1], "pong", sizeof("pong"));
            }
        } else {
            // parent
            res = wait(NULL);
        }
    }

    return 0;
}
