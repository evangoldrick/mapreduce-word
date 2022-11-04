#include "listener.h"

void listenForRequests() {
    int socketDescriptor = socket(AF_INET, SOCK_STREAM, 0);

    if (socketDescriptor == -1) {
        throw socketDescriptor;
    }
}