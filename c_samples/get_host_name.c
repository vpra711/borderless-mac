#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>

const int MACHINE_NAME_LENGTH = 32;

int main() {
    char *host_name = (char *) malloc(MACHINE_NAME_LENGTH - 1);
    memset(host_name, 0, MACHINE_NAME_LENGTH);
    if (gethostname(host_name, MACHINE_NAME_LENGTH - 1) == 0) {
        printf("hostname: %s", host_name);
    } else {
        printf("error getting hostname");
    }
    return 0;
}
