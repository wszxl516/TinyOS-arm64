#ifndef __SYMBOL_H__
#define __SYMBOL_H__
#include "common.h"
#include "config.h"

extern ld_script_pointer_t __start_symbol_table, __stop_symbol_table;

typedef struct{
    unsigned long addr;
    unsigned long size;
    char type;
    const char *symbol_name;
}symbols_entry_t;

#define ABS(x) (((x) >= 0)? (x): -(x))
symbols_entry_t *find_symbol(symbols_entry_t *symbol_res);
void lookup_name_and_offset(usize addr, char *symbol_name, usize *offset);
#define UNKNOWN_SYMBOL      ("Unknown")
#endif //__SYMBOL_H__