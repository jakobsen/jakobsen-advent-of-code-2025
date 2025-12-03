from day3 import calculate_joltage


def test_calculate_small_joltage() -> None:
    assert calculate_joltage("987654321111111", 2) == 98
    assert calculate_joltage("811111111111119", 2) == 89
    assert calculate_joltage("234234234234278", 2) == 78
    assert calculate_joltage("818181911112111", 2) == 92


def test_calculate_large_joltage() -> None:
    assert calculate_joltage("987654321111111", 12) == 987654321111
    assert calculate_joltage("811111111111119", 12) == 811111111119
    assert calculate_joltage("234234234234278", 12) == 434234234278
    assert calculate_joltage("818181911112111", 12) == 888911112111
