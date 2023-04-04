#include "symbol.h"

#include "address.h"
#include "config.h"
#include "printf.h"
#include "stdtypes.h"
#include "string.h"

static unsigned long symbol_addr_offset = 0;

static symbols_entry_t *_find_symbol_by_name(symbols_entry_t *symbol_res) {
  symbols_entry_t *symbol_tables = (symbols_entry_t *)__start_symbol_table;
  while (symbol_tables < (symbols_entry_t *)__stop_symbol_table) {
    if (!strncmp(symbol_res->symbol_name, symbol_tables->symbol_name,
                 strlen(symbol_tables->symbol_name))) {
      symbol_res->addr = symbol_tables->addr + symbol_addr_offset;
      symbol_res->size = symbol_tables->size;
      break;
    }
    symbol_tables++;
  }
  return symbol_res;
}
static inline void __init_symbol() {
  symbols_entry_t symbol = {.symbol_name = "_find_symbol_by_name", .addr = 0};
  if (symbol_addr_offset == 0) {
    _find_symbol_by_name(&symbol);
    if (symbol.addr != 0)
      symbol_addr_offset =
          (unsigned long)_find_symbol_by_name - (unsigned long)symbol.addr;
  }
}

symbols_entry_t *_find_symbol_by_addr(symbols_entry_t *symbol_res) {
  symbols_entry_t *symbol_entry = (symbols_entry_t *)__start_symbol_table;
  while (symbol_entry <= (symbols_entry_t *)__stop_symbol_table) {
    if (symbol_res->addr >= (symbol_entry->addr + symbol_addr_offset) &&
        symbol_res->addr <= ((symbol_entry + 1)->addr + symbol_addr_offset)) {
      if (symbol_res->type != 0 && symbol_res->type != symbol_entry->type &&
          ABS(symbol_res->type - symbol_entry->type) != 32)
        break;
      strncpy((char *)symbol_res->symbol_name, symbol_entry->symbol_name,
              strlen(symbol_entry->symbol_name));
      symbol_res->size = symbol_entry->size;
      symbol_res->addr = symbol_entry->addr;
      break;
    }
    symbol_entry++;
  }
  return symbol_res;
}

symbols_entry_t *find_symbol(symbols_entry_t *symbol_res) {
  __init_symbol();
  if (symbol_res->addr != 0) return _find_symbol_by_addr(symbol_res);

  if (symbol_res->symbol_name != NULL) return _find_symbol_by_name(symbol_res);
  return NULL;
}

void lookup_name_and_offset(usize addr, char *symbol_name, usize *offset) {
  symbols_entry_t entry = {.addr = VIR_2_PHY(addr), .symbol_name = symbol_name, .type = 'T'};
  memset(symbol_name, 0, 64);
  find_symbol(&entry);
  if (entry.symbol_name[0] == 0) {
    strncpy(symbol_name, UNKNOWN_SYMBOL, sizeof(UNKNOWN_SYMBOL));
    *offset = 0;
  }
  else
    *offset = VIR_2_PHY(addr) - entry.addr - symbol_addr_offset;
}