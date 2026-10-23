/* SPDX-License-Identifier: Apache-2.0 */
#include "pl1rt.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Active stream overrides (NULL = stdout / stdin). */
static FILE *pl1rt_outf = NULL;
static FILE *pl1rt_inf = NULL;
/* Logical name of the active input stream (default SYSIN). */
static char pl1rt_in_name[64] = "SYSIN";

static FILE *pl1rt_of(void) {
    return pl1rt_outf ? pl1rt_outf : stdout;
}

static FILE *pl1rt_if(void) {
    return pl1rt_inf ? pl1rt_inf : stdin;
}

int pl1rt_name_eq(const char *a, const char *b); /* defined below */

#define PL1RT_MAX_ON 8
static struct {
    char name[64]; /* empty = match any file */
    jmp_buf *jb;
} pl1rt_ef[PL1RT_MAX_ON];
static int pl1rt_ef_n = 0;

static void pl1rt_set_in_name(const char *name) {
    if (!name || !name[0]) {
        strncpy(pl1rt_in_name, "SYSIN", sizeof(pl1rt_in_name) - 1);
        pl1rt_in_name[sizeof(pl1rt_in_name) - 1] = '\0';
        return;
    }
    strncpy(pl1rt_in_name, name, sizeof(pl1rt_in_name) - 1);
    pl1rt_in_name[sizeof(pl1rt_in_name) - 1] = '\0';
}

void pl1rt_on_endfile(const char *file, jmp_buf *jb) {
    int i;
    if (!jb) {
        return;
    }
    for (i = 0; i < pl1rt_ef_n; i++) {
        if ((!file || !file[0]) && pl1rt_ef[i].name[0] == '\0') {
            pl1rt_ef[i].jb = jb;
            return;
        }
        if (file && file[0] && pl1rt_name_eq(pl1rt_ef[i].name, file)) {
            pl1rt_ef[i].jb = jb;
            return;
        }
    }
    if (pl1rt_ef_n >= PL1RT_MAX_ON) {
        return;
    }
    if (file && file[0]) {
        strncpy(pl1rt_ef[pl1rt_ef_n].name, file, sizeof(pl1rt_ef[0].name) - 1);
        pl1rt_ef[pl1rt_ef_n].name[sizeof(pl1rt_ef[0].name) - 1] = '\0';
    } else {
        pl1rt_ef[pl1rt_ef_n].name[0] = '\0';
    }
    pl1rt_ef[pl1rt_ef_n].jb = jb;
    pl1rt_ef_n++;
}

void pl1rt_revert_endfile(const char *file) {
    int i;
    for (i = 0; i < pl1rt_ef_n; i++) {
        int match = 0;
        if ((!file || !file[0]) && pl1rt_ef[i].name[0] == '\0') {
            match = 1;
        } else if (file && file[0] && pl1rt_name_eq(pl1rt_ef[i].name, file)) {
            match = 1;
        }
        if (match) {
            pl1rt_ef[i] = pl1rt_ef[pl1rt_ef_n - 1];
            pl1rt_ef_n--;
            return;
        }
    }
}

void pl1rt_signal_endfile(const char *file) {
    int i;
    jmp_buf *wild = NULL;
    const char *want = (file && file[0]) ? file : pl1rt_in_name;
    for (i = 0; i < pl1rt_ef_n; i++) {
        if (pl1rt_ef[i].name[0] == '\0') {
            wild = pl1rt_ef[i].jb;
            continue;
        }
        if (want && pl1rt_name_eq(pl1rt_ef[i].name, want)) {
            if (pl1rt_ef[i].jb) {
                longjmp(*pl1rt_ef[i].jb, 1);
            }
            return;
        }
    }
    if (wild) {
        longjmp(*wild, 1);
    }
}

void pl1rt_raise_endfile(void) {
    pl1rt_signal_endfile(pl1rt_in_name);
}

static void pl1rt_eof_check(void) {
    if (feof(pl1rt_if())) {
        pl1rt_raise_endfile();
    }
}

#define PL1RT_MAX_FILES 8
static struct {
    char name[64];
    FILE *fp;
} pl1rt_files[PL1RT_MAX_FILES];

