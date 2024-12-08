#include "aoc.h"

#include <err.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#include "lamath.h"
#include "mdspan.h"
#define MD_GET(_span, _x, _y) MD_CM_GET(_span, _x, _y)
#define MD_VGET(_span, _vec) MD_CM_VGET(_span, _vec)

DECLARE_VEC(idx_t, 2, long2)
DECLARE_VEC_COPY(long2)
DECLARE_VEC_EQ(long2, 2)
DECLARE_VEC_ADD(long2, 2)
DECLARE_VEC_SUB(long2, 2)
DECLARE_VEC_NEG(long2, 2)
DECLARE_VEC_SDIV(long2, idx_t, 2)

struct antenna_bucket_t {
  size_t count;
  long2 *positions;
};

#define ANTENNA_BUCKET_COUNT 255

typedef struct {
  struct antenna_bucket_t antenna_buckets[ANTENNA_BUCKET_COUNT];
  dim_t map_dimensions;
} input_t;

typedef MDSPAN(char) mdspan_t;
typedef MDSPAN(bool) bmdspan_t;
DECLARE_MDSPAN_SEARCH(mdspan_t, mdspan_search_char);

#define INPUT_ROWS 50
#define INPUT_COLS 50
#define INPUT_DIMS {INPUT_ROWS, INPUT_COLS}
#define INPUT_STRIDE (INPUT_ROWS + 1)
#define INPUT_BUFLEN (INPUT_COLS * INPUT_STRIDE)
#define INPUT_POSITION_BUFLEN 200

#define TILE_EMPTY '.'

input_t parse_input() {
  static char buffer[INPUT_BUFLEN];
  if (!(fread(buffer, sizeof(char), INPUT_BUFLEN, stdin) || feof(stdin))) {
    err(ferror(stdin), NULL);
  }
  buffer[INPUT_BUFLEN - 1] = '\0';
  const mdspan_t map = {
      .buffer = buffer,
      .dimensions = INPUT_DIMS,
      .stride = INPUT_STRIDE,
  };

  static long2 position_buffer[ANTENNA_BUCKET_COUNT][INPUT_POSITION_BUFLEN];
  input_t input = {.antenna_buckets = {0}, .map_dimensions = INPUT_DIMS};
  long2 p = {0};
  for_all_points_cm(p[0], p[1], map.dimensions) {
    char c = MD_GET(map, p[0], p[1]);
    if (c == TILE_EMPTY) {
      continue;
    }
    input.antenna_buckets[c].positions = position_buffer[c];
    memcpy(position_buffer[c][input.antenna_buckets[c].count++], p,
           sizeof(long2));
  }

  return input;
}

#define VISITED_COLS INPUT_COLS
#define VISITED_ROWS INPUT_ROWS
#define VISITED_STRIDE INPUT_COLS
#define VISITED_DIM {VISITED_COLS, VISITED_ROWS}
#define VISITED_BUFLEN (VISITED_ROWS * VISITED_STRIDE)

uint64_t part1(input_t input) {
  bool visited_buffer[VISITED_BUFLEN] = {false};
  bmdspan_t visited = {
      .buffer = visited_buffer,
      .stride = VISITED_STRIDE,
      .dimensions = VISITED_DIM,
  };

  uint64_t count = 0;
  for (int i = 0; i < 255; i++) {
    struct antenna_bucket_t *current_bucket = &input.antenna_buckets[i];
    if (current_bucket->count == 0) {
      continue;
    }
    for (size_t j = 0; j < current_bucket->count - 1; j++) {
      for (size_t k = j + 1; k < current_bucket->count; k++) {
        long2 start, end;
        long2_copy(start, current_bucket->positions[j]);
        long2_copy(end, current_bucket->positions[k]);

        long2 delta;
        long2_sub(delta, end, start);

        long2_add(end, end, delta);
        long2_neg(delta, delta);
        long2_add(start, start, delta);

        if (MD_VIN(start, visited.dimensions)) {
          count += !MD_VGET(visited, start);
          MD_VGET(visited, start) = true;
        }
        if (MD_VIN(end, visited.dimensions)) {
          count += !MD_VGET(visited, end);
          MD_VGET(visited, end) = true;
        }
      }
    }
  }
  return count;
}

idx_t gcd(idx_t a, idx_t b) {
  while (b) {
    idx_t t = b;
    b = a % b;
    a = t;
  }
  return a;
}

uint64_t part2(input_t input) {
  bool visited_buffer[VISITED_BUFLEN] = {false};
  bmdspan_t visited = {
      .buffer = visited_buffer,
      .stride = VISITED_STRIDE,
      .dimensions = VISITED_DIM,
  };

  uint64_t count = 0;
  for (int i = 0; i < 255; i++) {
    struct antenna_bucket_t *current_bucket = &input.antenna_buckets[i];
    if (current_bucket->count == 0) {
      continue;
    }
    for (size_t j = 0; j < current_bucket->count - 1; j++) {
      for (size_t k = j + 1; k < current_bucket->count; k++) {
        long2 start, end;
        long2_copy(start, current_bucket->positions[j]);
        long2_copy(end, current_bucket->positions[k]);

        long2 delta;
        long2_sub(delta, end, start);
        long2_sdiv(delta, delta, gcd(delta[0], delta[1]));

        long2 current;
        long2_copy(current, end);
        while (long2_add(current, current, delta),
               MD_VIN(current, visited.dimensions)) {
          count += !MD_VGET(visited, current);
          MD_VGET(visited, current) = true;
        }

        long2_copy(current, start);
        while (MD_VIN(current, visited.dimensions)) {
          count += !MD_VGET(visited, current);
          MD_VGET(visited, current) = true;
          long2_add(current, current, delta);
        }

        long2_neg(delta, delta);
        long2_copy(current, start);
        while (long2_add(current, current, delta),
               MD_VIN(current, visited.dimensions)) {
          count += !MD_VGET(visited, current);
          MD_VGET(visited, current) = true;
        }
      }
    }
  }
  return count;
}

DECLARE_AOC_RUNNER()
