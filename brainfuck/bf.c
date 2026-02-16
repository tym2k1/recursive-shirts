#include <stdio.h>
#include <stdlib.h>

#define TAPE 30000
#define PROG 100000

int main(int argc, char **argv) {
    unsigned char tape[TAPE] = {0};
    unsigned char *p = tape;
    char prog[PROG];
    int jump[PROG], stack[PROG];
    int pc = 0, len = 0, sp = 0;
    int ch;

    FILE *f = argc > 1 ? fopen(argv[1], "r") : stdin;
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
}
