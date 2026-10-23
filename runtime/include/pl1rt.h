/* SPDX-License-Identifier: Apache-2.0 */
#ifndef PL1RT_H
#define PL1RT_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <setjmp.h>

/* PUT LIST / PUT SKIP LIST — L0 minimum */
void pl1rt_put_skip(void);
void pl1rt_put_cstr(const char *s);
void pl1rt_put_i32(int v);
void pl1rt_put_f64(double v);
void pl1rt_put_space(void);
void pl1rt_put_nl(void);
/* PUT EDIT helpers */
void pl1rt_put_i32_w(int v, int width);
void pl1rt_put_f64_w(double v, int width, int frac);
void pl1rt_put_e64(double v, int width, int frac);
void pl1rt_put_cstr_w(const char *s, int width);
void pl1rt_put_x(int n);
void pl1rt_put_column(int col);
void pl1rt_put_page(void);
void pl1rt_put_line(int line);
void pl1rt_put_skip_n(int n);
/* Minimal picture: Z/9 digit slots, optional V → '.' */
void pl1rt_put_picture_i32(int v, const char *pic);

/* Fixed CHAR(n): pad/truncate with blanks; dst has n data bytes + NUL */
void pl1rt_char_assign(char *dst, size_t n, const char *src);
void pl1rt_char_assign_n(char *dst, size_t n, const char *src, size_t src_n);

/* GET LIST / GET SKIP LIST — L0 minimum (stdin) */
void pl1rt_get_skip(void);
int pl1rt_get_i32(void);
double pl1rt_get_f64(void);
void pl1rt_get_char(char *dst, size_t n);
/* GET EDIT helpers (fixed-width fields) */
void pl1rt_get_x(int n);
int pl1rt_get_i32_w(int width);
double pl1rt_get_f64_w(int width);
void pl1rt_get_char_w(char *dst, size_t n, int width);
/* GET DATA — next `NAME=value` field (case-insensitive name) */
int pl1rt_get_data_i32(const char *name);
double pl1rt_get_data_f64(const char *name);
void pl1rt_get_data_char(const char *name, char *dst, size_t n);
/* Free-form GET DATA: next field into name/val buffers; 1=ok, 0=EOF/end */
int pl1rt_get_data_next(char *name, size_t nsz, char *val, size_t vsz);
int pl1rt_name_eq(const char *a, const char *b);
/* Named STREAM files (OPEN/CLOSE FILE + FILE(f) on GET/PUT) */
/* Open ddname; path is the OS filename (may differ from name via TITLE). */
int pl1rt_fopen(const char *name, const char *path, const char *mode);
void pl1rt_fclose(const char *name);
void pl1rt_set_out(const char *name); /* NULL → stdout */
void pl1rt_set_in(const char *name);  /* NULL → stdin */

/* ON ENDFILE[(file)] — longjmp to jb when GET hits EOF on that stream */
void pl1rt_on_endfile(const char *file, jmp_buf *jb);
void pl1rt_revert_endfile(const char *file);
/* Raise ENDFILE for the current input stream if a handler is armed. */
void pl1rt_raise_endfile(void);
/* SIGNAL ENDFILE[(file)] — raise for named file (NULL → current input). */
void pl1rt_signal_endfile(const char *file);

#ifdef __cplusplus
}
#endif

#endif /* PL1RT_H */
