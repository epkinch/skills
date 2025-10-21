// Write a program that creates a zombie, and then call system() from stdlib.h
// (for executing a command from within a program) to execute the ps(1) command
// to verify that the process is a zombie.

#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>

int main() {
    int pid;

    if ((pid = fork()) < 0) {
        perror("fork error");
        exit(0);
    }
    else if (pid == 0) {
        int pid2;
        if ((pid2 = fork()) < 0) {
            perror("fork error");
            exit(0);
        }
        else if (pid == 0) {
            sleep(10000);
        }
        exit(0);
    }
    system("ps");

}