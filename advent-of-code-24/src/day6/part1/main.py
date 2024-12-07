def parse_map(map_str):
    lines = map_str.strip().split('\n')
    grid = [list(line) for line in lines]
    return grid

def find_guard(grid):
    directions = {'^': (-1, 0), '>': (0, 1), 'v': (1, 0), '<': (0, -1)}
    for r, row in enumerate(grid):
        for c, cell in enumerate(row):
            if cell in directions:
                return (r, c), directions[cell]
    return None, None

def turn_right(direction):
    turns = {(-1, 0): (0, 1), (0, 1): (1, 0), (1, 0): (0, -1), (0, -1): (-1, 0)}
    return turns[direction]

def simulate_guard(grid):
    start_pos, direction = find_guard(grid)
    if not start_pos:
        return 0

    visited = set()
    r, c = start_pos
    visited.add((r, c))

    while 0 <= r < len(grid) and 0 <= c < len(grid[0]):
        dr, dc = direction
        nr, nc = r + dr, c + dc

        if 0 <= nr < len(grid) and 0 <= nc < len(grid[0]) and grid[nr][nc] != '#':
            r, c = nr, nc
        else:
            direction = turn_right(direction)

        visited.add((r, c))

    return len(visited)

# Example usage
map_str = """
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"""
grid = parse_map(map_str)
result = simulate_guard(grid)
print(result)  # Output: 41