all: build

build:
	meson setup ./work
	meson compile -C ./work
	cp ./work/cpp-test ./main

clean:
	rm -fr ./work
