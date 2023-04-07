#ifndef __PTABLE_H__
#define __PTABLE_H__
#include "printf.h"

int pr_table(const char *fmt, u32 width, ...);
int pr_table_title(u32 width);
int pr_table_blank(u32 width);
int pr_table_end(u32 width);

#endif  //__PTABLE_H__