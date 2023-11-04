all:
	@echo Available targets:
	@echo - update_models

update_models:
	git submodule update --init
	(cd models && git checkout master && git pull)
	rm src/models/*.rs
	(cd sunspec-gen && cargo run ../models/smdx ../src/models)
	cargo fmt src/models
	git add models src/models
