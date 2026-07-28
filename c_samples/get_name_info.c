#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <arpa/inet.h>
#include <stdio.h>

int main() {

    struct sockaddr_in sa4 = {0};

    sa4.sin_family = AF_INET,
    sa4.sin_len = sizeof(sa4);
    sa4.sin_port = htons(7);
    inet_aton("127.0.0.1", &sa4.sin_addr);
    char hbuf[NI_MAXHOST] = {0};
    char sbuf[NI_MAXSERV] = {0};

    int getnameinfo_status = getnameinfo(&sa4, sizeof(sa4), hbuf, NI_MAXHOST, sbuf, NI_MAXSERV, NI_NAMEREQD);

    if(getnameinfo_status == 0) {
        printf("host: %s, service: %s.", hbuf, sbuf);
    } else {
        printf("get name info failure");
    }
    return 0;
}
