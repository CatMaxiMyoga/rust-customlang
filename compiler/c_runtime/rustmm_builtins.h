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

bool rustmm_builtin_stringToBool(rustmm_type_string s);
bool rustmm_builtin_intToBool(int i);
bool rustmm_builtin_floatToBool(double f);

int rustmm_builtin_stringToInt(rustmm_type_string s);
int rustmm_builtin_boolToInt(bool b);
int rustmm_builtin_floatToInt(double f);

double rustmm_builtin_stringToFloat(rustmm_type_string s);
double rustmm_builtin_boolToFloat(bool b);
double rustmm_builtin_intToFloat(int i);

#endif
