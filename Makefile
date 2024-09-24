.PHONY: all run attach rm

all:
	docker-compose up 

run:
	docker-compose run --service-ports --rm rust bash

attach:
	docker-compose exec rust bash

rm:
	docker-compose down
	rm -rf target/*
