run: conway
	cat conway.rs
	./conway

conway: conway.rs
	rustc conway.rs

clean:
	find . -type f -perm +111 -maxdepth 1 -exec rm {} \;
