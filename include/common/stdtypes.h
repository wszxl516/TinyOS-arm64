#ifndef __STD_TYPES__
#define __STD_TYPES__

typedef unsigned char u8;
typedef char i8;

typedef unsigned short int u16;
typedef short int i16;

typedef unsigned int u32;
typedef int i32;

typedef unsigned long long u64;
typedef long long i64;

typedef unsigned long usize;
typedef long isize;

typedef u32 bool;
#define true 1
#define false 0
#define NULL ((void*)0)
typedef u8 _Reserved;

#define USIZE_MAX (~(usize)0)
#define ISIZE_MAX ((usize)(USIZE_MAX >> 1))

#define U8_MAX ((u8)~0U)
#define I8_MAX ((i8)(U8_MAX >> 1))
#define I8_MIN ((i8)(-I8_MAX - 1))
#define U16_MAX ((u16)~0U)
#define I16_MAX ((i16)(U16_MAX >> 1))
#define I16_MIN ((i16)(-I16_MAX - 1))
#define U32_MAX ((u32)~0U)
#define U32_MIN ((u32)0)
#define I32_MAX ((i32)(U32_MAX >> 1))
#define I32_MIN ((i32)(-I32_MAX - 1))
#define U64_MAX ((u64)~0ULL)
#define I64_MAX ((i64)(U64_MAX >> 1))
#endif  //__STD_TYPES__