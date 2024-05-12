all:
	@echo
	@echo "make <autonodes|native|autofix>"
	@echo

autonodes:
	cd tree-sitter-php/php && tree-sitter generate
	rm src/autonodes/*.rs
	node gennodes.mjs
	rustfmt src/autonodes/*.rs

native:
	cd src/native && php generate.php
	rustfmt src/native/**/*.rs

autofix:
	cargo fix --allow-dirty --allow-staged
	rustfmt src/**/*.rs
