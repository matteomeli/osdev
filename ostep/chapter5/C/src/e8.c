#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/wait.h>
#include <errno.h>

// foo comment
int main(int argc, char *argv[]) {
    int fd[2];
    int child1, child2;
    int stdout_copy = dup(STDOUT_FILENO);

    int pipe_result = pipe(fd);
    if (pipe_result < 0) {
        fprintf(stderr, "pipe failed\n");
        exit(1);
    }

    if ((child1 = fork()) < 0) {
        fprintf(stderr, "fork failed\n");
        exit(1);
    } else if (child1 == 0) {
        // child 1: grep foo e8.c
        printf("hello 1 (child:%d)\n", (int)getpid());

        close(fd[0]);
        dup2(fd[1], STDOUT_FILENO);

        char *myargs[4];
        myargs[0] = strdup("grep");
        myargs[1] = strdup("foo");
        myargs[2] = strdup("e8.c");
        myargs[3] = NULL;

        execvp(myargs[0], myargs);
        printf("child 1 exec error\n");
    } else {
        // parent
        if ((child2 = fork()) < 0) {
            fprintf(stderr, "fork failed\n");
            exit(1);
        } else if (child2 == 0) {
            // child 2: wc -l
            printf("hello 2 (child:%d)\n", (int)getpid());

            close(fd[1]);
            dup2(fd[0], STDIN_FILENO);

            char *myargs[3];
            myargs[0] = strdup("wc");
            myargs[1] = strdup("-l");
            myargs[2] = NULL;

            execvp(myargs[0], myargs);
            printf("child 2 exec error\n");
        } else {
            // parent
            int wc = wait(NULL);
            printf("goodbye (pid:%d) (child1:%d) (child2:%d)\n", (int)getpid(), child1, child2);
        }
    }
    return 0;
}
