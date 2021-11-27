#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#include "jyutping.h"

int find_entry(int l, int r, char* x)
{
    if (r >= l) {
        int mid = l + (r - l) / 2;
        if (strcmp(PRONUNCIATIONS[mid][0], x) == 0)
            return mid;
        if (strcmp(PRONUNCIATIONS[mid][0], x) > 0)
            return find_entry(l, mid - 1, x);
        return find_entry(mid + 1, r, x);
    }
    return -1;
}

int main(int argc, char* argv[]) {
    if (argc > 1) {
        // print out jyutping
        int input_length = strlen(argv[1]);
        for (int i = 0; i < input_length; i += 3) {
            char buff[4]; // unicode chinese chars are 3 bytes + \0 = 4 bytes
            memcpy(buff, argv[1] + i, 3);
            buff[3] = '\0';
            printf("%s\t%s\n", buff, PRONUNCIATIONS[find_entry(0, PRONUNCIATIONS_LENGTH, buff)][1]);
        }

        #if __APPLE__
        // speak
        char speech_command[strlen("say -v sin-ji ") + strlen(argv[1])];
        strcpy(speech_command, "say -r 87 -v sin-ji ");
        strcat(speech_command, argv[1]);
        system(speech_command);
        #endif
    }
}

