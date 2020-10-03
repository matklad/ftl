#define _POSIX_C_SOURCE 200809L

#include "inttypes.h"
#include "stdint.h"
#include "stdio.h"
#include "threads.h"
#include "time.h"

thread_local uint32_t COUNTER = 0;

const uint32_t STEPS = 1000000000;

uint32_t sum_c() {
  for (uint32_t step = 0; step < STEPS; step++) {
    uint32_t inc = (step * step) ^ step;
    COUNTER += inc;
  }
  return COUNTER;
}

uint64_t now_ms() {
  struct timespec spec;
  clock_gettime(CLOCK_MONOTONIC, &spec);
  return spec.tv_sec * 1000 + spec.tv_nsec / 1000000;
}

int main(void) {
  uint64_t t = now_ms();
  uint32_t r = sum_c();
  printf("C:               %" PRIu32 " %"PRIu64"ms\n", r, now_ms() - t);
  return 0;
}
