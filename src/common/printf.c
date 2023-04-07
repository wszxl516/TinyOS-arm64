#include "printf.h"

#include <string.h>

#include "stdtypes.h"
#include "uart.h"
#ifdef __GNUC__
#define _k_GCC_NO_INLINE_ __attribute__((noinline))
#else
#define _k_GCC_NO_INLINE_
#endif

/*
 * Implementation
 */
struct param {
  char lz : 1;  /**<  Leading zeros */
  char alt : 1; /**<  alternate form */
  char uc : 1;  /**<  Upper case (for base16 only) */
  char prefix : 1;
  char align_left : 1; /**<  0 == align right (default), 1 == align left */
  u32 width;           /**<  field width */
  char sign;           /**<  The sign to display (if any) */
  u32 base;            /**<  number base (e.g.: 8, 10, 16) */
  char *bf;            /**<  Buffer to output */
};

static void _k_GCC_NO_INLINE_ ulli2a(u64 num, struct param *p) {
  int n = 0;
  u64 d = 1;
  char *bf = p->bf;
  while (num / d >= p->base) d *= p->base;
  while (d != 0) {
    int dgt = num / d;
    num %= d;
    d /= p->base;
    if (n || dgt > 0 || d == 0) {
      *bf++ = dgt + (dgt < 10 ? '0' : (p->uc ? 'A' : 'a') - 10);
      ++n;
    }
  }
  *bf = 0;
}

static void lli2a(i64 num, struct param *p) {
  if (num < 0) {
    num = -num;
    p->sign = '-';
  }
  ulli2a(num, p);
}

static void uli2a(usize num, struct param *p) {
  int n = 0;
  usize d = 1;
  char *bf = p->bf;
  while (num / d >= p->base) d *= p->base;
  while (d != 0) {
    int dgt = num / d;
    num %= d;
    d /= p->base;
    if (n || dgt > 0 || d == 0) {
      *bf++ = dgt + (dgt < 10 ? '0' : (p->uc ? 'A' : 'a') - 10);
      ++n;
    }
  }
  *bf = 0;
}

static void li2a(isize num, struct param *p) {
  if (num < 0) {
    num = -num;
    p->sign = '-';
  }
  uli2a(num, p);
}

static void ui2a(u32 num, struct param *p) {
  int n = 0;
  u32 d = 1;
  char *bf = p->bf;
  while (num / d >= p->base) d *= p->base;
  while (d != 0) {
    int dgt = num / d;
    num %= d;
    d /= p->base;
    if (n || dgt > 0 || d == 0) {
      *bf++ = dgt + (dgt < 10 ? '0' : (p->uc ? 'A' : 'a') - 10);
      ++n;
    }
  }
  *bf = 0;
}

static void i2a(int num, struct param *p) {
  if (num < 0) {
    num = -num;
    p->sign = '-';
  }
  ui2a(num, p);
}

static int a2d(char ch) {
  if (ch >= '0' && ch <= '9')
    return ch - '0';
  else if (ch >= 'a' && ch <= 'f')
    return ch - 'a' + 10;
  else if (ch >= 'A' && ch <= 'F')
    return ch - 'A' + 10;
  else
    return -1;
}

static char a2u(char ch, const char **src, int base, u32 *nump) {
  const char *p = *src;
  u32 num = 0;
  int digit;
  while ((digit = a2d(ch)) >= 0) {
    if (digit > base) break;
    num = num * base + digit;
    ch = *p++;
  }
  *src = p;
  *nump = num;
  return ch;
}

static void putchw(void *putp, putcf putf, struct param *p) {
  char ch;
  int n = p->width;
  char *bf = p->bf;

  /* Number of filling characters */
  while (*bf++ && n > 0) n--;
  if (p->sign) n--;
  if (p->alt && p->base == 16)
    n -= 2;
  else if (p->alt && p->base == 8)
    n--;

  /* Fill with space to align to the right, before alternate or sign */
  if (!p->lz && !p->align_left) {
    while (n-- > 0) putf(putp, ' ');
  }

  /* print sign */
  if (p->sign) putf(putp, p->sign);

  /* Alternate */
  if (p->alt && p->base == 16 && p->prefix) {
    putf(putp, '0');
    putf(putp, (p->uc ? 'X' : 'x'));
  } else if (p->alt && p->base == 8) {
    putf(putp, '0');
  }

  /* Fill with zeros, after alternate or sign */
  if (p->lz) {
    while (n-- > 0) putf(putp, '0');
  }

  /* Put actual buffer */
  bf = p->bf;
  while ((ch = *bf++)) putf(putp, ch);

  /* Fill with space to align to the left, after string */
  if (!p->lz && p->align_left) {
    while (n-- > 0) putf(putp, ' ');
  }
}

