#!/usr/bin/env python

import sys
import math

def analyze_string(s: str):
    values = [ord(c) for c in s]
    mean = sum(values) / len(values)
    variance = sum(abs(v - mean) for v in values) / len(values)
    return mean, variance

def generate_path(input_string: str, max_branch_distance: int):
    'Generate string path'
    branch_list = []
    sequence = []
    for input_char in input_string:
        min_branch_distance = max_branch_distance
        closest_branch = None
        for branch_head in enumerate(branch[-1] for branch in branch_list):
            branch_distance = abs(branch_head[1] - ord(input_char))
            if branch_distance < min_branch_distance:
                closest_branch = branch_head[0]
        if closest_branch:
            sequence.append(closest_branch)
            branch_list[closest_branch].append(ord(input_char))
        else:
            sequence.append(len(branch_list))
            branch_list.append([ord(input_char)])
    return (branch_list, tuple(sequence))

def shift_by(num: int):
    'Return shift string'
    if not num:
        return ''
    return '>' * num if num > 0 else '<' * -num

def change_by(num: int):
    'Return increment/decrement string'
    if not num:
        return ''
    return '+' * num if num > 0 else '-' * -num

def generate_code(input_string: str, max_branch_distance: int, loop_base: int):
    final_code = '+' * loop_base + '['
    appr = []
    branches, seq = generate_path(input_string, max_branch_distance)
    for branch in branches:
        coefficient = round(branch[0] / loop_base)
        final_code += ">" + change_by(coefficient)
        appr.append(coefficient * loop_base)
    final_code += shift_by(-len(branches)) + '-]>'
    indexes = [0] * len(branches)
    for step, branch_index in enumerate(seq):
        index = indexes[branch_index]
        branch = branches[branch_index]
        if step > 0:
            final_code += shift_by(branch_index - seq[step - 1])
        if index > 0:
            final_code += change_by(branch[index] - branch[index - 1])
        else:
            final_code += change_by(branch[0] - appr[branch_index])
        indexes[branch_index] += 1
        final_code += '.'
    final_pointer = seq[-1] if seq else 0
    final_cells = [branch[-1] for branch in branches]
    return final_code, final_cells, final_pointer

def autotune(input_string: str):

    def test_range(dist_range, base_range, current_best):
        best = current_best
        best_len = current_best[1] if current_best else None

        for dist in dist_range:
            for base in base_range:
                code, cells_state, ptr = generate_code(input_string, dist, base)
                length = len(code)

                if best_len is None or length < best_len:
                    best = (code, length, (dist, base), cells_state, ptr)
                    best_len = length

        return best

    # --- stage 1: coarse global search ---
    best = test_range(range(1, 41, 3), range(4, 17, 2), None)

    _, _, (d, b), _, _ = best

    # --- stage 2: local refinement ---
    best = test_range(range(max(1,d-3), d+4),
                      range(max(3,b-3), b+4),
                      best)

    # --- stage 3: fine search ---
    _, _, (d, b), _, _ = best
    best = test_range(range(max(1,d-1), d+2),
                      range(max(3,b-1), b+2),
                      best)

    return best

def main():
    clean = False
    args = sys.argv[1:]

    input_string = args[0]

    best_code, best_len, (best_dist, best_base), cells_state, ptr = autotune(input_string)

    print(f"Best branch distance: {best_dist}")
    print(f"Best loop base: {best_base}")
    print(f"Memory cells used: {len(cells_state)}")
    print(f"Final pointer position: {ptr}")
    print("Final tape state:")
    tape_str = []
    for i, v in enumerate(cells_state):
        if i == ptr:
            tape_str.append(f"[{v}]")
        else:
            tape_str.append(str(v))
    print(" ".join(tape_str))
    print(f"Program length: {best_len}")
    print()
    print(best_code)

if __name__ == "__main__":
    main()
