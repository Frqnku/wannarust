NAME=infected

all: build

build:
	docker build -t $(NAME) .

run:
	docker run -it $(NAME)

down:
	docker stop $$(docker ps -q --filter "ancestor=$(NAME)")
	docker rm $$(docker ps -aq --filter "ancestor=$(NAME)")

clean:
	docker rm -f $(NAME)

.PHONY: all build-docker run down clean