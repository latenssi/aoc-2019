IMAGE_NAME=aoc-rust-dev

bash: build
	docker run -it --rm  -v $(shell pwd):/usr/src/myapp ${IMAGE_NAME} /bin/bash

build:
	docker build -t ${IMAGE_NAME} .

day%: build
	-docker run -it --rm  -v $(shell pwd):/usr/src/myapp ${IMAGE_NAME} /bin/bash -c "cargo new $@" && rm -rf $@/.git
	docker run -it --rm  -v $(shell pwd):/usr/src/myapp ${IMAGE_NAME} /bin/bash -c "cd $@; cargo run"