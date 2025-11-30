#include "rustmm_builtins.h"
#include <string.h>

rustmm_type_string rustmm_internal_make_string(const char* src) {
  rustmm_type_string s;

  size_t len = strlen(src);
  if (len > 511) {
    len = 511;
  }

  memcpy(s.data, src, len);
  s.data[len] = '\0';
  s.len = len;

  return s;
}
