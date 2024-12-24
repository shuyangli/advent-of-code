from typing import Tuple, Dict
import re
from collections import defaultdict

def part1(initial_values: Dict[str, int], connections: Dict[str, Tuple[str, str, str]]):
    values = initial_values.copy()

    def evaluate(wire: str) -> int:
        if wire in values:
            return values[wire]

        input1, input2, operator = connections[wire]
        input1 = evaluate(input1)
        input2 = evaluate(input2)
        if operator == "AND":
            output = input1 & input2
        elif operator == "OR":
            output = input1 | input2
        elif operator == "XOR":
            output = input1 ^ input2
        values[wire] = output
        return output

    z_wires = sorted([w for w in connections.keys() if w.startswith("z")])

    output = 0
    for i, wire in enumerate(z_wires):
        output += evaluate(wire) * (2 ** i)

    return output

def part2(connections: Dict[str, Tuple[str, str, str]]):
    # Correct wiring:
    # z00 = x00 ^ y00
    # carry0 = x00 & y00

    # _sum_no_carry_01 = x01 ^ y01
    # z01 = _sum_no_carry_01 ^ carry0
    # carry1 = (x01 & y01) | (carry0 & _sum_no_carry_01)

    connections_by_inputs = defaultdict(lambda: None)

    def get_wire_key(inwire1, inwire2, operator):
        first_in, second_in = (inwire1, inwire2) if inwire1 < inwire2 else (inwire2, inwire1)
        return (first_in, second_in, operator)

    for outwire, (inwire1, inwire2, operator) in connections.items():
        connections_by_inputs[get_wire_key(inwire1, inwire2, operator)] = outwire

    last_carry = connections_by_inputs[("x00", "y00", "AND")]
    for i in range(1, 45):
        x_in = f"x{i:02}"
        y_in = f"y{i:02}"
        sum_without_carry_wire = connections_by_inputs[get_wire_key(x_in, y_in, "XOR")]
        if sum_without_carry_wire is None:
            print(f"{x_in} XOR {y_in} is nonexistent")
            break
        z_out = connections_by_inputs[get_wire_key(sum_without_carry_wire, last_carry, "XOR")]
        if z_out is None or z_out != f"z{i:02}":
            print(f"z{i:02} is wrong: encountered {z_out}, involved wires: {sum_without_carry_wire}, {last_carry}")
            break
        and_wire = connections_by_inputs[get_wire_key(x_in, y_in, "AND")]
        if and_wire is None:
            print(f"{x_in} AND {y_in} is nonexistent")
            break
        carryover_carry_wire = connections_by_inputs[get_wire_key(last_carry, sum_without_carry_wire, "AND")]
        if carryover_carry_wire is None:
            print(f"{last_carry} AND {sum_without_carry_wire} is nonexistent")
            break
        carry_wire = connections_by_inputs[get_wire_key(and_wire, carryover_carry_wire, "OR")]
        if carryover_carry_wire is None:
            print(f"{and_wire} OR {carryover_carry_wire} is nonexistent")
            break
        last_carry = carry_wire

    # Done by hand; how do I do this automatically?
    incorrect_wires = [
        "z06", "fkp", "z11", "ngr", "mfm", "z31", "krj", "bpt"
    ]
    return ",".join(sorted(incorrect_wires))

def main():
    with open("inputs/day24") as file:
        lines = [l.strip() for l in file.readlines()]

    initial_values: Dict[str, int] = {}
    # Connections: { destination => (wire, wire, operator) }
    connections: Dict[str, Tuple[str, str, str]] = {}

    parse_connections = False
    connection_regex = re.compile(r'(...) (AND|OR|XOR) (...) -> (...)')
    for line in lines:
        if not line:
            parse_connections = True
            continue
        if parse_connections:
            matches = connection_regex.match(line)
            connections[matches.group(4)] = (matches.group(1), matches.group(3), matches.group(2))
        else:
            wire, value = line.split(": ")
            initial_values[wire] = int(value)

    print(f"Part 1: {part1(initial_values, connections)}")
    print(f"Part 2: {part2(connections)}")

if __name__=="__main__":
    main()
