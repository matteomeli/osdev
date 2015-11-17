#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>

int main(int argc, char *argv[]) {
    int x = 100;  // each child process receives its own copy of x
    int rc = fork();
    if (rc < 0) {
        // fork failed; exit
        fprintf(stderr, "fork failed\n");
        exit(1);
    } else if (rc == 0) {
        x++;
        printf("hello, I am child (pid:%d) x=%d\n", (int)getpid(), x);
        sleep(1);
    } else {
        int wc = wait(NULL);

        x++;
        printf("hello, I am parent of %d (wc:%d) (pid:%d) x=%d\n", rc, wc, (int)getpid(), x);
    }
    return 0;
}
