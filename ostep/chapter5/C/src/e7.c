#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/wait.h>

int main(int argc, char *argv[]) {
    int rc = fork();
    if (rc < 0) {
        fprintf(stderr, "fork failed\n");
        exit(1);
    } else if (rc == 0) {
        close(STDOUT_FILENO);
        printf("hello\n");    // won't print anything
        sleep(5);
    } else {
        int wc = wait(NULL);
        printf("goodbye\n");    // will print it
    }
    return 0;
}
