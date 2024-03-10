SRC=$(wildcard src/*.c)
DIRS=-Iinclude
LIBS=-lm -lgmp
CFLAGS=-Wall -Wextra -Werror -O2

main: $(SRC)
	gcc $(DIRS) $(CFLAGS) -o crel $^ $(LIBS)

clean:
	rm -rf crel