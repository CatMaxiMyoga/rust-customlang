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
