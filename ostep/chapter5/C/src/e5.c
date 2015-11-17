#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>
#include <errno.h>

int main(int argc, char *argv[]) {
    int rc = fork();
    if (rc < 0) {
        fprintf(stderr, "fork failed\n");
        exit(1);
    } else if (rc == 0) {
        int wc = wait(NULL);    // A call to wait in the child waits for the parent
                                // and returns -1 and sets errno to ECHILD value
        printf("hello (child:%d) (wc:%d)\n", (int)getpid(), wc);
        if (errno == ECHILD) printf("errno=ECHILD\n");
    } else {
        // int wc = wait(NULL);
        printf("goodbye (pid:%d)\n", (int)getpid());
    }
    return 0;
}
