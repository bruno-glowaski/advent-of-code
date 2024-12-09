#include <signal.h>
#include <stdint.h>
#include <stdio.h>

#define DECLARE_STUB(return_t, func_name, ...)                                 \
  _Noreturn return_t func_name(__VA_ARGS__) {                                  \
    raise(SIGIOT);                                                             \
    while (1) {                                                                \
    }                                                                          \
  }

#ifdef RUN_TESTS
#include "rexo.h"

#define UTEST(_suite, _name, _body) RX_TEST_CASE(_suite, _name) _body

#define DECLARE_AOC_RUNNER()                                                   \
  int main(int argc, const char *argv[]) {                                     \
    return rx_main(0, NULL, argc, argv) == RX_SUCCESS ? 0 : 1;                 \
  }
#else
#define UTEST(_suite, _name, _body)
#define DECLARE_AOC_RUNNER()                                                   \
  int main() {                                                                 \
    input_t input = parse_input();                                             \
    printf("Part 1: %li\n", part1(input));                                     \
    printf("Part 2: %li\n", part2(input));                                     \
  }
#endif
