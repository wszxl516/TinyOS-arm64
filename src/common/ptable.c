#include "ptable.h"

int pr_table_title(u32 width) {
  width = width % 2 ? width + 1 : width;
  char title[128] = {0};
  for (u32 i = 0; i < width; i++) title[i] = '-';
  return pr_debug("%s\n", title);
}

int pr_table_blank(u32 width) {
  width = width % 2 ? width + 1 : width;
  char title[128] = {0};
  for (u32 i = 0; i < width; i++) title[i] = ' ';
  return pr_debug("%s\n", title);
}

int pr_table_end(u32 width) {
  pr_table_title(width);
  return pr_table_blank(width);
}

int pr_table(const char *fmt, u32 width, ...) {
  va_list ap;
  int retval;
  char buffer[1024];
  va_start(ap, width);
  retval = k_vsprintf(buffer, fmt, ap);
  va_end(ap);

  width = width % 2 ? width + 1 : width;
  u32 str_len = strlen(buffer);
  u32 l_align_width = (width - str_len) / 2;
  u32 r_align_width = str_len % 2 ? l_align_width + 1 : l_align_width;
  char title[128] = {0};
  char l_blank[64] = {0};
  char r_blank[64] = {0};
  for (u32 i = 0; i < width; i++) title[i] = '-';
  for (u32 i = 0; i < l_align_width - 1; i++) l_blank[i] = ' ';
  for (u32 i = 0; i < r_align_width - 1; i++) r_blank[i] = ' ';
  retval = pr_debug("%s\n|%s%s%s|\n", title, l_blank, buffer, r_blank);
  return retval;
}