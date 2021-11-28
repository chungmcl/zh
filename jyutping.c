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
        int input_byte_count = strlen(argv[1]); // strlen returns ASCII strlen, but we may have unicode
        unsigned char* byte = (unsigned char*)argv[1];
        while (*byte != '\0') {
            // Thanks to @paul.ago on StackOverflow 
            // Binary    Hex          Comments
            // 0xxxxxxx  0x00..0x7F   Only byte of a 1-byte character encoding
            // 10xxxxxx  0x80..0xBF   Continuation byte: one of 1-3 bytes following the first
            // 110xxxxx  0xC0..0xDF   First byte of a 2-byte character encoding
            // 1110xxxx  0xE0..0xEF   First byte of a 3-byte character encoding
            // 11110xxx  0xF0..0xF7   First byte of a 4-byte character encoding
            int token_length = 1;
            if (*byte >= 0xC0 && *byte <= 0xDF) {
                token_length = 2;
            }
            else if (*byte >= 0xE0 && *byte <= 0xEF) {
                token_length = 3;
            }
            else if (*byte >= 0xF0) {
                token_length = 4;
            }

            char buff[5]; 
            memcpy(buff, byte, token_length); 
            buff[token_length] = '\0';
            int pos = find_entry(0, PRONUNCIATIONS_LENGTH, buff);
            if (token_length >= 3 && token_length < 4 && pos >= 0) printf("%s\t\t%s\n", buff, PRONUNCIATIONS[pos][1]);
            else printf("%s\n", buff);

            byte += token_length;
        }

        #if __APPLE__ // If on macOS, call say command to speak
        // Equivalent expression: strlen("say -v sin-ji ") + strlen("\"") + strlen(argv[1]) + strlen("\"");
        char speech_command[strlen("say -v sin-ji ") + strlen(argv[1]) + 4];
        strcpy(speech_command, "say -r 87 -v sin-ji \"");
        strcat(speech_command, argv[1]);
        strcat(speech_command, "\"");
        system(speech_command);
        #endif
    }
}

