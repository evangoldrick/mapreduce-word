#include <sys/socket.h>
#include <netinet/in.h>
#include <stdio.h>
#include <unistd.h>
#include <cstdlib>
#include <errno.h>
#define PORT 8080
void listenForRequests();