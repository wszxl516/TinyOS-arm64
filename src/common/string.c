#include "string.h"


usize strlen(const char *s)
{
	const char *a = s;
	for (; *s; s++);
	return s-a;
}

char *stpncpy(char *restrict d, const char *restrict s, usize n)
{
	for (; n && (*d=*s); n--, s++, d++);
	memset(d, 0, n);
	return d;
}

char *strncat(char *restrict d, const char *restrict s, usize n)
{
	char *a = d;
	d += strlen(d);
	while (n && *s) n--, *d++ = *s++;
	*d++ = 0;
	return a;
}

int strncmp(const char *_l, const char *_r, usize n)
{
	const u8 *l=(void *)_l, *r=(void *)_r;
	if (!n--) return 0;
	for (; *l && *r && n && *l == *r ; l++, r++, n--);
	return *l - *r;
}
