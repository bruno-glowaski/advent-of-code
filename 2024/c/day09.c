#include "aoc.h"

#include <err.h>
#include <math.h>
#include <string.h>

typedef const char *input_t;

#define INPUT_BUFLEN 20000

input_t parse_input() {
  static char input_buffer[INPUT_BUFLEN] = {0};
  if (!(fread(input_buffer, sizeof(char), INPUT_BUFLEN - 1, stdin) ||
        feof(stdin))) {
    err(ferror(stdin), NULL);
  }
  return input_buffer;
}

static inline size_t min(size_t a, size_t b) { return a < b ? a : b; }

typedef uint16_t block_id_t;

#define DISK_BUFLEN (INPUT_BUFLEN * 9)

uint64_t part1(input_t input) {
  size_t input_len = strlen(input);
  size_t last_id = (input_len - 1) / 2;

#define READ_FILE(_x) (input[2 * (_x)] - '0')
#define READ_FREE(_x) (input[2 * (_x) + 1] - '0')

  static block_id_t disk[DISK_BUFLEN];
  size_t disk_len = 0;

  block_id_t start = 0, end = last_id;
  size_t remainder = READ_FILE(end);
  while (start < end) {
    size_t fill_count = READ_FILE(start);
    for (size_t i = 0; i < fill_count; i++) {
      disk[disk_len + i] = start;
    }
    disk_len += fill_count;

    size_t free_space_count = READ_FREE(start);
    while (free_space_count > 0) {
      fill_count = min(remainder, free_space_count);
      for (size_t i = 0; i < fill_count; i++) {
        disk[disk_len + i] = end;
      }
      disk_len += fill_count;
      remainder -= fill_count;
      if (remainder == 0) {
        end--;
        if (end <= start) {
          break;
        }
        remainder = READ_FILE(end);
      }
      free_space_count -= fill_count;
    }

    start++;
  }
  for (size_t i = 0; i < remainder; i++) {
    disk[disk_len + i] = end;
  }
  disk_len += remainder;

  uint64_t checksum = 0;
  for (size_t i = 0; i < disk_len; i++) {
    checksum += disk[i] * i;
  }
  return checksum;
}

struct file_t {
  uint8_t length;
  uint8_t free_ahead;
  block_id_t prev;
  block_id_t next;
};

#define DISK_MAP_BUFLEN 10000

uint64_t part2(input_t input) {
  size_t input_len = strlen(input);
  size_t last_id = (input_len - 1) / 2;

  static struct file_t disk_map[DISK_MAP_BUFLEN] = {0};
  for (size_t i = 0; i < last_id; i++) {
    disk_map[i].next = i + 1;
    disk_map[i].length = input[2 * i] - '0';
    disk_map[i].free_ahead = input[2 * i + 1] - '0';
    disk_map[i + 1].prev = i;
  }
  disk_map[0].prev = last_id;
  disk_map[last_id].length = input[input_len - 1] - '0';

  for (block_id_t current = last_id; current > 1; current--) {
    block_id_t old_prev = disk_map[current].prev,
               old_next = disk_map[current].next;
    for (block_id_t new_prev = 0; new_prev != current;
         new_prev = disk_map[new_prev].next) {
      if (disk_map[current].length > disk_map[new_prev].free_ahead) {
        continue;
      }
      block_id_t new_next = disk_map[new_prev].next;

      disk_map[old_prev].free_ahead +=
          disk_map[current].length + disk_map[current].free_ahead;
      disk_map[current].free_ahead =
          disk_map[new_prev].free_ahead - disk_map[current].length;
      disk_map[new_prev].free_ahead = 0;

      if (old_prev == new_prev) {
        break;
      }

      disk_map[current].next = new_next;
      disk_map[current].prev = new_prev;

      disk_map[old_prev].next = old_next;
      disk_map[old_next].prev = old_prev;

      disk_map[new_next].prev = current;
      disk_map[new_prev].next = current;
      break;
    }
  }

  uint64_t checksum = 0;
  uint64_t offset = 0;
  const block_id_t start = 0;
  block_id_t i = start;
  do {
    struct file_t file = disk_map[i];
    checksum += i * (2 * offset + file.length - 1) * (file.length) / 2;
    offset += disk_map[i].length + disk_map[i].free_ahead;
    i = file.next;
  } while (i != start);
  return checksum;
}

UTEST(day06, part1, {
  const char *input = "2333133121414131402";
  RX_INT_REQUIRE_EQUAL(part1(input), 1928);
})

UTEST(day06, part2, {
  const char *input = "2333133121414131402";
  RX_INT_REQUIRE_EQUAL(part2(input), 2858);
})

DECLARE_AOC_RUNNER()
