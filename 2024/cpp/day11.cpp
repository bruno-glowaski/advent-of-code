#include <algorithm>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <iterator>
#include <print>
#include <unordered_map>
#include <vector>

using number_t = uint64_t;
using subdiv_t = uint64_t;
using blink_t = uint8_t;
using input_t = std::vector<number_t>;
using memo_key_t = std::pair<blink_t, number_t>;

struct memo_hasher {
  size_t operator()(const memo_key_t &p) const {
    return std::hash<uint64_t>{}(((uint64_t)p.first << 56) | p.second);
  }
};

using memo_t =
    std::unordered_map<std::pair<blink_t, number_t>, subdiv_t, memo_hasher>;

input_t parse_input() {
  std::istream_iterator<number_t> start{std::cin}, end{};
  std::vector<number_t> input{start, end};
  return input;
}

subdiv_t count_stone_subdivisions(blink_t n_blinks, number_t n, memo_t &memo) {
  if (n_blinks == 0) {
    return 1;
  }

  if (memo.contains({n_blinks, n})) {
    return memo[{n_blinks, n}];
  }

  subdiv_t result;
  int n_digits;
  if (n == 0) {
    result = count_stone_subdivisions(n_blinks - 1, 1, memo);
    goto end;
  }

  n_digits = std::log10(n) + 1;
  if (n_digits % 2 == 0) {
    number_t mask = std::pow(10, n_digits / 2);
    number_t a = n / mask, b = n % mask;
    result = count_stone_subdivisions(n_blinks - 1, a, memo) +
             count_stone_subdivisions(n_blinks - 1, b, memo);
    goto end;
  }

  result = count_stone_subdivisions(n_blinks - 1, n * 2024, memo);

end:
  memo[{n_blinks, n}] = result;
  return result;
}

uint64_t part1(const input_t &input) {
  memo_t memo{};
  return std::ranges::fold_left(input, 0, [&memo](subdiv_t acc, number_t n) {
    return acc + count_stone_subdivisions(25, n, memo);
  });
}

uint64_t part2(const input_t &input) {
  memo_t memo{};
  return std::ranges::fold_left(input, 0, [&memo](subdiv_t acc, number_t n) {
    return acc + count_stone_subdivisions(75, n, memo);
  });
}

int main(void) {
  input_t input = parse_input();
  std::println("Part 1: {}", part1(input));
  std::println("Part 2: {}", part2(input));
}
