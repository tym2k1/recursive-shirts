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


    if print_cells_state[print_ptr] == 0:
        printing_pointer_alignment = printing_logic
    else:
        # Find nearest zero cell
        best_dist = None

        for i, val in enumerate(print_cells_state):
            if val == 0:
                dist = i - print_ptr
                if best_dist is None or abs(dist) < abs(best_dist):
                    best_dist = dist

        # Also consider virtual zero cells outside current bounds
        # Left outside → index -1
        left_outside_dist = -1 - print_ptr
        if best_dist is None or abs(left_outside_dist) < abs(best_dist):
            best_dist = left_outside_dist

        # Right outside → index len(print_cells_state)
        right_outside_dist = len(print_cells_state) - print_ptr
        if abs(right_outside_dist) < abs(best_dist):
            best_dist = right_outside_dist

        # Generate movement
        if best_dist > 0:
            move = ">" * best_dist
        elif best_dist < 0:
            move = "<" * (-best_dist)
        else:
            move = ""

        printing_pointer_alignment = move + printing_logic + (
            "<" * best_dist if best_dist > 0 else ">" * (-best_dist)
        )

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
