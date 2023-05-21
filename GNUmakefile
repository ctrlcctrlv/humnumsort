all: dist

.PHONY: dist
dist:
	cargo generate-rpm -p bin
	cargo deb -p hns
	$(MAKE) mv

.PHONY: mv
mv:
	mv target/debian/*.deb .
	mv target/generate-rpm/*.rpm .
