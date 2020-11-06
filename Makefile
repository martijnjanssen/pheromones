##
# Pheromones
#
# @file
# @version 0.1

.PHONY: run

pkg:
	wasm-pack build

www/node_modules: pkg
	cd www
	npm install

run: www/node_modules
	cd www; yarn start

# end
