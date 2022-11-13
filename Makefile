.PHONY: all

all:
	$(MAKE) -C parmesan all
	$(MAKE) -C prolog all
	$(MAKE) -C rust all

	docker compose up
