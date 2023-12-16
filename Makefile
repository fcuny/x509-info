.PHONY: build run clean

BUILD_DIR=bin
PROGRAM_FILE=x509-info

build:
	@go build \
		-o ${BUILD_DIR}/${PROGRAM_FILE} \
		main.go

run: build
	@./${BUILD_DIR}/${PROGRAM_FILE}

clean:
	@go clean
	@rm -rf ${BUILD_DIR}/${PROGRAM_FILE}
