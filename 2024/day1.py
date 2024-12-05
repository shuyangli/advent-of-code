from collections import defaultdict

def part1():
    with open("inputs/day1", "r") as file:
        lines = file.readlines()

    first_list: list[int] = []
    second_list: list[int] = []

    for line in lines:
        [first, second] = line.split()
        first_list.append(int(first))
        second_list.append(int(second))
    
    first_list.sort()
    second_list.sort()

    sum_distances = 0
    for first, second in zip(first_list, second_list):
        sum_distances += abs(first - second)

    return sum_distances

def part2():
    with open("inputs/day1", "r") as file:
        lines = file.readlines()

    first_list: list[int] = []
    second_dict = defaultdict(lambda: 0)

    for line in lines:
        [first, second] = line.split()
        first_list.append(int(first))
        second_dict[int(second)] += 1

    similarity = 0
    for value in first_list:
        similarity += value * second_dict[value]

    return similarity

def main():
    print(f"Part 1: {part1()}")
    print(f"Part 2: {part2()}")

if __name__=="__main__":
    main()
