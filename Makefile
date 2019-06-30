.PHONY: setup
setup:
	yarn

.PHONY: start
start:
	yarn start

.PHONY: build
build:
	(cd td4 && wasm-pack build -d ../static)
	yarn build

.PHONY: test
test:
	(cd td4 && cargo test)

.PHONY: deploy
deploy:
	yarn deploy
