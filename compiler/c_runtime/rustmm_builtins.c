#include "rustmm_builtins.h"
#include "rustmm_internals.h"
#include <stdbool.h>
#include <stdio.h>

void rustmm_builtin_print(rustmm_type_string s) {
  printf("%.*s", s.len, s.data);
}
void rustmm_builtin_println(rustmm_type_string s) {
  printf("%.*s\n", s.len, s.data);
}

rustmm_type_string rustmm_builtin_boolToString(bool b) {
  return rustmm_internal_make_string(b ? "true" : "false");
}
rustmm_type_string rustmm_builtin_intToString(int i) {
  static char buffer[32];
  snprintf(buffer, sizeof(buffer), "%d", i);
  return rustmm_internal_make_string(buffer);
}
rustmm_type_string rustmm_builtin_floatToString(double f) {
  static char buffer[32];
  snprintf(buffer, sizeof(buffer), "%f", f);
  return rustmm_internal_make_string(buffer);
}

bool rustmm_builtin_stringToBool(rustmm_type_string s) {
  return (s.len > 0);
}
bool rustmm_builtin_intToBool(int i) {
  return (i != 0);
}
bool rustmm_builtin_floatToBool(double f) {
  return (f != 0.0);
}

int rustmm_builtin_stringToInt(rustmm_type_string s) {
  int result = 0;
  sscanf(s.data, "%d", &result);
  return result;
}
int rustmm_builtin_boolToInt(bool b) {
  return b ? 1 : 0;
}
int rustmm_builtin_floatToInt(double f) {
  return (int)f;
}

double rustmm_builtin_stringToFloat(rustmm_type_string s) {
  double result = 0.0;
  sscanf(s.data, "%lf", &result);
  return result;
}
double rustmm_builtin_boolToFloat(bool b) {
  return b ? 1.0 : 0.0;
}
double rustmm_builtin_intToFloat(int i) {
  return (double)i;
}
