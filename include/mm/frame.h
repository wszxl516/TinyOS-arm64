#ifndef __FRAME_H__
#define __FRAME_H__
#include "address.h"
#include "common.h"
#include "config.h"
#include "stdtypes.h"
#include "spinlock.h"

#define FRAME_START         ((usize)frame_start)
#define FRAME_END           ((usize)frame_end)
#define FRAME_PAGE_NUM      (PAGE_FRAME_SIZE / PAGE_SIZE)


typedef struct OPTIMIZATION_ALIGN (8)
{
  u8 used_map[FRAME_PAGE_NUM];
  vir_addr_t start_addr;
  vir_addr_t end_addr;
  u32 current_page;
  spinlock_t lock;

} allocator_t;


#define PAGE_NUM(alloc)     ((alloc.end_addr - alloc.start_addr) / PAGE_SIZE)


void frma_alloc_init();
void *frame_alloc_page();
bool frame_free_page(void *addr);
#endif //__FRAME_H__