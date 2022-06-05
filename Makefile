current_dir = $(shell pwd)

.PHONY: docker-build
docker-build:
	docker build . -f ./dockerfiles/build -t rust-build-container
	docker run -v $(current_dir):/app rust-build-container  

.PHONY: docker-run 
docker-run:
	docker build . -f ./dockerfiles/run -t rust-run-container
	docker run rust-run-container