VERSION := 0.1.4
PKGNAME := linker-cli
BINARY := target/release/$(PKGNAME)
DEB_DIR := deb_build/$(PKGNAME)_$(VERSION)

all: build deb aur

build:
	cargo build --release

deb: $(BINARY)
	rm -rf $(DEB_DIR)
	mkdir -p $(DEB_DIR)/DEBIAN
	mkdir -p $(DEB_DIR)/usr/bin
	cp $(BINARY) $(DEB_DIR)/usr/bin/$(PKGNAME)
	chmod 755 $(DEB_DIR)/usr/bin/$(PKGNAME)

	echo "Package: $(PKGNAME)"                          > $(DEB_DIR)/DEBIAN/control
	echo "Version: $(VERSION)"                        >> $(DEB_DIR)/DEBIAN/control
	echo "Section: utils"                             >> $(DEB_DIR)/DEBIAN/control
	echo "Priority: optional"                         >> $(DEB_DIR)/DEBIAN/control
	echo "Architecture: amd64"                        >> $(DEB_DIR)/DEBIAN/control
	echo "Maintainer: Christian Regueiro <christianregueiro2001@gmail.com>" >> $(DEB_DIR)/DEBIAN/control
	echo "Description: Gestor de enlaces desde la terminal" >> $(DEB_DIR)/DEBIAN/control

	dpkg-deb --build $(DEB_DIR)

aur:
	cd aur && makepkg --printsrcinfo > .SRCINFO && git add PKGBUILD .SRCINFO && git commit -m "v$(VERSION)" && git push

