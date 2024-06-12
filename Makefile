.PHONY: build test clean

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