#include "stdint.h"
#include "threads.h"

thread_local uint32_t COUNTER = 0;

uint32_t* get_thread_local() {
  return &COUNTER;
}
