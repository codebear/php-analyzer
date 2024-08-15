all:
	@echo
	@echo "make <autonodes|native|autofix>"
	@echo

autonodes:
	cd tree-sitter-php/php && tree-sitter generate
	rm -f src/autonodes/*.rs
	node gennodes.mjs
	rustfmt src/autonodes/*.rs

native:
	cd src/native && php generate.php
	rustfmt src/native/**/*.rs

autofix:
	cargo fix --allow-dirty --allow-staged
	$(MAKE) rustfmt

rustfmt:
	rustfmt src/**/*.rs


clippyfix:
	cargo clippy --fix --allow-dirty --allow-staged
	# -- --deny clippy::useless_conversion
	# -- --allow clippy::from_over_into
	#  -- \
	#	--deny clippy::all \
	#	--allow clippy::useless_conversion \
	#	--allow clippy::redundant_pattern_matching \
	#	--allow clippy::len_without_is_empty \
	#	--allow clippy::if_same_then_else \
	#	--allow clippy::match_like_matches_macro \
	#	--allow clippy::ptr_arg \
	#	--allow clippy::assertions_on_constants