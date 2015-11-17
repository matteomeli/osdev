#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <assert.h>
#include <fcntl.h>
#include <sys/wait.h>

int main(int argc, char *argv[]) {
    int fd = open("./e2.output", O_WRONLY | O_CREAT | O_TRUNC, S_IRWXU);
    assert(fd > -1);

    int rc = fork();
    if (rc < 0) {
        fprintf(stderr, "fork failed\n");
        exit(1);
    } else if (rc == 0) {
        int rc = write(fd, "hello, I am child\n", 18);
        sleep(1);
    } else {
        int wc = wait(NULL);
        int rc = write(fd, "hello, I am parent\n", 19);
        close(fd);
    }
    return 0;
}
