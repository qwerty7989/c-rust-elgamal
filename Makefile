SRC=$(wildcard src/*.c)
LIBS=-lm

main: $(SRC)
	gcc -I include -o crel $^ $(LIBS)

clean:
	rm -rf crel