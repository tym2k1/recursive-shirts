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
    parser.add_argument("--width", type=int, default=63,
                        help="SVG width (default: 63)")
    parser.add_argument("--height", type=int, default=60,
                        help="SVG height (default: 60)")
    parser.add_argument("--y-increment", type=float, default=5,
                        help="Y increment per line (default: 5)")
    parser.add_argument("--initial-y", type=float, default=3,
                        help="Initial Y position (default: 3)")

    return parser.parse_args()


def svg_header(args):
    return (
        f'<svg font-family="{args.font}" '
        f'width="{args.width}" '
        f'height="{args.height}" '
        f'font-size="{args.font_size}">'
    )


def main():
    args = parse_args()

    final_code = ""

    # SVG Header
    header = svg_header(args)
    header += '<text x="0" y="'
    header += str(args.initial_y)
    header += '">'
    header_code, _, _, header_cells_state, header_ptr = autotune(header)

    # Dont use cells used for printing header later in the code (potential optimization area)
    move_right = len(header_cells_state) - header_ptr
    pointer_alignment = ">" * move_right if move_right > 0 else ""

    y_increment_setup = ">" + "+" * args.initial_y + "<"

    # initial part of logic for "print every character until new line
    detect_newline_logic = ",[----------[[++++++++++.[-]],----------]"

    # result of `autotune` for '</text><text x="0" y="'. Adjusted to clean cells
    # after itself as it'll run arbitrary amount of times and not touch the increment cell
    newline_prefix_1 = ">>+++++++++++++++[>++++>+++>++++++++>+++++++>++++++++>++++>++<<<<<<<-]>.>++.>----.>----.>.<<.>>>++.--.<<<.>.>.<<.>>>>++.<<.>+.>++.<<<<<+.>>>>>.--.<<+.>.>++.[-]<[-]<[-]<[-]<[-]<[-]<[-]<<<>"

    # Taken from https://esolangs.org/wiki/Brainfuck_algorithms#Print_value_of_cell_x_as_number_for_ANY_sized_cell_(eg_8bit,_100000bit_etc
    print_increment_value_logic = ">[-]>[-]+>[-]+<[>[-<-<<[->+>+<<]>[-<+>]>>]++++++++++>[-]+>[-]>[-]>[-]<<<<<[->-[>+>>]>[[-<+>]+>+>>]<<<<<]>>-[-<<+>>]<[-]++++++++[-<++++++>]>>[-<<+>>]<<]<[.[-]<]<<"

    # print '">'
    newline_prefix_2 = ">>+++++++[>+++++>+++++++++<<-]>-.>-.[-]<[-]<<<"

    # quits when end of file
    detect_newline_end_logic = ",]"

    exit_svg = ">>+++++++++++++++[>++++>+++>++++++++>+++++++>++++<<<<<-]>.>++.>----.>----.<++++.----.>>++.--.<<<.>-.+++.>++.>++."

    y_increment_logic = "+" * args.y_increment

    sections = [
        header_code,
        pointer_alignment,
        y_increment_setup,
        detect_newline_logic,
        newline_prefix_1,
        y_increment_logic,
        print_increment_value_logic,
        newline_prefix_2,
        detect_newline_end_logic,
        exit_svg,
    ]

    final_code = "".join(sections)

    print("Final code:")
    print(final_code)
    print()
    print("Code length:")
    print(len(final_code))

if __name__ == "__main__":
    main()
