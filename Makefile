GOCMD=go
BUILD_DIR=$(PWD)/out

clean:
	echo 'clean:'
	if [[ -d "${BUILD_DIR}" ]]; then rm -rf "${BUILD_DIR}"; fi

build:
	echo 'build:'
	if [[ ! -d "${BUILD_DIR}" ]]; then mkdir "${BUILD_DIR}"; fi
	$(GOCMD) build -o ${BUILD_DIR} ./cmd/kaf
