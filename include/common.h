#ifndef __COMMON_H__
#define __COMMON_H__
#include "stdtypes.h"
#define NO_OPTIMIZATION_ALIGN    __attribute__ ((packed))
#define OPTIMIZATION_ALIGN(n)    __attribute__ ((aligned(n)))
#define SECTION(n)               __attribute__ ((section(n)))
#define __USED__                 __attribute__((used))
#define __UNUSED__               __attribute__((unused))
#define FUNC_NORETURN            __attribute__((__noreturn__))


#define REG volatile
#define REG_WRITE(addr, type, value)      ((*(REG type*)(addr)) = value)
#define REG_READ(addr, type)       (*(REG type*)(addr))
#define REG_WRITE32(addr, value) 	REG_WRITE(addr, u32, value)
#define REG_READ32(addr) 	        REG_READ(addr, u32)
#define GET_BIT(data, bit_sht) ((data >> bit_sht)&1)

#define OFFSET_OF(TYPE, MEMBER) ((usize) &((TYPE*)0)->MEMBER)
#define CONTAINER_OF(ptr, type, member) ({ \
        const typeof( ((type *)0)->member ) *__mptr = \
        (const typeof( ((type *)0)->member ) *)(ptr); \
        (type *)( (char *)__mptr - OFFSET_OF(type, member) ); \
        })
        
#endif //__COMMON_H__