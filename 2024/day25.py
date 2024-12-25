from typing import List

def part1(locks: List[List[int]], keys: List[List[int]]):
    def does_key_fit_lock(lock: List[int], key: List[int]) -> bool:
        for l, k in zip(lock, key):
            if l + k > 5:
                return False
        return True

    num_combinations = 0
    for lock in locks:
        for key in keys:
            if does_key_fit_lock(lock, key):
                num_combinations += 1
    return num_combinations

def part2():
    return "Unimplemented"

def main():
    with open("inputs/day25") as file:
        lines = [l.strip() for l in file.readlines()]

    locks: List[List[int]] = []
    keys: List[List[int]] = []
    for i in range(0, len(lines), 8):
        if lines[i] == "#####":
            # This is a lock
            lock = [0, 0, 0, 0, 0]
            for j in range(i + 1, i + 6):
                for k, char in enumerate(lines[j]):
                    if char == "#":
                        lock[k] += 1
            locks.append(lock)
        elif lines[i] == ".....":
            # This is a key
            key = [0, 0, 0, 0, 0]
            for j in range(i + 1, i + 6):
                for k, char in enumerate(lines[j]):
                    if char == "#":
                        key[k] += 1
            keys.append(key)

    print(f"Part 1: {part1(locks, keys)}")
    print(f"Part 2: {part2()}")

if __name__=="__main__":
    main()
