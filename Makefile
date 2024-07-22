run: conway
	./conway

conway: conway.rs
	rustc conway.rs

clear:
	find . -type f -perm +111 -maxdepth 1 -exec rm {} \;