static int pl1rt_file_slot(const char *name) {
    int i;
    int free_i = -1;
    if (!name || !name[0]) {
        return -1;
    }
    for (i = 0; i < PL1RT_MAX_FILES; i++) {
        if (pl1rt_files[i].fp == NULL) {
            if (free_i < 0) {
                free_i = i;
            }
            continue;
        }
        if (pl1rt_name_eq(pl1rt_files[i].name, name)) {
            return i;
        }
    }
    return free_i;
}

int pl1rt_fopen(const char *name, const char *path, const char *mode) {
    int i;
    FILE *fp;
    const char *os = path && path[0] ? path : name;
    if (!name || !mode || !os) {
        return 0;
    }
    i = pl1rt_file_slot(name);
    if (i < 0) {
        return 0;
    }
    if (pl1rt_files[i].fp) {
        fclose(pl1rt_files[i].fp);
        pl1rt_files[i].fp = NULL;
    }
    fp = fopen(os, mode);
    if (!fp) {
        return 0;
    }
    strncpy(pl1rt_files[i].name, name, sizeof(pl1rt_files[i].name) - 1);
    pl1rt_files[i].name[sizeof(pl1rt_files[i].name) - 1] = '\0';
    pl1rt_files[i].fp = fp;
    return 1;
}

void pl1rt_fclose(const char *name) {
    int i;
    if (!name) {
        return;
    }
    for (i = 0; i < PL1RT_MAX_FILES; i++) {
        if (pl1rt_files[i].fp && pl1rt_name_eq(pl1rt_files[i].name, name)) {
            fclose(pl1rt_files[i].fp);
            pl1rt_files[i].fp = NULL;
            pl1rt_files[i].name[0] = '\0';
            return;
        }
    }
}

static FILE *pl1rt_lookup(const char *name) {
    int i;
    if (!name) {
        return NULL;
    }
    if (pl1rt_name_eq(name, "SYSPRINT")) {
        return stdout;
    }
    if (pl1rt_name_eq(name, "SYSIN")) {
        return stdin;
    }
    for (i = 0; i < PL1RT_MAX_FILES; i++) {
        if (pl1rt_files[i].fp && pl1rt_name_eq(pl1rt_files[i].name, name)) {
            return pl1rt_files[i].fp;
        }
    }
    return NULL;
}

void pl1rt_set_out(const char *name) {
    if (!name) {
        pl1rt_outf = NULL;
        return;
    }
    pl1rt_outf = pl1rt_lookup(name);
}

void pl1rt_set_in(const char *name) {
    if (!name) {
        pl1rt_inf = NULL;
        pl1rt_set_in_name("SYSIN");
        return;
    }
    pl1rt_inf = pl1rt_lookup(name);
    pl1rt_set_in_name(name);
}


/* 1-based column / line of the next character (COLUMN / LINE / PAGE). */
static int pl1rt_out_col = 1;
static int pl1rt_out_line = 1;

static void pl1rt_adv(int n) {
    if (n > 0) {
        pl1rt_out_col += n;
    }
}

void pl1rt_put_skip(void) {
    fputc('\n', pl1rt_of());
    pl1rt_out_col = 1;
    pl1rt_out_line++;
}

void pl1rt_put_skip_n(int n) {
    for (int i = 0; i < n; i++) {
        pl1rt_put_skip();
    }
}

void pl1rt_put_page(void) {
    fputc('\f', pl1rt_of());
    pl1rt_out_col = 1;
    pl1rt_out_line = 1;
}

void pl1rt_put_line(int line) {
    if (line < 1) {
        line = 1;
    }
    if (pl1rt_out_line > line) {
        pl1rt_put_page();
    }
    while (pl1rt_out_line < line) {
        fputc('\n', pl1rt_of());
        pl1rt_out_line++;
        pl1rt_out_col = 1;
    }
}

void pl1rt_put_cstr(const char *s) {
    if (s) {
        fputs(s, pl1rt_of());
        pl1rt_adv((int)strlen(s));
    }
}

void pl1rt_put_i32(int v) {
    int n = fprintf(pl1rt_of(), "%d", v);
    pl1rt_adv(n);
}

void pl1rt_put_f64(double v) {
    int n = fprintf(pl1rt_of(), "%g", v);
    pl1rt_adv(n);
}

void pl1rt_put_space(void) {
    fputc(' ', pl1rt_of());
    pl1rt_adv(1);
}

