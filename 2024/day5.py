from collections import defaultdict
from typing import Dict, DefaultDict, List, Set, Tuple

def is_valid_manual(page_dependencies: Dict[str, Set[str]], manual: List[str]):
    pages_produced = set()
    dependencies_not_seen = set()
    for page in manual:
        if page in dependencies_not_seen:
            return False
        if page in page_dependencies:
            for dependency in page_dependencies[page]:
                if dependency not in pages_produced:
                    dependencies_not_seen.add(dependency)
        pages_produced.add(page)
    return True

def sort_manual(page_dependencies: Dict[str, Set[str]], manual: List[str]) -> List[str]:
    # {later page => {earlier pages}}
    required_dependencies: DefaultDict[str, Set[str]] = defaultdict(lambda: set())

    for i in range(len(manual)):
        first_page = manual[i]
        # Initialize all values in the dict
        required_dependencies[first_page]
        for j in range(i, len(manual)):
            second_page = manual[j]
            if first_page in page_dependencies[second_page]:
                required_dependencies[second_page].add(first_page)
            if second_page in page_dependencies[first_page]:
                required_dependencies[first_page].add(second_page)

    # Then topo-sort the edges
    pages_produced = []
    while required_dependencies:
        for page, dependencies in required_dependencies.items():
            if not dependencies:
                pages_produced.append(page)
                del required_dependencies[page]
                for _, dependencies in required_dependencies.items():
                    dependencies.discard(page)
                break

    return pages_produced

def part1(page_dependencies: Dict[str, Set[str]], manuals: List[List[str]]):
    sum_middle_pages = 0
    for manual in manuals:
        if is_valid_manual(page_dependencies, manual):
            sum_middle_pages += int(manual[len(manual) // 2])

    return sum_middle_pages

def part2(page_dependencies: Dict[str, Set[str]], manuals: List[List[str]]):
    sum_middle_pages = 0
    for manual in manuals:
        if not is_valid_manual(page_dependencies, manual):
            sorted_manual = sort_manual(page_dependencies, manual)
            sum_middle_pages += int(sorted_manual[len(sorted_manual) // 2])

    return sum_middle_pages

def main():
    # page_dependencies: {later page => {earlier pages}}
    page_dependencies = defaultdict(lambda: set())
    manuals: List[List[str]] = []

    with open("inputs/day5", "r") as file:
        is_parsing_dependencies = True
        for line in file.readlines():
            if not line.strip():
                is_parsing_dependencies = False
                continue
            if is_parsing_dependencies:
                earlier_page, later_page = line.strip().split("|")
                page_dependencies[later_page].add(earlier_page)
            else:
                manuals.append(line.strip().split(","))

    print(f"Part 1: {part1(page_dependencies, manuals)}")
    print(f"Part 2: {part2(page_dependencies, manuals)}")

if __name__=="__main__":
    main()
