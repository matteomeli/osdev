#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/wait.h>
#include <errno.h>

int main(int argc, char *argv[]) {
    int rc = fork();
    if (rc < 0) {
        fprintf(stderr, "fork failed\n");
        exit(1);
    } else if (rc == 0) {
        char *program_name = strdup("ls");
        char *myargs[2];
        myargs[0] = program_name;
        myargs[1] = NULL;

        // execl(myargs[0], myargs[0], myargs[1]);

        // char *env[] = {NULL};
        // execle(myargs[0], myargs[0], myargs[1], env);

        // execlp(myargs[0], myargs[0], myargs[1]);

        // execv(myargs[0], myargs);

        // execvp(myargs[0], myargs);

        execvP(myargs[0], "/bin", myargs);

        /* only get here on an exec error */
        if (errno == ENOENT)
            printf("%s not found in current directory\n", program_name);
        else if (errno == ENOMEM)
            printf("not enough memory to execute child\n");
        else
            printf("error %d trying to execute child\n", errno);
    } else {
        int wc = wait(NULL);
    }
    return 0;
}
