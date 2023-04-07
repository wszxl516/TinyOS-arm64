#include "frame.h"

#include "address.h"
#include "spinlock.h"
#include "stdtypes.h"

STATIC_INIT_SPIN_LOCK(allocator_lock);
static allocator_t ALLOCATOR;

void frma_alloc_init() {
  ALLOCATOR.start_addr = PHY_2_VIR(FRAME_START);
  ALLOCATOR.end_addr = PHY_2_VIR(FRAME_END);
  ALLOCATOR.current_page = 0;
  ALLOCATOR.lock = allocator_lock;
}

void *frame_alloc_page() {
  usize page_nums = PAGE_NUM(ALLOCATOR);
  for (u32 i = ALLOCATOR.current_page; i < page_nums; i++) {
    if (!ALLOCATOR.used_map[i]) {
      spin_lock(&ALLOCATOR.lock);
      ALLOCATOR.used_map[i] = 1;
      spin_unlock(&ALLOCATOR.lock);
      return (void *)(ALLOCATOR.start_addr + (i * PAGE_SIZE));
    }
  }
  return NULL;
}

bool frame_free_page(void *addr) {
  u32 page_num = ((usize)addr - ALLOCATOR.start_addr) / PAGE_SIZE;
  if (page_num > FRAME_PAGE_NUM) {
    return false;
  }
  spin_lock(&ALLOCATOR.lock);
  ALLOCATOR.used_map[page_num] = 0;
  spin_unlock(&ALLOCATOR.lock);
  return true;
}