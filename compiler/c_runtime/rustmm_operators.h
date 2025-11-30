#ifndef RUSTMM_OPERATORS_H
#define RUSTMM_OPERATORS_H

#include "rustmm_builtins.h"
#include <stdbool.h>

int rustmm_operator_add_int_int(int a, int b);
double rustmm_operator_add_int_float(int a, double b);
double rustmm_operator_add_float_float(double a, double b);
double rustmm_operator_add_float_int(double a, int b);
rustmm_type_string rustmm_operator_add_string_string(rustmm_type_string a,
                                                     rustmm_type_string b);

int rustmm_operator_sub_int_int(int a, int b);
double rustmm_operator_sub_int_float(int a, double b);
double rustmm_operator_sub_float_float(double a, double b);
double rustmm_operator_sub_float_int(double a, int b);

int rustmm_operator_mul_int_int(int a, int b);
double rustmm_operator_mul_int_float(int a, double b);
double rustmm_operator_mul_float_float(double a, double b);
double rustmm_operator_mul_float_int(double a, int b);

int rustmm_operator_div_int_int(int a, int b);
double rustmm_operator_div_int_float(int a, double b);
double rustmm_operator_div_float_float(double a, double b);
double rustmm_operator_div_float_int(double a, int b);

bool rustmm_operator_eq_int_int(int a, int b);
bool rustmm_operator_eq_int_float(int a, double b);
bool rustmm_operator_eq_float_float(double a, double b);
bool rustmm_operator_eq_float_int(double a, int b);
bool rustmm_operator_eq_bool_bool(bool a, bool b);
bool rustmm_operator_eq_string_string(rustmm_type_string a,
                                      rustmm_type_string b);

bool rustmm_operator_ne_int_int(int a, int b);
bool rustmm_operator_ne_int_float(int a, double b);
bool rustmm_operator_ne_float_float(double a, double b);
bool rustmm_operator_ne_float_int(double a, int b);
bool rustmm_operator_ne_bool_bool(bool a, bool b);
bool rustmm_operator_ne_string_string(rustmm_type_string a,
                                      rustmm_type_string b);

bool rustmm_operator_gt_int_int(int a, int b);
bool rustmm_operator_gt_int_float(int a, double b);
bool rustmm_operator_gt_float_float(double a, double b);
bool rustmm_operator_gt_float_int(double a, int b);

bool rustmm_operator_lt_int_int(int a, int b);
bool rustmm_operator_lt_int_float(int a, double b);
bool rustmm_operator_lt_float_float(double a, double b);
bool rustmm_operator_lt_float_int(double a, int b);

bool rustmm_operator_ge_int_int(int a, int b);
bool rustmm_operator_ge_int_float(int a, double b);
bool rustmm_operator_ge_float_float(double a, double b);
bool rustmm_operator_ge_float_int(double a, int b);

bool rustmm_operator_le_int_int(int a, int b);
bool rustmm_operator_le_int_float(int a, double b);
bool rustmm_operator_le_float_float(double a, double b);
bool rustmm_operator_le_float_int(double a, int b);

#endif
