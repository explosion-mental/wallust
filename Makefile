# wallust See LICENSE file for copyright and license details.
VERSION = 2.10.0

# paths
PREFIX = /usr/local
MANPREFIX = ${PREFIX}/share/man
CARGO = /usr/bin/cargo

# Hardcoded completions paths
ZSHPREFIX  = ${PREFIX}/share/zsh/site-functions
BASHPREFIX = ${PREFIX}/share/bash-completion/completions
FISHPREFIX = ${PREFIX}/fish/vendor_completions.d

#RELEASE = $$(test -z "$(TARGET)" && echo "target/release") \
#	  $$(test -n "$(TARGET)" && echo "target/$(TARGET)/release")
# TODO rust arch, empty means native
#TARGET =
#CARGOFLAGS = $$(test -z "$(TARGET)" && printf "%s" "--release" || printf "%s" "--release --target=$(TARGET)")

# some common targets
NIX=x86_64-unknown-linux-musl
#NIX=x86_64-apple-darwin
WIN=x86_64-pc-windows-gnu

# Redefine this variable if you use a given TARGET
RELEASE = target/release
# https://stackoverflow.com/a/32696474
CARGOFLAGS = --release

all: ${RELEASE}/wallust

${RELEASE}/wallust:
	@${CARGO} build ${CARGOFLAGS}

# The generated completion could differ if `themes` compiletime feature is disabled.
completions:
	@${CARGO} test --quiet --features=buildgen --test=completions

# Here, however, if `themes is disabled, you only need to omit `wallust-themes.1` man page.
# also no need to rebuilt it on install, since by default, the repo includes them.
man:
	@${CARGO} test --quiet --features=buildgen --test=man

install-completions: completions ## installs completions files
	mkdir -p ${DESTDIR}${ZSHPREFIX}
	cp -f completions/_wallust ${DESTDIR}${ZSHPREFIX}/_wallust
	mkdir -p ${DESTDIR}${BASHPREFIX}
	cp -f completions/wallust.bash ${DESTDIR}${BASHPREFIX}/wallust.bash
	mkdir -p ${DESTDIR}${FISHPREFIX}
	cp -f completions/wallust.fish ${DESTDIR}${BASHPREFIX}/wallust.fish

dist: clean
	mkdir -p wallust-${VERSION}
	cp -R LICENSE Makefile README.md wallust.toml Cargo.toml Cargo.lock src/* man/* completions/* wallust-${VERSION}
	tar -cf wallust-${VERSION}.tar wallust-${VERSION}
	gzip wallust-${VERSION}.tar
	rm -rf wallust-${VERSION}

install: all install-completions
	mkdir -p ${DESTDIR}${PREFIX}/bin
	cp -f ${RELEASE}/wallust ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/wallust
	mkdir -p ${DESTDIR}${MANPREFIX}/man1
	mkdir -p ${DESTDIR}${MANPREFIX}/man5
	cp -f man/wallust.1 man/wallust-theme.1 man/wallust-cs.1 $(DESTDIR)$(MANPREFIX)/man1
	cp -f man/wallust.5 $(DESTDIR)$(MANPREFIX)/man5
	chmod 644 ${DESTDIR}${MANPREFIX}/man1/wallust.1 \
		${DESTDIR}${MANPREFIX}/man1/wallust-theme.1 \
		${DESTDIR}${MANPREFIX}/man1/wallust-cs.1 \
		${DESTDIR}${MANPREFIX}/man5/wallust.5

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/wallust \
		${DESTDIR}${MANPREFIX}/man1/wallust.1 \
		${DESTDIR}${MANPREFIX}/man1/wallust-theme.1 \
		${DESTDIR}${MANPREFIX}/man1/wallust-cs.1 \
		${DESTDIR}${MANPREFIX}/man5/wallust.5 \
		${ZSHPREFIX}/_wallust \
		${BASHPREFIX}/wallust.bash \
		${BASHPREFIX}/wallust.fish
pkg-nix:
	@${CARGO} build --release --target ${NIX}
	cp -f target/${NIX}/release/wallust wallust
	tar czvf wallust-${VERSION}-${NIX}.tar.gz wallust
	rm -f wallust

pkg-win: ## can't be generalized out because of the .exe
	@${CARGO} build --release --target ${WIN}
	cp -f target/${WIN}/release/wallust.exe wallust.exe
	tar czvf wallust-${VERSION}-${WIN}.tar.gz wallust.exe
	rm -f wallust.exe

mostlyclean:
	rm -f ${RELEASE}/wallust

clean:
	@${CARGO} clean

.PHONY: all clean mostlyclean dist install uninstall install-completions completions
