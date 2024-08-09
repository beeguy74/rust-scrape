
.PHONY: run rm
all:
	docker-compose up -d

run:
	docker-compose run --service-ports --rm rust bash
