#ifndef __MEM_OP__
#define __MEM_OP__
#include "stdtypes.h"

void *memset(void *dest, int c, usize n);

void *memcpy(void *restrict dest, const void *restrict src, usize n);

int memcmp(const void *vl, const void *vr, usize n);

void *memchr (const void *, int, usize);

usize strlen(const char *s);

char *stpncpy(char *restrict d, const char *restrict s, usize n);

char *strncat(char *restrict d, const char *restrict s, usize n);

int strncmp(const char *_l, const char *_r, usize n);
#endif //__MEM_OP__