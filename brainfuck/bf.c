#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define TAPE 30000
#define PROG 100000

int main(int argc, char **argv) {
    unsigned char tape[TAPE] = {0};
    unsigned char *p = tape;
    char prog[PROG];
    int jump[PROG], stack[PROG];
    int pc = 0, len = 0, sp = 0;
    int ch;

    int dump = 0;
    char *filename = NULL;

    /* parse args */
    for (int i = 1; i < argc; i++) {
        if (!strcmp(argv[i], "-d")) {
            if (i + 1 >= argc) {
                fprintf(stderr, "-d requires a number\n");
                return 1;
            }
            dump = atoi(argv[++i]);
            if (dump < 0 || dump > TAPE) {
                fprintf(stderr, "invalid dump size\n");
                return 1;
            }
        } else {
            filename = argv[i];
        }
    }

    FILE *f = filename ? fopen(filename, "r") : stdin;
    if (!f) return 1;

    /* read program */
    int c;
    while (len < PROG && (c = fgetc(f)) != EOF)
        if (c=='>'||c=='<'||c=='+'||c=='-'||
            c=='.'||c==','||c=='['||c==']')
            prog[len++] = c;

    if (f != stdin) fclose(f);

    /* build jump table */
    for (int i = 0; i < len; i++) {
        if (prog[i] == '[')
            stack[sp++] = i;
        else if (prog[i] == ']') {
            if (!sp) return 1;
            int j = stack[--sp];
            jump[i] = j;
            jump[j] = i;
        }
    }
    if (sp) return 1;

    /* execute */
    while (pc < len) {
        switch (prog[pc]) {
            case '>': if (++p >= tape + TAPE) return 1; break;
            case '<': if (--p < tape) return 1; break;
            case '+': ++*p; break;
            case '-': --*p; break;
            case '.': putchar(*p); break;
            case ',': ch = getchar(); *p = (ch == EOF) ? 0 : ch; break;
            case '[': if (!*p) pc = jump[pc]; break;
            case ']': if (*p) pc = jump[pc]; break;
        }
        pc++;
    }
    /* debug dump */
    if (dump > 0) {
        fprintf(stderr, "\n\n--- tape dump (%d cells) ---\n", dump);
        for (int i = 0; i < dump; i++) {
            if (i == p - tape)
                fprintf(stderr, "[%3d] ", tape[i]);  // mark pointer
            else
                fprintf(stderr, " %3d  ", tape[i]);
            if ((i + 1) % 16 == 0) fprintf(stderr, "\n");
        }
        fprintf(stderr, "\nptr = %ld\n", p - tape);
    }
}
