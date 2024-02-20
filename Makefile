SRC=$(wildcard src/*.c)

main: $(SRC)
	gcc -o $@ $^

clean:
	rm -rf main