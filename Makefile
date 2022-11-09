.PHONY: all

all:
	$(MAKE) -C parmesan all
	$(MAKE) -C prolog all

	docker compose up
