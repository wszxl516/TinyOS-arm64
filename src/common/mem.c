#include "string.h"

void *memset(void *dest, int c, usize n) {
  u8 *s = (u8 *)dest;
  for (; n; n--, s++) *s = c;
  return dest;
}

void *memcpy(void *restrict dest, const void *restrict src, usize n) {
  u8 *d = dest;
  const u8 *s = src;

  for (; n; n--) *d++ = *s++;
  return dest;
}

int memcmp(const void *vl, const void *vr, usize n) {
  const u8 *l = vl, *r = vr;
  for (; n && *l == *r; n--, l++, r++)
    ;
  return n ? *l - *r : 0;
}

void *memchr(const void *src, i32 c, usize n) {
  const u8 *s = src;
  c = (u8)c;
  for (; n && *s != c; s++, n--)
    ;
  return n ? (void *)s : 0;
}