void pl1rt_put_nl(void) {
    fputc('\n', pl1rt_of());
    pl1rt_out_col = 1;
    pl1rt_out_line++;
}

void pl1rt_put_i32_w(int v, int width) {
    int n;
    if (width > 0) {
        n = fprintf(pl1rt_of(), "%*d", width, v);
    } else {
        n = fprintf(pl1rt_of(), "%d", v);
    }
    pl1rt_adv(n);
}

void pl1rt_put_f64_w(double v, int width, int frac) {
    int n;
    if (width > 0 && frac >= 0) {
        n = fprintf(pl1rt_of(), "%*.*f", width, frac, v);
    } else if (width > 0) {
        n = fprintf(pl1rt_of(), "%*g", width, v);
    } else {
        n = fprintf(pl1rt_of(), "%g", v);
    }
    pl1rt_adv(n);
}

void pl1rt_put_e64(double v, int width, int frac) {
    int n;
    if (width > 0 && frac >= 0) {
        n = fprintf(pl1rt_of(), "%*.*E", width, frac, v);
    } else if (width > 0) {
        n = fprintf(pl1rt_of(), "%*E", width, v);
    } else if (frac >= 0) {
        n = fprintf(pl1rt_of(), "%.*E", frac, v);
    } else {
        n = fprintf(pl1rt_of(), "%E", v);
    }
    pl1rt_adv(n);
}

void pl1rt_put_cstr_w(const char *s, int width) {
    if (!s) {
        s = "";
    }
    if (width <= 0) {
        fputs(s, pl1rt_of());
        pl1rt_adv((int)strlen(s));
        return;
    }
    int len = (int)strlen(s);
    if (len >= width) {
        fwrite(s, 1, (size_t)width, pl1rt_of());
        pl1rt_adv(width);
    } else {
        fputs(s, pl1rt_of());
        for (int i = len; i < width; i++) {
            fputc(' ', pl1rt_of());
        }
        pl1rt_adv(width);
    }
}

void pl1rt_put_x(int n) {
    for (int i = 0; i < n; i++) {
        fputc(' ', pl1rt_of());
    }
    if (n > 0) {
        pl1rt_adv(n);
    }
}

void pl1rt_put_column(int col) {
    if (col < 1) {
        col = 1;
    }
    if (pl1rt_out_col > col) {
        fputc('\n', pl1rt_of());
        pl1rt_out_col = 1;
        pl1rt_out_line++;
    }
    while (pl1rt_out_col < col) {
        fputc(' ', pl1rt_of());
        pl1rt_out_col++;
    }
}

static void pl1rt_pic_emit_sig(int *sig, int *want_dollar) {
    if (*want_dollar) {
        fputc('$', pl1rt_of());
        pl1rt_adv(1);
        *want_dollar = 0;
    }
    *sig = 1;
}

void pl1rt_put_picture_i32(int v, const char *pic) {
    char digits[64];
    int nd = 0;
    int i;
    unsigned absv;
    int sig;
    int di;
    int want_dollar = 0;
    int after_v = 0;
    int frac_digits = 0;
    if (!pic) {
        pic = "";
    }
    for (i = 0; pic[i]; i++) {
        char c = pic[i];
        if (c == 'V' || c == 'v') {
            after_v = 1;
            continue;
        }
        if (c == '9' || c == 'Z' || c == 'z') {
            nd++;
            if (after_v) {
                frac_digits++;
            }
        }
    }
    if (nd <= 0 || nd >= (int)sizeof(digits)) {
        return;
    }
    if (v < 0) {
        fputc('-', pl1rt_of());
        pl1rt_adv(1);
        absv = (unsigned)(-(v + 1)) + 1u;
    } else {
        absv = (unsigned)v;
    }
    for (i = nd - 1; i >= 0; i--) {
        digits[i] = (char)('0' + (absv % 10u));
        absv /= 10u;
    }
    digits[nd] = '\0';
    (void)frac_digits; /* scale implied by V digit count; value is already scaled */
    sig = 0;
    di = 0;
    for (i = 0; pic[i]; i++) {
        char c = pic[i];
        if (c == '9') {
            pl1rt_pic_emit_sig(&sig, &want_dollar);
            fputc(digits[di++], pl1rt_of());
            pl1rt_adv(1);
        } else if (c == 'Z' || c == 'z') {
            char d = digits[di++];
            if (!sig && d == '0') {
                fputc(' ', pl1rt_of());
            } else {
                pl1rt_pic_emit_sig(&sig, &want_dollar);
                fputc(d, pl1rt_of());
            }
            pl1rt_adv(1);
        } else if (c == 'V' || c == 'v') {
            fputc('.', pl1rt_of());
            pl1rt_adv(1);
            /* fractional digits always significant for display */
            sig = 1;
        } else if (c == ',') {
            if (sig) {
                fputc(',', pl1rt_of());
            } else {
                fputc(' ', pl1rt_of());
            }
            pl1rt_adv(1);
        } else if (c == '$') {
            if (sig) {
                fputc('$', pl1rt_of());
                pl1rt_adv(1);
            } else {
                want_dollar = 1;
            }
        } else if (c == '.') {
            /* explicit point (same as V for L0) */
            fputc('.', pl1rt_of());
            pl1rt_adv(1);
            sig = 1;
        } else {
            fputc(c, pl1rt_of());
            pl1rt_adv(1);
        }
    }
    if (want_dollar) {
        /* no significant digits — still show a dollar if picture asked */
        fputc('$', pl1rt_of());
        pl1rt_adv(1);
    }
}

