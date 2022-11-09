.PHONY: all

all:
	$(MAKE) -C prolog all
	docker compose up
