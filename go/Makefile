all: main

main: src/*.go
	go build -o main ./src

run: main
	./main

watch: main
	./build_and_watch.sh

clean:
	rm -f ./main
