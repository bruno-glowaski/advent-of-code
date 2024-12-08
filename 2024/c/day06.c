#include <err.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

typedef intptr_t idx_t;
typedef idx_t dim_t[2];
struct mdspan_t {
  char *buffer;
  dim_t dimensions;
  idx_t stride;
};

#define MD_IN(_x, _y, _dimensions)                                             \
  ((_x) >= 0 && (_y) >= 0 && (_x) < (_dimensions)[0] && (_y) < (_dimensions)[1])
#define for_all_points(_x, _y, _dimensions)                                    \
  for ((_x) = 0, (_y) = 0; MD_IN((_x), (_y), (_dimensions));                   \
       (_y) += ((_x) + 1) / (_dimensions)[0],                                  \
      (_x) = ((_x) + 1) % (_dimensions)[0])
#define MD_CM_MAPPING(_x, _y, _stride) ((_y) * (_stride) + (_x))
#define MD_CM_BUFLEN(_span)                                                    \
  MD_CM_MAPPING(0, (_span).dimensions[1], (_span).stride)
#define MD_CM_GET(_x, _y, _span)                                               \
  ((_span).buffer[MD_CM_MAPPING((_x), (_y), (_span).stride)])

typedef struct mdspan_t input_t;

static inline void mdspan_clone_from(struct mdspan_t *dest,
                                     const struct mdspan_t *src) {
  memcpy(dest->dimensions, src->dimensions, sizeof(dim_t));
  dest->buffer = malloc(MD_CM_BUFLEN(*src));
  dest->stride = src->stride;
  memcpy(dest->buffer, src->buffer, MD_CM_BUFLEN(*src));
}

static inline void mdspan_search(struct mdspan_t map, idx_t *x, idx_t *y,
                                 char c) {
  for_all_points(*x, *y, map.dimensions) {
    if (MD_CM_GET(*x, *y, map) == c) {
      break;
    }
  }
}

#define INPUT_DIMS {130, 130}
#define INPUT_STRIDE 131

input_t parse_input() {
  input_t result = {.dimensions = INPUT_DIMS, .stride = INPUT_STRIDE};
  const size_t buffer_size = MD_CM_BUFLEN(result);
  result.buffer = malloc(buffer_size * sizeof(char));
  if (!(fread(result.buffer, sizeof(char), buffer_size, stdin) ||
        feof(stdin))) {
    err(ferror(stdin), NULL);
  }
  result.buffer[buffer_size - 1] = '\0';
  return result;
}

void free_input(input_t input) { free(input.buffer); }

#define TILE_EMPTY '.'
#define TILE_COP '^'
#define TILE_BLOCK '#'
#define TILE_EXPLORED 'X'

enum dir_t {
  DIR_UP = 0b0001,
  DIR_LEFT = 0b0010,
  DIR_DOWN = 0b0100,
  DIR_RIGHT = 0b1000,
};

static inline enum dir_t rotate_right(enum dir_t direction) {
  switch (direction) {
  case DIR_UP:
    return DIR_RIGHT;
  case DIR_LEFT:
    return DIR_UP;
  case DIR_DOWN:
    return DIR_LEFT;
  case DIR_RIGHT:
    return DIR_DOWN;
  }
}

static inline void step_towards(idx_t *x, idx_t *y, enum dir_t direction) {
  switch (direction) {
  case DIR_UP:
    (*y)--;
    break;
  case DIR_LEFT:
    (*x)--;
    break;
  case DIR_DOWN:
    (*y)++;
    break;
  case DIR_RIGHT:
    (*x)++;
    break;
  }
}

struct cursor_t {
  idx_t x, y;
  enum dir_t direction;
};

static inline bool step_once(struct cursor_t *cursor, struct mdspan_t map) {
  while (true) {
    idx_t x_next = cursor->x, y_next = cursor->y;
    step_towards(&x_next, &y_next, cursor->direction);
    if (!MD_IN(x_next, y_next, map.dimensions)) {
      return false;
    }
    if (MD_CM_GET(x_next, y_next, map) != TILE_BLOCK) {
      break;
    }
    cursor->direction = rotate_right(cursor->direction);
  }
  step_towards(&cursor->x, &cursor->y, cursor->direction);
  return true;
}

uint32_t part1(const input_t *input) {
  struct mdspan_t map;
  mdspan_clone_from(&map, input);

  struct cursor_t cursor = {.x = 0, .y = 0, .direction = DIR_UP};
  mdspan_search(map, &cursor.x, &cursor.y, TILE_COP);
  do {
    MD_CM_GET(cursor.x, cursor.y, map) = TILE_EXPLORED;
  } while (step_once(&cursor, map));

  uint32_t count = 0;
  idx_t x, y;
  for_all_points(x, y, map.dimensions) {
    count += MD_CM_GET(x, y, map) == TILE_EXPLORED;
  }

  free(map.buffer);
  return count;
}

bool find_loop(struct cursor_t cursor, struct mdspan_t map,
               struct mdspan_t directions) {
  while (step_once(&cursor, map)) {
    if (MD_CM_GET(cursor.x, cursor.y, directions) & cursor.direction) {
      return true;
    }
    MD_CM_GET(cursor.x, cursor.y, directions) |= cursor.direction;
  }
  return false;
}

uint32_t part2(const input_t *input) {
  struct mdspan_t map,
      directions = {.dimensions = INPUT_DIMS, .stride = INPUT_STRIDE - 1};
  mdspan_clone_from(&map, input);
  memcpy(directions.dimensions, input->dimensions, sizeof(dim_t));
  directions.buffer = malloc(MD_CM_BUFLEN(directions));

  struct cursor_t start = {.x = 0, .y = 0, .direction = DIR_UP};
  mdspan_search(map, &start.x, &start.y, TILE_COP);

  struct cursor_t current = start;
  do {
    MD_CM_GET(current.x, current.y, map) = TILE_EXPLORED;
  } while (step_once(&current, map));

  MD_CM_GET(start.x, start.y, map) = TILE_COP;
  uint32_t count = 0;
  idx_t x, y;
  for_all_points(x, y, map.dimensions) {
    if (MD_CM_GET(x, y, map) != TILE_EXPLORED) {
      continue;
    }
    MD_CM_GET(x, y, map) = TILE_BLOCK;
    memset(directions.buffer, 0, MD_CM_BUFLEN(directions));
    if (find_loop(start, map, directions)) {
      MD_CM_GET(x, y, map) = TILE_EXPLORED;
      count++;
    } else {
      MD_CM_GET(x, y, map) = TILE_EMPTY;
    }
  }
  free(map.buffer);
  free(directions.buffer);
  return count;
}

int main() {
  input_t input = parse_input();
  printf("Part 1: %i\n", part1(&input));
  printf("Part 2: %i\n", part2(&input));
  free_input(input);
}
