.PHONY: build test clean

# The binary to build (just the basename).
BINARY := monkey

BINARY := monkey

run: build
	./bin/${BINARY}

build:
	go build -o ./bin/${BINARY}

test:
	go test -v ./...

clean:
	go clean
	rm -f ./bin/${BINARY}