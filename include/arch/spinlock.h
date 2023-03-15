#ifndef __SPIN_LOCK__
#define __SPIN_LOCK__
#include "stdtypes.h"
#include "common.h"

typedef struct OPTIMIZATION_ALIGN(4)
{
    u32 lock;
} spinlock_t;

#define STATIC_INIT_SPIN_LOCK(name) static spinlock_t name = {.lock = 0}

extern void spin_lock(spinlock_t *lock);
extern void spin_unlock(spinlock_t *lock);
#endif //__SPIN_LOCK__