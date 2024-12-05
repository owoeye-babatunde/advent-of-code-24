def parse_input(input_data):
    rules_section, updates_section = input_data.strip().split("\n\n")
    rules = [tuple(map(int, rule.split("|"))) for rule in rules_section.split("\n")]
    updates = [list(map(int, update.split(","))) for update in updates_section.split("\n")]
    return rules, updates

def is_update_valid(update, rules):
    for rule in rules:
        if rule[0] in update and rule[1] in update:
            if update.index(rule[0]) > update.index(rule[1]):
                return False
    return True

def get_middle_page(update):
    try:
        return update[len(update) // 2]
    except Exception as e:
        print(update, "   error: ", e)

def order_update(update, rules):
    from collections import defaultdict, deque

    graph = defaultdict(list)
    in_degree = defaultdict(int)
    nodes_in_update = set(update)

    # Build the graph and in-degree count
    for rule in rules:
        if rule[0] in nodes_in_update and rule[1] in nodes_in_update:
            graph[rule[0]].append(rule[1])
            in_degree[rule[1]] += 1
            if rule[0] not in in_degree:
                in_degree[rule[0]] = 0

    # Initialize the queue with nodes having zero in-degree
    queue = deque([node for node in update if in_degree[node] == 0])
    ordered_update = []

    while queue:
        node = queue.popleft()
        ordered_update.append(node)
        for neighbor in graph[node]:
            in_degree[neighbor] -= 1
            if in_degree[neighbor] == 0:
                queue.append(neighbor)

    # Ensure all nodes in the update are included in the ordered update
    ordered_update = [node for node in ordered_update if node in update]

    # If there are nodes left out, add them in the order they appear in the original update
    remaining_nodes = [node for node in update if node not in ordered_update]
    ordered_update.extend(remaining_nodes)

    return ordered_update

def main(input_data):
    rules, updates = parse_input(input_data)
    invalid_updates = [update for update in updates if not is_update_valid(update, rules)]
    #print(invalid_updates)
    corrected_updates = [order_update(update, rules) for update in invalid_updates]
    #print(corrected_updates)
    middle_pages_sum = sum(get_middle_page(update) for update in corrected_updates)
    return middle_pages_sum

if __name__ == "__main__":
    with open("input.txt", "r") as file:
        input_data = file.read()
    result = main(input_data)
    print(result)