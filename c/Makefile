CC = gcc
CFLAGS = -g -Wall
LDFLAGS = -g
LDLIBS = 

PHONY: clean
clean: 
	rm -rf *.o core hello

.PHONY: all 
all: clean hello

.PHONY: run
run: hello
	./hello

.PHONY: vg
vg: <executable>
	valgrind --leak-check=yes ./hello
