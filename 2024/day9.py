from typing import List, Tuple
from collections import namedtuple

def part1(disk_layout: str):
    def parse_disk_data() -> List[int | None]:
        disk_data: List[int | None] = []
        is_file = True
        file_id: int = 0
        for char in disk_layout:
            if is_file:
                # If it's file, we push the content onto disk
                disk_data.extend([file_id] * int(char))
                file_id += 1
            else:
                # If it's empty space, we push Nones onto disk
                disk_data.extend([None] * int(char))
            is_file = not is_file
        return disk_data

    # Swap data on disk until the two pointers meet
    disk_data = parse_disk_data()
    start_idx = 0
    end_idx = len(disk_data) - 1
    while True:
        while start_idx < len(disk_data) and disk_data[start_idx] is not None:
            start_idx += 1
        while end_idx >= 0 and disk_data[end_idx] is None:
            end_idx -= 1
        if start_idx >= end_idx or start_idx >= len(disk_data) or end_idx < 0:
            break
        disk_data[start_idx], disk_data[end_idx] = disk_data[end_idx], disk_data[start_idx]
        start_idx += 1
        end_idx -= 1

    checksum = 0
    for idx, value in enumerate(disk_data):
        if value is None:
            break
        checksum += idx * value
    return checksum

DiskFile = namedtuple("DiskFile", ["size", "file_id"])

def part2(disk_layout: str):
    # List of (size, file_id)
    def parse_disk_data() -> List[DiskFile]:
        disk_data = []
        is_file = True
        file_id: int = 0
        for char in disk_layout:
            if is_file:
                # If it's file, we push the content onto disk
                disk_data.append(DiskFile(int(char), file_id))
                file_id += 1
            else:
                # If it's empty space, we push Nones onto disk
                disk_data.append(DiskFile(int(char), None))
            is_file = not is_file
        return disk_data
    disk_data = parse_disk_data()

    # Swap one file at a time; this is n^2, but the list is small, so ¯\_(ツ)_/¯
    end_idx = len(disk_data) - 1
    while end_idx > 0:
        file = disk_data[end_idx]
        if file.file_id is None:
            end_idx -= 1
            continue
        # From left to right, find a large enough space to move the file to, and possibly split the space
        # Don't need to worry about repeatedly moving a file, since the first time it will move to the leftmost space that fits it.
        for start_idx in range(0, end_idx):
            maybe_space = disk_data[start_idx]
            if maybe_space.file_id is not None:
                continue
            if maybe_space.size < file.size:
                continue
            # Found a space, swap and maybe insert.
            disk_data[start_idx] = file
            disk_data[end_idx] = DiskFile(file.size, None)
            if maybe_space.size > file.size:
                disk_data.insert(start_idx + 1, DiskFile(maybe_space.size - file.size, None))
            break

        # We've either moved the file or tried all the spaces, move backwards to the next file
        end_idx -= 1

    checksum = 0
    current_disk_idx = 0
    for file in disk_data:
        if file.file_id is not None:
            for idx in range(current_disk_idx, current_disk_idx + file.size):
                checksum += idx * file.file_id
        current_disk_idx += file.size
    return checksum

def main():
    with open("inputs/day9") as file:
        disk_layout = file.read().strip()

    print(f"Part 1: {part1(disk_layout)}")
    print(f"Part 2: {part2(disk_layout)}")

if __name__=="__main__":
    main()
