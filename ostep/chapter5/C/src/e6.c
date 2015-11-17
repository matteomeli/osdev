#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>
#include <errno.h>

int main(int argc, char *argv[]) {
    const int MAX_WAIT_IN_SECONDS = 15;
    int status = 0;

    int rc = fork();
    if (rc < 0) {
        fprintf(stderr, "fork failed\n");
        exit(1);
    } else if (rc == 0) {
        sleep(5);
        printf("hello (pid:%d)\n", (int)getpid());
    } else {
        // int wc = waitpid(-1, &status, /*WNOHANG |*/ WUNTRACED | WCONTINUED);
        // printf("goodbye (pid:%d) (child:%d)\n", (int)getpid(), rc);

        int times = 0;
        while (times++ < MAX_WAIT_IN_SECONDS) {
            int wc = waitpid(-1, &status, WNOHANG | WUNTRACED | WCONTINUED);
            if (wc == -1) {    // waitpid error
                fprintf(stderr, "waitpid error\n");
                exit(1);
            } else if (wc == 0) {    // child is still running
                // wait for MAX_WAIT_IN_SECONDS seconds
                printf("Parent waiting for child (pid:%d)\n", rc);
                sleep(1);
            } else if (wc == rc) {
                printf("goodbye (pid:%d) (child:%d)\n", (int)getpid(), rc);

                if (WIFEXITED(status))
                    printf("child ended normally\n");
                else if (WIFSIGNALED(status))
                    printf("child ended because of an uncaught signal\n");
                else if (WIFSTOPPED(status))
                    printf("child process has stopped\n");

                exit(0);
            }
        }
    }

    // shoudl not get here
    return 1;
}
