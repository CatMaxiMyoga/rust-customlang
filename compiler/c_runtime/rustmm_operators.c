#include "rustmm_builtins.h"
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

/* ADDITION */
int rustmm_operator_add_int_int(int a, int b) { return a + b; }
double rustmm_operator_add_int_float(int a, double b) { return a + b; }
double rustmm_operator_add_float_float(double a, double b) { return a + b; }
double rustmm_operator_add_float_int(double a, int b) { return a + b; }
rustmm_type_string rustmm_operator_add_string_string(rustmm_type_string a, rustmm_type_string b) {
  rustmm_type_string result;

  size_t total_len = a.len + b.len;
  if (total_len > 511) {
    total_len = 511;
  }

  memcpy(result.data, a.data, a.len);
  memcpy(result.data + a.len, b.data, total_len - a.len);
  result.data[total_len] = '\0';
  result.len = total_len;

  return result;
}

/* SUBTRACTION */
int rustmm_operator_sub_int_int(int a, int b) { return a - b; }
double rustmm_operator_sub_int_float(int a, double b) { return a - b; }
double rustmm_operator_sub_float_float(double a, double b) { return a - b; }
double rustmm_operator_sub_float_int(double a, int b) { return a - b; }

/* MULTIPLICATION */

int rustmm_operator_mul_int_int(int a, int b) { return a * b; }
double rustmm_operator_mul_int_float(int a, double b) { return a * b; }
double rustmm_operator_mul_float_float(double a, double b) { return a * b; }
double rustmm_operator_mul_float_int(double a, int b) { return a * b; }

/* DIVISION */
int rustmm_operator_div_int_int(int a, int b) { return a / b; }
double rustmm_operator_div_int_float(int a, double b) { return a / b; }
double rustmm_operator_div_float_float(double a, double b) { return a / b; }
double rustmm_operator_div_float_int(double a, int b) { return a / b; }

/* EQUAL TO */
bool rustmm_operator_eq_int_int(int a, int b) { return a == b; }
bool rustmm_operator_eq_int_float(int a, double b) { return a == b; }
bool rustmm_operator_eq_float_float(double a, double b) { return a == b; }
bool rustmm_operator_eq_float_int(double a, int b) { return a == b; }
bool rustmm_operator_eq_bool_bool(bool a, bool b) { return a == b; }
bool rustmm_operator_eq_string_string(const char* a, const char* b) {
  if (a == NULL && b == NULL) return true;
  if (a == NULL || b == NULL) return false;
  return strcmp(a, b) == 0;
}

/* NOT EQUAL TO */
bool rustmm_operator_ne_int_int(int a, int b) { return a != b; }
bool rustmm_operator_ne_int_float(int a, double b) { return a != b; }
bool rustmm_operator_ne_float_float(double a, double b) { return a != b; }
bool rustmm_operator_ne_float_int(double a, int b) { return a != b; }
bool rustmm_operator_ne_bool_bool(bool a, bool b) { return a != b; }
bool rustmm_operator_ne_string_string(const char* a, const char* b) {
  if (a == NULL && b == NULL) return false;
  if (a == NULL || b == NULL) return true;
  return strcmp(a, b) != 0;
}

/* GREATER THAN */
bool rustmm_operator_gt_int_int(int a, int b) { return a > b; }
bool rustmm_operator_gt_int_float(int a, double b) { return a > b; }
bool rustmm_operator_gt_float_float(double a, double b) { return a > b; }
bool rustmm_operator_gt_float_int(double a, int b) { return a > b; }

/* LESS THAN */
bool rustmm_operator_lt_int_int(int a, int b) { return a < b; }
bool rustmm_operator_lt_int_float(int a, double b) { return a < b; }
bool rustmm_operator_lt_float_float(double a, double b) { return a < b; }
bool rustmm_operator_lt_float_int(double a, int b) { return a < b; }

/* GREATER THAN OR EQUAL TO */
bool rustmm_operator_ge_int_int(int a, int b) { return a >= b; }
bool rustmm_operator_ge_int_float(int a, double b) { return a >= b; }
bool rustmm_operator_ge_float_float(double a, double b) { return a >= b; }
bool rustmm_operator_ge_float_int(double a, int b) { return a >= b; }

/* LESS THAN OR EQUAL TO */
bool rustmm_operator_le_int_int(int a, int b) { return a <= b; }
bool rustmm_operator_le_int_float(int a, double b) { return a <= b; }
bool rustmm_operator_le_float_float(double a, double b) { return a <= b; }
bool rustmm_operator_le_float_int(double a, int b) { return a <= b; }
