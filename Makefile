# wallust See LICENSE file for copyright and license details.
VERSION = 3.3.0

# Customize below to fit your system

# paths
PREFIX ?= /usr/local
MANPREFIX ?= ${PREFIX}/share/man
CARGO ?= /usr/bin/cargo

# bsd
#PREFIX = /usr/pkg
#MANPREFIX = ${PREFIX}/man
#CARGO = ${PREFIX}/bin/cargo

# Hardcoded completions paths
ZSHPREFIX  = ${PREFIX}/share/zsh/site-functions
BASHPREFIX = ${PREFIX}/share/bash-completion/completions
FISHPREFIX = ${PREFIX}/share/fish/vendor_completions.d

# Some common (and "universal") targets for pkg-nix and pkg-win
# used by `pkg-nix`
NIX=x86_64-unknown-linux-musl
#NIX=x86_64-apple-darwin

# used by `pkg-win`
WIN=x86_64-pc-windows-gnu

# Redefine this variable if you use a given TARGET
RELEASE = target/release
# https://stackoverflow.com/a/32696474
CARGOFLAGS = --release

all: ${RELEASE}/wallust

${RELEASE}/wallust:
	@${CARGO} build ${CARGOFLAGS}

assets: man completions schema update-docs

# The generated completion could differ if `themes` compiletime feature is disabled.
completions:
	@echo "GENERATING SHELL COMPLETIONS"
	@${CARGO} test --quiet --features=buildgen --test=completions

# Here, however, if `themes is disabled, you only need to omit `wallust-themes.1` man page.
# also no need to rebuilt it on install, since by default, the repo includes them.
man:
	@echo "GENERATING MAN PAGES"
	@${CARGO} test --quiet --features=buildgen --test=man

schema:
	@echo "GENERATING JSON SCHEMA"
	@${CARGO} test --quiet --features=schema --test=jsonschema


update-docs:
	@echo "UPDATING MDBOOK DOCUMENTATION"
	@${CARGO} test --quiet --features=doc --test=docs

dist: clean
	mkdir -p wallust-${VERSION}
	cp -R LICENSE Makefile README.md wallust.toml Cargo.toml Cargo.lock src/* man/* completions/* wallust-${VERSION}
	tar -cf wallust-${VERSION}.tar wallust-${VERSION}
	gzip wallust-${VERSION}.tar
	rm -rf wallust-${VERSION}

docs: book/
	mdbook build

serve-docs: book/
	mdbook serve

install-completions: ## installs completions files
	mkdir -p ${DESTDIR}${ZSHPREFIX}
	cp -f completions/_wallust ${DESTDIR}${ZSHPREFIX}/_wallust
	mkdir -p ${DESTDIR}${BASHPREFIX}
	cp -f completions/wallust.bash ${DESTDIR}${BASHPREFIX}/wallust.bash
	mkdir -p ${DESTDIR}${FISHPREFIX}
	cp -f completions/wallust.fish ${DESTDIR}${FISHPREFIX}/wallust.fish

install-man: ## installs completions files
	mkdir -p ${DESTDIR}${MANPREFIX}/man1
	mkdir -p ${DESTDIR}${MANPREFIX}/man5
	cp -f man/wallust.1 man/wallust-theme.1 man/wallust-cs.1 man/wallust-run.1 $(DESTDIR)$(MANPREFIX)/man1
	cp -f man/wallust.5 $(DESTDIR)$(MANPREFIX)/man5
	chmod 644 ${DESTDIR}${MANPREFIX}/man1/wallust.1 \
                  ${DESTDIR}${MANPREFIX}/man5/wallust.5 \
                  ${DESTDIR}${MANPREFIX}/man1/wallust-cs.1 \
                  ${DESTDIR}${MANPREFIX}/man1/wallust-run.1 \
                  ${DESTDIR}${MANPREFIX}/man1/wallust-theme.1

install: all install-completions install-man
	mkdir -p ${DESTDIR}${PREFIX}/bin
	cp -f ${RELEASE}/wallust ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/wallust

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/wallust \
		${DESTDIR}${MANPREFIX}/man1/wallust.1 \
		${DESTDIR}${MANPREFIX}/man1/wallust-theme.1 \
		${DESTDIR}${MANPREFIX}/man1/wallust-cs.1 \
		${DESTDIR}${MANPREFIX}/man1/wallust-run.1 \
		${DESTDIR}${MANPREFIX}/man5/wallust.5 \
		${ZSHPREFIX}/_wallust \
		${BASHPREFIX}/wallust.bash \
		${FISHPREFIX}/wallust.fish
pkg-nix-with-assets:
	@${CARGO} build --release --target ${NIX}
	mkdir -p wallust-${VERSION}-${NIX}-with-assets/
	cp -f target/${NIX}/release/wallust wallust-${VERSION}-${NIX}-with-assets/wallust
	cp -fr completions/ wallust-${VERSION}-${NIX}-with-assets
	cp -fr man/ wallust-${VERSION}-${NIX}-with-assets
	tar czvf wallust-${VERSION}-${NIX}-with-assets.tar.gz wallust-${VERSION}-${NIX}-with-assets
	rm -fr wallust wallust-${VERSION}-${NIX}-with-assets

pkg-nix:
	@${CARGO} build --release --target ${NIX}
	cp -f target/${NIX}/release/wallust wallust
	tar czvf wallust-${VERSION}-${NIX}.tar.gz wallust
	rm -f wallust

pkg-win-with-assets:
	@${CARGO} build --release --target ${WIN}
	mkdir -p wallust-${VERSION}-${WIN}-with-assets/
	cp -f target/${WIN}/release/wallust.exe wallust-${VERSION}-${WIN}-with-assets/wallust.exe
	cp -fr completions/ wallust-${VERSION}-${WIN}-with-assets
	cp -fr man/ wallust-${VERSION}-${WIN}-with-assets
	tar czvf wallust-${VERSION}-${WIN}-with-assets.tar.gz wallust-${VERSION}-${WIN}-with-assets
	rm -fr wallust.exe wallust-${VERSION}-${WIN}-with-assets

pkg-win: ## can't be generalized out because of the .exe
	@${CARGO} build --release --target ${WIN}
	cp -f target/${WIN}/release/wallust.exe wallust.exe
	tar czvf wallust-${VERSION}-${WIN}.tar.gz wallust.exe
	rm -f wallust.exe

mostlyclean:
	rm -f ${RELEASE}/wallust

clean:
	@${CARGO} clean

.PHONY: all clean mostlyclean dist install uninstall install-completions completions man config
