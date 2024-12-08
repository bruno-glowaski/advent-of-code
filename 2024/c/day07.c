#include "shared.h"

#include <err.h>
#include <errno.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef uint64_t number_t;

#define EQUATION_TERMINATOR 0
#define EQUATION_VALUE_LEN 30

struct equation_t {
  number_t test_value;
  uint32_t value_count;
  number_t values[EQUATION_VALUE_LEN];
};

#define EQUATIONS_LEN 850

typedef struct {
  size_t equation_count;
  const struct equation_t *equations;
} input_t;

#define BUFFER_SIZE 64
input_t parse_input() {
  static struct equation_t equations[EQUATIONS_LEN];
  char buffer[BUFFER_SIZE];

  size_t equation_count = 0;
  while (fgets(buffer, BUFFER_SIZE, stdin) != NULL) {
    char *b_cursor = buffer;
    struct equation_t equation = {0};
    int offset;

    if (sscanf(b_cursor, "%li: %n", &equation.test_value, &offset) != 1) {
      fprintf(stderr, "error: invalid equation header at %li\n",
              equation_count);
      exit(EINVAL);
    }
    b_cursor += offset;

    do {
      if (sscanf(b_cursor, "%li %n", &equation.values[equation.value_count++],
                 &offset) != 1) {
        fprintf(stderr, "error: invalid equation number at %li, %i\n",
                equation_count, equation.value_count);
        exit(EINVAL);
      }
      b_cursor += offset;
    } while (*b_cursor);

    equations[equation_count++] = equation;
  }
  if (!feof(stdin)) {
    err(ferror(stdin), NULL);
  }

  input_t result = {.equation_count = equation_count, .equations = equations};
  return result;
}

enum operator_t {
  OP_ADD = 0,
  OP_MUL = 1,
  OP_CONCAT = 2,
};

static inline number_t concat_numbers(number_t a, number_t b) {
  const number_t initial_base = 10;
  number_t preffix = 10;
  while (preffix <= b) {
    number_t base = initial_base;
    while (base * base <= b) {
      base *= base;
    }
    preffix *= base;
  }
  return a * preffix + b;
}

static inline bool test_op_permutation(uint64_t op_permutation,
                                       uint64_t op_count,
                                       struct equation_t equation) {
  number_t acc = equation.values[0];
  for (int i = 1; i < equation.value_count; i++) {
    number_t next = equation.values[i];
    switch (op_permutation % op_count) {
    default:
    case OP_ADD:
      acc = acc + next;
      break;
    case OP_MUL:
      acc = acc * next;
      break;
    case OP_CONCAT:
      acc = concat_numbers(acc, next);
      break;
    }
    op_permutation = op_permutation / op_count;
  }
  return acc == equation.test_value;
}

number_t part1(input_t input) {
  number_t result = 0;
  for (size_t i = 0; i < input.equation_count; i++) {
    struct equation_t current = input.equations[i];
    uint32_t permutation_count = 1 << (current.value_count - 1);
    for (uint32_t permutation = 0; permutation < permutation_count;
         permutation++) {
      if (test_op_permutation(permutation, 2, current)) {
        result += current.test_value;
        break;
      }
    }
  }
  return result;
}

uint64_t ulipow(uint64_t base, uint64_t exp) {
  uint32_t result = 1;
  while (true) {
    if (exp % 2) {
      result *= base;
    }
    exp /= 2;
    if (exp == 0) {
      break;
    }
    base *= base;
  }
  return result;
}

number_t part2(input_t input) {
  number_t result = 0;
  for (size_t i = 0; i < input.equation_count; i++) {
    struct equation_t current = input.equations[i];
    uint64_t permutation_count = ulipow(3, current.value_count - 1);
    for (uint64_t permutation = 0; permutation < permutation_count;
         permutation++) {
      if (test_op_permutation(permutation, 3, current)) {
        result += current.test_value;
        break;
      }
    }
  }
  return result;
}

DECLARE_AOC_RUNNER();
