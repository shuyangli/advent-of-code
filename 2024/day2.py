def part1():
    def is_safe_line(levels: list[int]):
        deltas: list[int] = []
        for i in range(len(levels) - 1):
            deltas.append(levels[i + 1] - levels[i])
        
        for d in deltas:
            if (d > 0) != (deltas[0] > 0):
                return False
            if abs(d) > 3 or abs(d) < 1:
                return False
        return True

    with open("inputs/day2", "r") as file:
        lines = file.readlines()

    num_safe_lines = 0

    for line in lines:
        values = [int(x) for x in line.split()]
        print(f"values: {values}")
        if is_safe_line(values):
            num_safe_lines += 1

    return num_safe_lines

def part2():
    def is_safe_line(levels: list[int]):
        # Returns index of the first violation found.
        def first_violation(levels: list[int]):
            is_positive = levels[1] > levels[0]
            for i in range(len(levels) - 1):
                delta = levels[i + 1] - levels[i]
                if (delta > 0) != is_positive:
                    return i
                if abs(delta) > 3 or abs(delta) < 1:
                    return i
            return -1
        
        # When we encounter the first violation, try removing one of the
        # two values at the violation and proceed with the validation.
        # Also if the index of the first violation is 1, try removing one of the
        # first two values to see if the increasing/decreasing status is wrong.
        i = first_violation(levels)
        if i == -1 or i == len(levels) - 2:
            return True
        if first_violation(levels[:i] + levels[i + 1:]) == -1 or first_violation(levels[:i + 1] + levels[i + 2:]) == -1:
            return True
        if i == 1 and first_violation(levels[1:]) == -1:
            return True
        return False
    
    with open("inputs/day2", "r") as file:
        lines = file.readlines()

    num_safe_lines = 0

    for line in lines:
        values = [int(x) for x in line.split()]
        if is_safe_line(values):
            num_safe_lines += 1

    return num_safe_lines



def main():
    print(f"Part 1: {part1()}")
    print(f"Part 2: {part2()}")

if __name__=="__main__":
    main()
