#include <stdlib.h>
#include <stdio.h>
#include <sys/types.h>
#include <unistd.h>

int main(int argc, char* argv[]) {
    int fd1[2];
    int fd2[2];
    pid_t pid1;
    pid_t pid2;
    pid_t pid3;
    char* cmd1 = "/bin/cat";
    char* cmd2 = "/usr/bin/sort";
    char* cmd3 = "/usr/bin/uniq";
    char* args1[] = {"cat", argv[1], NULL};
    char* args2[] = {"sort", NULL};
    char* args3[] = {"uniq", NULL};

    if (pipe(fd1) < 0) {
        perror("pipe1 error");
    }
    if (pipe(fd2) < 0) {
        perror("pipe2 error");
    }

    if ((pid1 = fork()) < 0) {
        perror("fork1 error");
    }
    else if (pid1 == 0) {
        close(fd1[0]);
        dup2(fd1[1], STDOUT_FILENO);
        close(fd1[1]);

        execve(cmd1, args1, NULL);
        perror("execve1 error");
        exit(0);
    }
    close(fd1[1]);

    if ((pid2 = fork()) < 0) {
        perror("fork2 error");
    }
    else if (pid2 == 0) {
        close(fd1[1]);
        dup2(fd1[0], STDIN_FILENO);
        close(fd1[0]);

        close(fd2[0]);
        dup2(fd2[1], STDOUT_FILENO);
        close(fd2[1]);

        execve(cmd2, args2, NULL);
        perror("execve2 error");
        exit(0);
    }
    close(fd1[0]);
    if ((pid3 = fork()) < 0) {
        perror("fork3 error");
    }
    else if (pid3 == 0) {
        close(fd2[1]);
        dup2(fd2[0], STDIN_FILENO);
        close(fd2[0]);

        execve(cmd3, args3, NULL);
        perror("execve3 error");
        exit(0);
    }

    close(fd2[0]);
    close(fd2[1]);
    waitpid(pid1, NULL, 0);
    waitpid(pid2, NULL, 0);
    waitpid(pid3, NULL, 0);
}