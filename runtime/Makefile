# Build static libpl1rt for the host (gcc).
# Usage: make -C runtime CC=gcc

CC ?= gcc
AR ?= ar
CFLAGS ?= -std=c11 -Wall -Wextra -O2 -Iinclude

OBJ = src/put.o
LIB = libpl1rt.a

.PHONY: all clean

all: $(LIB)

$(LIB): $(OBJ)
	$(AR) rcs $@ $(OBJ)

src/put.o: src/put.c include/pl1rt.h
	$(CC) $(CFLAGS) -c src/put.c -o $@

clean:
	rm -f $(OBJ) $(LIB)
