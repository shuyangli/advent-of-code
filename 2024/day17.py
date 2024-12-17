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

# (program_ptr, output)
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
    # C = A // B
    # B = B ^ 0x110
    # B = B ^ C
    # OUTPUT B % 0x111
    # A = A // 8
    # end loop
    #
    # Register A can only get smaller, and we only need it 3 bits at a time
    # The final value is between 8^16 and 8^17 given output length.
    for register_a in range(64):
        print(f"Trying register_a={register_a}")
        registers = Registers(register_a, 0, 0)
        program_ptr = 0
        while program_ptr is not None:
            program_ptr = execute(program, program_ptr, registers)
        print("Output : ", registers.output)
        # print("Program: ", program)
        # if registers.output == program:
        #     return register_a
        # elif len(registers.output) < len(program):
        #     lower_bound = register_a
        # elif len(registers.output) > len(program):
        #     upper_bound = register_a
        # else:
        #     print("Same length, checking the lower values")
        #     # This comparison is wrong. Can't just compare the values elementwise.
        #     elementwise = zip(reversed(registers.output), reversed(program))
        #     for r, p in elementwise:
        #         if r < p:
        #             lower_bound = register_a
        #             break
        #         elif r > p:
        #             upper_bound = register_a
        #             break
        # register_a = (lower_bound + upper_bound) // 2

        # if lower_bound + 2 > upper_bound:
        #     break
    return "Did not terminate."

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
