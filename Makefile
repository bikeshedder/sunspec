all:
	@echo Available targets:
	@echo - models
	@echo - update_models
	@echo - gen_models

models: update_models gen_models

update_models:
	git submodule update --init
	(cd models && git checkout master && git pull)

gen_models:
	rm -f src/models/*.rs
	(cd sunspec-gen && cargo run ../models/json ../src/models)
	cargo fmt -- src/models/*.rs
