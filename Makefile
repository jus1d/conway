run: conway
	./conway

conway: conway.rs
	rustc conway.rs
