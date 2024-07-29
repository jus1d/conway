conway: conway.rs
	rustc conway.rs
	./conway

clean:
	find . -type f -perm +111 -maxdepth 1 -exec rm {} \;
