def calculate_joltage(bank: str, k: int) -> int:
    digits = [int(x) for x in bank.strip()]
    first_biggest = _idx_of_max(_get_candidate_digits(digits, -1, k))
    return _do_calculate_joltage(digits, k - 1, [first_biggest])


def _do_calculate_joltage(digits: list[int], k: int, acc: list[int]):
    if k == 0:
        return int("".join(str(digits[idx]) for idx in acc))
    offset = acc[-1]
    candidates = _get_candidate_digits(digits, offset, k)
    acc.append(_idx_of_max(candidates) + offset + 1)
    return _do_calculate_joltage(digits, k - 1, acc)


def _idx_of_max(digits: list[int]) -> int:
    return digits.index(max(digits))


def _get_candidate_digits(digits, previous_idx, k):
    if k == 1:
        return digits[previous_idx + 1 :]
    return digits[previous_idx + 1 : 1 - k]


def part1():
    with open("input.txt") as f:
        return sum(calculate_joltage(bank, 2) for bank in f.readlines())


def part2():
    with open("input.txt") as f:
        return sum(calculate_joltage(bank, 12) for bank in f.readlines())


if __name__ == "__main__":
    print(f"Part 1: {part1()}")
    print(f"Part 2: {part2()}")
