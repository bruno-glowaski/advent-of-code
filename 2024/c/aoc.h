#include <signal.h>
#include <stdio.h>

#define DECLARE_AOC_RUNNER_ALLOC()                                             \
  int main() {                                                                 \
    input_t input = parse_input();                                             \
    printf("Part 1: %li\n", part1(input));                                     \
    printf("Part 2: %li\n", part2(input));                                     \
    free_input(input);                                                         \
  }

#define DECLARE_AOC_RUNNER()                                                   \
  int main() {                                                                 \
    input_t input = parse_input();                                             \
    printf("Part 1: %li\n", part1(input));                                     \
    printf("Part 2: %li\n", part2(input));                                     \
  }

#define DECLARE_STUB(return_t, func_name, ...)                                 \
  _Noreturn return_t func_name(__VA_ARGS__) {                                  \
    raise(SIGIOT);                                                             \
    while (1) {                                                                \
    }                                                                          \
  }
