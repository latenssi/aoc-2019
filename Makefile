IMAGE_NAME=aoc-rust-dev
WORKDIR=/usr/src/myapp

bash: build
	docker run -it --rm  -v $(shell pwd):${WORKDIR} ${IMAGE_NAME} /bin/bash

build:
	docker build -t ${IMAGE_NAME} .

day%: build
	-docker run -it --rm  -v $(shell pwd):${WORKDIR} ${IMAGE_NAME} /bin/bash -c "cargo new $@" && rm -rf $@/.git
	docker  run -it --rm  -v $(shell pwd):${WORKDIR} ${IMAGE_NAME} /bin/bash -c "cd $@; cargo run"