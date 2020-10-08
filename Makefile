kequality_spoj:
	rustc -C opt-level=3 kequality_spoj.rs
	strip kequality_spoj

clean:
	rm -f kequality_spoj
