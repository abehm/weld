CC=clang++
LDFLAGS=-L../../../target/release/
LIBS=-lweld

.PHONY: all clean

all:
	${CC} -O3 -march=native ${LDFLAGS} ${LIBS} vector_benchmark.cpp -o run

clean:
	rm -rf run

