NAME := fcat
CARGO := cargo
INSTALL_DIR ?= /usr/local/bin
VERSION := $(shell grep -m1 version Cargo.toml | cut -d '"' -f2)

.PHONY: all build install test release debian aur clean lint

all: build

build:
	$(CARGO) build --release

install: build
	install -Dm755 target/release/$(NAME) $(INSTALL_DIR)/$(NAME)

test:
	$(CARGO) test

clean:
	$(CARGO) clean
	rm -rf debian/*.deb

lint:
	$(CARGO) clippy --all-targets -- -D warnings
	$(CARGO) fmt --all -- --check

version-bump:
	@echo "Current version: $(VERSION)"
	@read -p "Enter new version: " new_version; \
	sed -i "s/version = \"$(VERSION)\"/version = \"$${new_version}\"/g" Cargo.toml; \
	echo "Version updated to $${new_version}"

release: test lint
	@echo "Creating release v$(VERSION)"
	@if [ -z "$(shell git tag -l v$(VERSION))" ]; then \
		git tag -a v$(VERSION) -m "Release v$(VERSION)"; \
		git push origin v$(VERSION); \
	else \
		echo "Tag v$(VERSION) already exists. Run 'make version-bump' to update version."; \
	fi

debian: build
	mkdir -p debian/DEBIAN
	mkdir -p debian/usr/bin
	mkdir -p debian/usr/share/doc/$(NAME)
	cp target/release/$(NAME) debian/usr/bin/
	cp LICENSE debian/usr/share/doc/$(NAME)/copyright
	cp README.md debian/usr/share/doc/$(NAME)/
	@echo "Package: $(NAME)" > debian/DEBIAN/control
	@echo "Version: $(VERSION)" >> debian/DEBIAN/control
	@echo "Section: utils" >> debian/DEBIAN/control
	@echo "Priority: optional" >> debian/DEBIAN/control
	@echo "Architecture: amd64" >> debian/DEBIAN/control
	@echo "Depends: libc6 (>= 2.27)" >> debian/DEBIAN/control
	@echo "Recommends: wl-clipboard" >> debian/DEBIAN/control
	@echo "Maintainer: Thomas Almeida <hi@thomasalmeida.com>" >> debian/DEBIAN/control
	@echo "Description: Fast command-line file concatenator" >> debian/DEBIAN/control
	@echo " fcat is a high-performance command-line file concatenator written in Rust," >> debian/DEBIAN/control
	@echo " designed to safely aggregate text files while automatically ignoring binary" >> debian/DEBIAN/control
	@echo " and media files. Features clipboard integration and smart filtering." >> debian/DEBIAN/control
	@echo "fcat ($(VERSION)) unstable; urgency=low" > debian/usr/share/doc/$(NAME)/changelog
	@echo "" >> debian/usr/share/doc/$(NAME)/changelog
	@echo "  * Release version $(VERSION)" >> debian/usr/share/doc/$(NAME)/changelog
	@echo "" >> debian/usr/share/doc/$(NAME)/changelog
	@echo " -- Thomas Almeida <hi@thomasalmeida.com>  $$(date -R)" >> debian/usr/share/doc/$(NAME)/changelog
	gzip -9 -n debian/usr/share/doc/$(NAME)/changelog
	chmod 755 debian/usr/bin/$(NAME)
	find debian -type d -exec chmod 755 {} \;
	dpkg-deb --build debian $(NAME)_$(VERSION)_amd64.deb

# For manual AUR management - GitHub Actions will handle automated updates
aur:
	@echo "Updating AUR package"
	@if [ ! -d aur ]; then \
		git clone ssh://aur@aur.archlinux.org/$(NAME).git aur; \
	else \
		cd aur && git pull; \
	fi
	cat > aur/PKGBUILD << EOF
	# Maintainer: Thomas Almeida <hi@thomasalmeida.com>
	pkgname=$(NAME)
	pkgver=$(VERSION)
	pkgrel=1
	pkgdesc="Fast command-line file concatenator with smart filtering and clipboard integration"
	arch=('x86_64')
	url="https://github.com/thomasalmeida/$(NAME)"
	license=('MIT')
	depends=('gcc-libs')
	optdepends=('wl-clipboard: clipboard support')
	makedepends=('cargo' 'git')
	source=("\$${pkgname}-\$${pkgver}.tar.gz::https://github.com/thomasalmeida/$(NAME)/archive/v\$${pkgver}.tar.gz")
	sha256sums=('SKIP')

	build() {
	  cd "\$${pkgname}-\$${pkgver}"
	  cargo build --release --locked
	}

	check() {
	  cd "\$${pkgname}-\$${pkgver}"
	  cargo test --release
	}

	package() {
	  cd "\$${pkgname}-\$${pkgver}"
	  install -Dm755 "target/release/\$${pkgname}" "\$${pkgdir}/usr/bin/\$${pkgname}"
	  install -Dm644 LICENSE "\$${pkgdir}/usr/share/licenses/\$${pkgname}/LICENSE"
	  install -Dm644 README.md "\$${pkgdir}/usr/share/doc/\$${pkgname}/README.md"
	}
	EOF
	cd aur && makepkg --printsrcinfo > .SRCINFO
	cd aur && git add PKGBUILD .SRCINFO
	cd aur && git commit -m "Update to version $(VERSION)"
	cd aur && git push

