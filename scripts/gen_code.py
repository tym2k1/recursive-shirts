#!/usr/bin/env python

from string_to_brainfuck import autotune
import argparse
import sys


def parse_args():
    parser = argparse.ArgumentParser(
        description="Generate code for brainfuck self-vectorizing shirts"
    )

    parser.add_argument("--font", default="monospace",
                        help="Font family (default: monospace)")
    parser.add_argument("--font-size", type=float, default=3,
                        help="Font size (default: 3)")
    parser.add_argument("--line-height", type=float, default=3,
                        help="Y increment per line (default: 3)")
    parser.add_argument("--initial-y", type=float, default=3,
                        help="Initial Y position (default: 3)")

    return parser.parse_args()


def svg_header(args):
    return (
        f'<svg font-family="{args.font}" '
        f'font-size="{args.font_size}" '
        f'style="white-space:pre;line-height:{args.line_height}">'
    )


def main():
    args = parse_args()

    final_code = ""

    # SVG Header
    print_code = svg_header(args)
    print_code += '<text y="'
    print_code += str(args.initial_y)
    print_code += '">'
    print_code += '<![CDATA['

    char_count_header = len(print_code)

    print_code += ']]></text></svg>'
    print_code, _, _, print_cells_state, print_ptr = autotune(print_code)

    printing_logic = ",[.,]"

    move_right = len(print_cells_state) - print_ptr
    printing_pointer_alignment = ">" * move_right + printing_logic + "<" * move_right if move_right > 0 else printing_logic

    # Insert printing_pointer_alignment after char_count_header dots
    dot_count = 0
    inserted = False
    new_print_code = []


    for c in print_code:
        new_print_code.append(c)

        if c == '.':
            dot_count += 1
            if dot_count == char_count_header and not inserted:
                new_print_code.append(printing_pointer_alignment)
                inserted = True

    final_code = ''.join(new_print_code)
    print(final_code)

if __name__ == "__main__":
    main()
