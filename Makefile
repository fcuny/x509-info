.PHONY: build run clean

BUILD_DIR=bin
PROGRAM_FILE=x509-info
VERSION=$(shell git describe --tag --always)
BUILD_DATE ?= $(shell TZ=UTC0 git show -s --format=%cd --date=format-local:'%Y-%m-%dT%H:%M:%SZ' HEAD)

build:
	@go build -o ${BUILD_DIR}/${PROGRAM_FILE} \
		-ldflags "-X main.Version=${VERSION} -X main.BuildDate=${BUILD_DATE}" \
		-trimpath main.go

run: build
	@./${BUILD_DIR}/${PROGRAM_FILE}

clean:
	@go clean
	@rm -rf ${BUILD_DIR}/${PROGRAM_FILE}
