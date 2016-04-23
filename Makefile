all: main run

main: main.rs
	rustc main.rs

run:
	./main

clean:
	-rm main
