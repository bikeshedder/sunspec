all:
	@echo Available targets:
	@echo - update_models

update_models:
	git submodule update --init
	(cd models && git checkout master && git pull)
	(cd sunspec-gen && cargo run ../models/smdx > ../src/models.rs)
	cargo fmt src/models.rs
	git add models src/models.rs
