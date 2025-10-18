#include <netdb.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>


#define BACKLOG 10

int main() {
    struct sockaddr_storage their_addr; 
    socklen_t addr_size;

    struct addrinfo hints, *res;
    int sockfd, client_fd;
    memset(&hints, 0, sizeof hints);
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_flags = AI_PASSIVE;

    getaddrinfo("127.0.0.1", "2025", &hints, &res);
    sockfd = socket(res->ai_family, res->ai_socktype, 0);
    bind(sockfd, res->ai_addr, res->ai_addrlen);
    freeaddrinfo(res);
    listen(sockfd, BACKLOG);
    char* msg = "Hello World!";
    while(1) {
        addr_size = sizeof their_addr;
        client_fd = accept(sockfd, (struct sockaddr *)&their_addr, &addr_size);

        int pid;
        if ((pid = fork()) < 0) {
            perror("fork error");
        }
        else if (pid == 0) {
            send(client_fd, msg, strlen(msg), 0);
            close(client_fd);
            exit(0);
        }
        close(client_fd);
    }
}