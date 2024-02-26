SRC=$(wildcard src/*.c)
LIBS=-lm

main: $(SRC)
	gcc -I include -o $@ $^ $(LIBS)

clean:
	rm -rf main