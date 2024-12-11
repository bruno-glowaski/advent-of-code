#include "aoc.h"

#include <err.h>

#include "lamath.h"
#include "mdspan.h"

#define MD_GET(_span, _x, _y) MD_CM_GET(_span, _x, _y)
#define MD_VGET(_span, _vec) MD_CM_VGET(_span, _vec)
#define MD_GET_RAW(_arr, _stride, _x, _y) MD_CM_GET_RAW(_arr, _stride, _x, _y)

DECLARE_VEC(idx_t, 2, long2);
DECLARE_VEC_COPY(long2);
DECLARE_VEC_ADD(long2, 2);
typedef int8_t height_t;
typedef MDSPAN(height_t) height_mdspan_t;
typedef MDSPAN(bool) bmdspan_t;
typedef MDSPAN(int16_t) int16_mdspan_t;
DECLARE_MDSPAN_SEARCH(height_mdspan_t, search_height)

typedef height_mdspan_t input_t;

#define STDIN_COLS 42
#define STDIN_ROWS 42
#define STDIN_DIMS {STDIN_COLS, STDIN_ROWS}
#define STDIN_STRIDE 43
#define STDIN_BUFLEN (STDIN_COLS * STDIN_STRIDE)

#define INPUT_COLS STDIN_COLS
#define INPUT_ROWS STDIN_ROWS
#define INPUT_DIMS {INPUT_COLS, INPUT_ROWS}
#define INPUT_STRIDE 42
#define INPUT_BUFLEN (INPUT_COLS * INPUT_STRIDE)

input_t parse_input() {
  static char buffer[STDIN_BUFLEN] = {0};
  if (!(fread(buffer, sizeof(char), STDIN_BUFLEN - 1, stdin) || feof(stdin))) {
    err(ferror(stdin), NULL);
  }
  static height_t input_buffer[INPUT_BUFLEN] = {0};
  height_mdspan_t input = {
      .buffer = input_buffer,
      .dimensions = INPUT_DIMS,
      .stride = INPUT_STRIDE,
  };
  idx_t x = 0, y = 0;
  for_all_points_cm(x, y, input.dimensions) {
    MD_GET(input, x, y) = MD_GET_RAW(buffer, STDIN_STRIDE, x, y) - '0';
  }
  return input;
}

#ifdef RUN_TESTS
//  89010123
//  78121874
//  87430965
//  96549874
//  45678903
//  32019012
//  01329801
//  10456732

static const height_t input_buffer[64] = {
    8, 9, 0, 1, 0, 1, 2, 3, 7, 8, 1, 2, 1, 8, 7, 4, 8, 7, 4, 3, 0, 9,
    6, 5, 9, 6, 5, 4, 9, 8, 7, 4, 4, 5, 6, 7, 8, 9, 0, 3, 3, 2, 0, 1,
    9, 0, 1, 2, 0, 1, 3, 2, 9, 8, 0, 1, 1, 0, 4, 5, 6, 7, 3, 2,
};
static const height_mdspan_t input = {
    .buffer = input_buffer,
    .dimensions = {8, 8},
    .stride = 8,
};
#endif

#define TRAILHEAD_HEIGHT 0
#define TRAILEND_HEIGHT 9

int16_t calculate_score(long2 position, height_mdspan_t heightmap,
                        bmdspan_t visited) {
  MD_VGET(visited, position) = true;
  height_t current_height = MD_VGET(heightmap, position);
  if (current_height == TRAILEND_HEIGHT) {
    return 1;
  }

  int16_t total_score = 0;
  static const long2 DIRECTIONS[4] = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};
  for (size_t i = 0; i < 4; i++) {
    long2 next;
    long2_add(next, position, DIRECTIONS[i]);
    if (!MD_VIN(next, heightmap.dimensions)) {
      continue;
    }
    height_t next_height = MD_VGET(heightmap, next);
    if ((next_height - current_height) != 1) {
      continue;
    }
    if (MD_VGET(visited, next)) {
      continue;
    }
    total_score += calculate_score(next, heightmap, visited);
  }
  return total_score;
}

uint64_t part1(input_t input) {
  static bool visited_buffer[INPUT_BUFLEN] = {false};
  bmdspan_t visited = {
      .buffer = visited_buffer,
      .stride = input.dimensions[0],
  };
  memcpy(visited.dimensions, input.dimensions, sizeof(dim_t));

  uint64_t total_score = 0;
  long2 trailhead = {0, 0};
  while (search_height(input, &trailhead[0], &trailhead[1], TRAILHEAD_HEIGHT)) {
    memset(visited_buffer, 0, MD_CM_BUFLEN(visited) * sizeof(bool));
    total_score += calculate_score(trailhead, input, visited);
    next_vpoint_cm(trailhead, input.dimensions);
  }

  return total_score;
}

#define RATING_UNDEFINED -1

int16_t calculate_rating(long2 position, height_mdspan_t heightmap,
                         int16_mdspan_t ratings) {
  if (MD_VGET(ratings, position) != RATING_UNDEFINED) {
    return MD_VGET(ratings, position);
  }

  height_t current_height = MD_VGET(heightmap, position);
  if (current_height == TRAILEND_HEIGHT) {
    MD_VGET(ratings, position) = 1;
    return 1;
  }

  int16_t total_score = 0;
  static const long2 DIRECTIONS[4] = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};
  for (size_t i = 0; i < 4; i++) {
    long2 next;
    long2_add(next, position, DIRECTIONS[i]);
    if (!MD_VIN(next, heightmap.dimensions)) {
      continue;
    }
    height_t next_height = MD_VGET(heightmap, next);
    if ((next_height - current_height) != 1) {
      continue;
    }
    total_score += calculate_rating(next, heightmap, ratings);
  }
  MD_VGET(ratings, position) = total_score;
  return total_score;
}

uint64_t part2(input_t input) {
  static int16_t rating_buffer[INPUT_BUFLEN] = {false};
  int16_mdspan_t ratings = {
      .buffer = rating_buffer,
      .stride = input.dimensions[0],
  };
  memcpy(ratings.dimensions, input.dimensions, sizeof(dim_t));

  uint64_t total_score = 0;
  long2 trailhead = {0, 0};
  while (search_height(input, &trailhead[0], &trailhead[1], TRAILHEAD_HEIGHT)) {
    idx_t x = 0, y = 0;
    for_all_points_cm(x, y, ratings.dimensions) {
      MD_GET(ratings, x, y) = RATING_UNDEFINED;
    }
    total_score += calculate_rating(trailhead, input, ratings);
    next_vpoint_cm(trailhead, input.dimensions);
  }

  return total_score;
}

UTEST(day10, part1, { RX_INT_REQUIRE_EQUAL(part1(input), 36); });
UTEST(day10, part2, { RX_INT_REQUIRE_EQUAL(part2(input), 81); });

DECLARE_AOC_RUNNER()
