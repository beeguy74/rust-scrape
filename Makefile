.PHONY: all run attach rm

all:
	docker-compose up -d

run:
	docker-compose run --service-ports --rm rust bash

attach:
	docker-compose exec rust bash

rm:
	docker-compose down
	docker container rm $(docker container ls -a -q)