void pl1rt_char_assign(char *dst, size_t n, const char *src) {
    size_t src_n = src ? strlen(src) : 0;
    pl1rt_char_assign_n(dst, n, src, src_n);
}

void pl1rt_char_assign_n(char *dst, size_t n, const char *src, size_t src_n) {
    size_t i = 0;
    if (src) {
        while (i < n && i < src_n) {
            dst[i] = src[i];
            i++;
        }
    }
    while (i < n) {
        dst[i++] = ' ';
    }
    dst[n] = '\0';
}

void pl1rt_get_skip(void) {
    int c;
    while ((c = fgetc(pl1rt_if())) != EOF && c != '\n') {
    }
}

int pl1rt_get_i32(void) {
    int v = 0;
    if (fscanf(pl1rt_if(), "%d", &v) != 1) {
        pl1rt_eof_check();
        return 0;
    }
    return v;
}

double pl1rt_get_f64(void) {
    double v = 0.0;
    if (fscanf(pl1rt_if(), "%lf", &v) != 1) {
        pl1rt_eof_check();
        return 0.0;
    }
    return v;
}

void pl1rt_get_char(char *dst, size_t n) {
    char buf[4097];
    size_t lim = n < 4096 ? n : 4096;
    buf[0] = '\0';
    if (fscanf(pl1rt_if(), "%4096s", buf) != 1) {
        pl1rt_eof_check();
        pl1rt_char_assign(dst, n, "");
        return;
    }
    (void)lim;
    pl1rt_char_assign(dst, n, buf);
}

static size_t pl1rt_read_field(char *buf, size_t bufsz, int width) {
    size_t lim;
    size_t i = 0;
    if (!buf || bufsz == 0 || width <= 0) {
        if (buf && bufsz > 0) {
            buf[0] = '\0';
        }
        return 0;
    }
    lim = (size_t)width < bufsz - 1 ? (size_t)width : bufsz - 1;
    while (i < lim) {
        int c = fgetc(pl1rt_if());
        if (c == EOF) {
            break;
        }
        buf[i++] = (char)c;
    }
    buf[i] = '\0';
    return i;
}

void pl1rt_get_x(int n) {
    for (int i = 0; i < n; i++) {
        int c = fgetc(pl1rt_if());
        if (c == EOF) {
            break;
        }
    }
}

int pl1rt_get_i32_w(int width) {
    char buf[64];
    long v;
    char *end = NULL;
    if (width <= 0) {
        return pl1rt_get_i32();
    }
    if (pl1rt_read_field(buf, sizeof(buf), width) == 0) {
        pl1rt_eof_check();
        return 0;
    }
    v = strtol(buf, &end, 10);
    (void)end;
    return (int)v;
}

double pl1rt_get_f64_w(int width) {
    char buf[64];
    double v;
    char *end = NULL;
    if (width <= 0) {
        return pl1rt_get_f64();
    }
    if (pl1rt_read_field(buf, sizeof(buf), width) == 0) {
        pl1rt_eof_check();
        return 0.0;
    }
    v = strtod(buf, &end);
    (void)end;
    return v;
}

