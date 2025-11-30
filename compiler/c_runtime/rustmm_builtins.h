#ifndef RUSTMM_BUILTINS_H
#define RUSTMM_BUILTINS_H

#include <stdbool.h>

typedef struct {
  char data[512];
  unsigned short len;
} rustmm_type_string;

void rustmm_builtin_print(rustmm_type_string s);
void rustmm_builtin_println(rustmm_type_string s);

rustmm_type_string rustmm_builtin_boolToString(bool b);
rustmm_type_string rustmm_builtin_intToString(int i);
rustmm_type_string rustmm_builtin_floatToString(double f);

#endif
