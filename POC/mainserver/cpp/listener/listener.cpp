#include "listener.h"

void listenForRequests() {
    int socketDescriptor = socket(AF_INET, SOCK_STREAM, 0);
    if (socketDescriptor < 0) {
        perror("Error creating socket");
        exit(errno);
    }

    struct sockaddr_in address;
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = htonl(INADDR_ANY);
    address.sin_port = htons(PORT);
    int bindCode = bind(socketDescriptor, (struct sockaddr*) &address, sizeof(address));
    if (bindCode < 0) {
        perror("Error binding");
        exit(errno);
    }

    int listenCode = listen(socketDescriptor, 100);
    if (listenCode < 0) {
        perror("Error listening");
        exit(errno);
    }

    socklen_t size = sizeof(address);
    int acceptCode = accept(socketDescriptor, (struct sockaddr*) &address, (socklen_t*) &size);
    char buffer[2048] = {0};
    int valRead = read(socketDescriptor, buffer, 2048);

    printf("%s", buffer);
    int x = close(socketDescriptor);
}