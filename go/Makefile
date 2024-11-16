all: main

main: src/*.go
	./build.sh

run: main
	./main

watch: main
	./build_and_watch.sh

clean:
	rm -f ./main
