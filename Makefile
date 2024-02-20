SRC=$(wildcard src/*.c)
LIBS=-lm

main: $(SRC)
	gcc -o $@ $^ $(LIBS)

clean:
	rm -rf main