void k_format(void *putp, putcf putf, const char *fmt, va_list va) {
  struct param p;
  char bf[23]; /* long = 64b on some architectures */
  char ch;
  p.bf = bf;

  while ((ch = *(fmt++))) {
    if (ch != '%') {
      putf(putp, ch);
    } else {
      char lng = 0;
      /* 1 for long, 2 for long long */
      /* Init parameter struct */
      p.lz = 0;
      p.alt = 0;
      p.width = 0;
      p.align_left = 0;
      p.sign = 0;

      /* Flags */
      while ((ch = *(fmt++))) {
        switch (ch) {
          case '-':
            p.align_left = 1;
            continue;
          case '0':
            p.lz = 1;
            continue;
          case '#':
            p.alt = 1;
            continue;
          default:
            break;
        }
        break;
      }

      /* Width */
      if (ch >= '0' && ch <= '9') {
        ch = a2u(ch, &fmt, 10, &(p.width));
      }

      /* We accept 'x.y' format but don't support it completely:
       * we ignore the 'y' digit => this ignores 0-fill
       * size and makes it == width (ie. 'x') */
      if (ch == '.') {
        p.lz = 1;
        /* zero-padding */
        /* ignore actual 0-fill size: */
        do {
          ch = *(fmt++);
        } while ((ch >= '0') && (ch <= '9'));
      }

      if (ch == 'z') {
        ch = *(fmt++);
        if (sizeof(usize) == sizeof(u32))
          lng = 1;
        else if (sizeof(usize) == sizeof(u64))
          lng = 2;
      } else

          if (ch == 'l') {
        ch = *(fmt++);
        lng = 1;
        if (ch == 'l') {
          ch = *(fmt++);
          lng = 2;
        }
      }
      switch (ch) {
        case 0:
          goto abort;
        case 'u':
          p.base = 10;

          if (2 == lng)
            ulli2a(va_arg(va, u64), &p);
          else if (1 == lng)
            uli2a(va_arg(va, usize), &p);
          else
            ui2a(va_arg(va, u32), &p);
          putchw(putp, putf, &p);
          break;
        case 'f':
        case 'F':
          p.base = 10;
          double val = va_arg(va, double);
          lli2a((usize)val, &p);
          putchw(putp, putf, &p);
          p.width = 0;
          p.bf[0] = '.';
          p.bf[1] = 0;
          putchw(putp, putf, &p);
          lli2a((val - (double)((usize)val)) * 100, &p);
          putchw(putp, putf, &p);
          break;
        case 'd':
        case 'i':
          p.base = 10;
          if (2 == lng)
            lli2a(va_arg(va, i64), &p);
          else if (1 == lng)
            li2a(va_arg(va, isize), &p);
          else
            i2a(va_arg(va, i32), &p);
          putchw(putp, putf, &p);
          break;
        case 'P':
          /* fall through */
        case 'p':
          p.width = 18;
          p.alt = 1;
          lng = 2;
          /* fall through */
        case 'x':
        case 'X':
          p.base = 16;
          p.uc = (ch == 'X') ? 1 : 0;
          p.prefix = (ch == 'p' || ch == 'P') ? 1 : 0;
          p.align_left = 0;
          p.alt = 1;
          if (2 == lng)
            ulli2a(va_arg(va, u64), &p);
          else if (1 == lng)
            uli2a(va_arg(va, usize), &p);
          else
            ui2a(va_arg(va, u32), &p);
          putchw(putp, putf, &p);
          break;
        case 'o':
          p.base = 8;
          ui2a(va_arg(va, u32), &p);
          putchw(putp, putf, &p);
          break;
        case 'c':
          putf(putp, (char)(va_arg(va, i32)));
          break;
        case 's':
          p.bf = va_arg(va, char *);
          putchw(putp, putf, &p);
          p.bf = bf;
          break;
        case 'n':
          // repeat p.bf p.width times
          p.bf = va_arg(va, char *);
          for (u32 i = 0; i < p.width; i++) {
            char *tmp = p.bf;
            while (*tmp) {
              putf(putp, *tmp);
              tmp++;
            }
          }
          break;
        case '%':
          putf(putp, ch);
        default:
          break;
      }
    }
  }
abort:;
}

struct _vsnprintf_putcf_data {
  usize dest_capacity;
  char *dest;
  usize num_chars;
};

static void _vsnprintf_putcf(void *p, char c) {
  struct _vsnprintf_putcf_data *data = (struct _vsnprintf_putcf_data *)p;
  if (data->num_chars < data->dest_capacity) data->dest[data->num_chars] = c;
  data->num_chars++;
}

int k_vsnprintf(char *str, usize size, const char *format, va_list ap) {
  struct _vsnprintf_putcf_data data;

  if (size < 1) return 0;

  data.dest = str;
  data.dest_capacity = size - 1;
  data.num_chars = 0;
  k_format(&data, _vsnprintf_putcf, format, ap);

  if (data.num_chars < data.dest_capacity)
    data.dest[data.num_chars] = '\0';
  else
    data.dest[data.dest_capacity] = '\0';

  return data.num_chars;
}

int k_snprintf(char *str, usize size, const char *format, ...) {
  va_list ap;
  int retval;

  va_start(ap, format);
  retval = k_vsnprintf(str, size, format, ap);
  va_end(ap);
  return retval;
}

struct _vsprintf_putcf_data {
  char *dest;
  usize num_chars;
};

static void _vsprintf_putcf(void *p, char c) {
  struct _vsprintf_putcf_data *data = (struct _vsprintf_putcf_data *)p;
  data->dest[data->num_chars++] = c;
}

int k_vsprintf(char *str, const char *format, va_list ap) {
  struct _vsprintf_putcf_data data;
  data.dest = str;
  data.num_chars = 0;
  k_format(&data, _vsprintf_putcf, format, ap);
  data.dest[data.num_chars] = '\0';
  return data.num_chars;
}

int k_sprintf(char *str, const char *format, ...) {
  va_list ap;
  int retval;

  va_start(ap, format);
  retval = k_vsprintf(str, format, ap);
  va_end(ap);
  return retval;
}

int k_printf(const char *fmt, ...) {
  va_list ap;
  int retval;
  char buffer[1024];
  va_start(ap, fmt);
  retval = k_vsprintf(buffer, fmt, ap);
  va_end(ap);
  puts(buffer);
  return retval;
}
