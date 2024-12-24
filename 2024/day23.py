from typing import DefaultDict, Set, List, Tuple
from collections import defaultdict

def part1(connections: List[List[str]]):
    network: DefaultDict[str, Set[str]] = defaultdict(lambda: set())
    triangles: List[List[str]] = []
    for v1, v2 in connections:
        network[v1].add(v2)
        network[v2].add(v1)
        for v3 in network[v1]:
            if v3 in network[v2]:
                triangles.append([v1, v2, v3])

    count_ts = 0
    for machines in triangles:
        for m in machines:
            if m[0] == "t":
                count_ts += 1
                break

    return count_ts

def max_clique(graph: DefaultDict[str, Set[str]]):
    def bron_kerbosch(R, P, X, max_clique_set):
        if not P and not X:
            if len(R) > len(max_clique_set[0]):
                max_clique_set[0] = R.copy()
            return

        pivot = max((len(set(graph[v]) & P) for v in P | X), default=0)
        pivot_vertex = next((v for v in P | X if len(set(graph[v]) & P) == pivot), None)

        for v in P - set(graph.get(pivot_vertex, [])):
            neighbors = set(graph[v])
            bron_kerbosch(R | {v}, P & neighbors, X & neighbors, max_clique_set)
            P = P - {v}
            X = X | {v}

    vertices = set(graph.keys())
    max_clique_set = [set()]
    bron_kerbosch(set(), vertices, set(), max_clique_set)
    return max_clique_set[0]

def part2(connections: List[List[str]]):
    direct_connections: DefaultDict[str, Set[str]] = defaultdict(lambda: set())
    for v1, v2 in connections:
        direct_connections[v1].add(v2)
        direct_connections[v2].add(v1)

    largest_set = max_clique(direct_connections)

    machines = list(largest_set)
    machines.sort()
    return ",".join(machines)

def main():
    with open("inputs/day23") as file:
        connections = [l.strip().split("-") for l in file.readlines()]

    print(f"Part 1: {part1(connections)}")
    print(f"Part 2: {part2(connections)}")

if __name__=="__main__":
    main()