void pl1rt_get_char_w(char *dst, size_t n, int width) {
    char buf[4097];
    int w = width;
    if (w <= 0) {
        w = (int)n;
    }
    if (w > 4096) {
        w = 4096;
    }
    pl1rt_read_field(buf, sizeof(buf), w);
    pl1rt_char_assign(dst, n, buf);
}

int pl1rt_name_eq(const char *a, const char *b) {
    if (!a || !b) {
        return 0;
    }
    while (*a && *b) {
        char ca = *a;
        char cb = *b;
        if (ca >= 'a' && ca <= 'z') {
            ca = (char)(ca - 'a' + 'A');
        }
        if (cb >= 'a' && cb <= 'z') {
            cb = (char)(cb - 'a' + 'A');
        }
        if (ca != cb) {
            return 0;
        }
        a++;
        b++;
    }
    return *a == '\0' && *b == '\0';
}

static int pl1rt_ci_eq(const char *a, const char *b) {
    return pl1rt_name_eq(a, b);
}

static int pl1rt_get_data_field(const char *name, char *val, size_t valsz) {
    char id[128];
    size_t i = 0;
    int c;
    if (!name || !val || valsz == 0) {
        return 0;
    }
    val[0] = '\0';
    do {
        c = fgetc(pl1rt_if());
    } while (c == ' ' || c == '\t' || c == '\n' || c == '\r');
    if (c == EOF) {
        pl1rt_raise_endfile();
        return 0;
    }
    while (c != EOF && c != '=' && c != ' ' && c != '\t' && c != '\n' && c != '\r') {
        if (i + 1 < sizeof(id)) {
            id[i++] = (char)c;
        }
        c = fgetc(pl1rt_if());
    }
    id[i] = '\0';
    if (c != '=') {
        return 0;
    }
    if (!pl1rt_ci_eq(id, name)) {
        return 0;
    }
    i = 0;
    c = fgetc(pl1rt_if());
    while (c != EOF && c != ' ' && c != '\t' && c != '\n' && c != '\r' && c != ';') {
        if (i + 1 < valsz) {
            val[i++] = (char)c;
        }
        c = fgetc(pl1rt_if());
    }
    val[i] = '\0';
    return 1;
}

int pl1rt_get_data_i32(const char *name) {
    char val[64];
    long v;
    char *end = NULL;
    if (!pl1rt_get_data_field(name, val, sizeof(val))) {
        return 0;
    }
    v = strtol(val, &end, 10);
    (void)end;
    return (int)v;
}

double pl1rt_get_data_f64(const char *name) {
    char val[64];
    double v;
    char *end = NULL;
    if (!pl1rt_get_data_field(name, val, sizeof(val))) {
        return 0.0;
    }
    v = strtod(val, &end);
    (void)end;
    return v;
}

void pl1rt_get_data_char(const char *name, char *dst, size_t n) {
    char val[4097];
    if (!pl1rt_get_data_field(name, val, sizeof(val))) {
        pl1rt_char_assign(dst, n, "");
        return;
    }
    pl1rt_char_assign(dst, n, val);
}

int pl1rt_get_data_next(char *name, size_t nsz, char *val, size_t vsz) {
    size_t i = 0;
    int c;
    if (!name || nsz == 0 || !val || vsz == 0) {
        return 0;
    }
    name[0] = '\0';
    val[0] = '\0';
    do {
        c = fgetc(pl1rt_if());
    } while (c == ' ' || c == '\t' || c == '\n' || c == '\r');
    if (c == EOF) {
        pl1rt_raise_endfile();
        return 0;
    }
    if (c == ';') {
        return 0;
    }
    while (c != EOF && c != '=' && c != ' ' && c != '\t' && c != '\n' && c != '\r' && c != ';') {
        if (i + 1 < nsz) {
            name[i++] = (char)c;
        }
        c = fgetc(pl1rt_if());
    }
    name[i] = '\0';
    if (c != '=') {
        return 0;
    }
    i = 0;
    c = fgetc(pl1rt_if());
    while (c != EOF && c != ' ' && c != '\t' && c != '\n' && c != '\r' && c != ';') {
        if (i + 1 < vsz) {
            val[i++] = (char)c;
        }
        c = fgetc(pl1rt_if());
    }
    val[i] = '\0';
    return name[0] != '\0';
}
