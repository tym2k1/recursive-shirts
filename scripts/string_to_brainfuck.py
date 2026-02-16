#!/usr/bin/env python

def text_to_brainfuck(text):
    bf_code = ""
    current = 0
    for ch in text:
        target = ord(ch)
        diff = target - current
        if diff > 0:
            bf_code += "+" * diff
        elif diff < 0:
            bf_code += "-" * (-diff)
        # Output the current character
        bf_code += "."
        # Update the current cell value to the target ASCII
        current = target
    return bf_code

if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        # Allow the input string to be passed as command line arguments
        input_text = " ".join(sys.argv[1:])
    else:
        # Otherwise, prompt the user for input
        input_text = input("Enter text: ")
    brainfuck_code = text_to_brainfuck(input_text)
    print("Generated Brainfuck code:")
    print(brainfuck_code)
