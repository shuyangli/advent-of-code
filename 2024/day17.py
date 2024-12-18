from typing import List, Tuple
import re

class Registers():
    def __init__(self, a: int, b: int, c: int):
        self.a = a
        self.b = b
        self.c = c
        self.output = []

    def __str__(self):
        return f"(a={self.a}, b={self.b}, c={self.c}, out={self.output})"

def get_combo_operand(operand: int, registers: Registers) -> int:
    if operand >= 0 and operand <= 3:
        return operand
    if operand == 4:
        return registers.a
    if operand == 5:
        return registers.b
    if operand == 6:
        return registers.c
    raise ValueError(f"Invalid operand value {operand}")

def execute(program: List[int], program_ptr: int, registers: Registers) -> int | None:
    if program_ptr < 0 or program_ptr >= len(program) - 1:
        # Halts
        return None
    op = program[program_ptr]
    literal_operand = program[program_ptr + 1]
    combo_operand = get_combo_operand(literal_operand, registers)

    if op == 0:
        registers.a = registers.a // (2 ** combo_operand)
    elif op == 1:
        registers.b = registers.b ^ literal_operand
    elif op == 2:
        registers.b = combo_operand % 8
    elif op == 3:
        if registers.a != 0:
            program_ptr = literal_operand - 2
    elif op == 4:
        registers.b = registers.b ^ registers.c
    elif op == 5:
        registers.output.append(combo_operand % 8)
    elif op == 6:
        registers.b = registers.a // (2 ** combo_operand)
    elif op == 7:
        registers.c = registers.a // (2 ** combo_operand)
    else:
        raise ValueError(f"Invalid operation {op}")

    program_ptr += 2
    return program_ptr

def part1(program: List[int], registers: Registers):
    program_ptr = 0
    while program_ptr is not None:
        program_ptr = execute(program, program_ptr, registers)

    return ",".join([str(o) for o in registers.output])

def part2(program: List[int]):
    # Program is:
    # loop do
    # B = A % 0x111
    # B = B ^ 0x101
    # C = A >> B
    # B = B ^ 0x110
    # B = B ^ C
    # OUTPUT B % 0x111
    # A = A // 8
    # end loop
    #
    # Register A can only get smaller, and we only need it 3 bits at a time
    # The final value is between 8^16 and 8^17 given output length.
    register_a = 0

    all_answers = []

    # (index, register_a so far)
    values_to_search = [(len(program) - 1, 0)]
    while values_to_search:
        (idx, register_a) = values_to_search.pop(0)
        if idx == -1:
            all_answers.append(register_a)
            continue

        target_output = program[idx]
        for next_three_bits_of_a in range(8):
            register_b = next_three_bits_of_a
            maybe_register_a = (register_a << 3) | next_three_bits_of_a
            register_b = register_b ^ 5
            register_c = maybe_register_a // (2 ** register_b)
            register_b = register_b ^ 6
            register_c_target = register_b ^ target_output
            if register_c % 8 == register_c_target:
                values_to_search.append((idx - 1, maybe_register_a))
    return min(all_answers)

def main():
    with open("inputs/day17") as file:
        lines = file.readlines()
        register_regex = re.compile(r'Register .: (-?[\d]+)')
        register_a = int(register_regex.match(lines[0]).group(1))
        register_b = int(register_regex.match(lines[1]).group(1))
        register_c = int(register_regex.match(lines[2]).group(1))

        program = [int(x) for x in lines[4].strip().split(" ")[1].split(",")]

    registers = Registers(register_a, register_b, register_c)
    print(f"Part 1: {part1(program, registers)}")
    print(f"Part 2: {part2(program)}")

if __name__=="__main__":
    main()
