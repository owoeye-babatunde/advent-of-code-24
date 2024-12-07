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
    return update[len(update) // 2]

def main(input_data):
    rules, updates = parse_input(input_data)
    valid_updates = [update for update in updates if is_update_valid(update, rules)]
    middle_pages_sum = sum(get_middle_page(update) for update in valid_updates)
    return middle_pages_sum

if __name__ == "__main__":
    with open("input.txt", "r") as file:
        input_data = file.read()
    result = main(input_data)
    print(result)