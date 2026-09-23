# linear -- build & install following the GNU PREFIX/DESTDIR convention.
#
#   make build                       # compile release binary
#   make install                     # install to ~/.local (PREFIX)
#   make install PREFIX=/usr/local   # or anywhere
#
# The dotfiles tool-builder invokes `make install PREFIX="$HOME/.local"`.

PREFIX   ?= $(HOME)/.local
DESTDIR  ?=
CARGO    ?= cargo
FEATURES ?= secure-storage

BINDIR   := $(DESTDIR)$(PREFIX)/bin

.PHONY: all build install uninstall clean

all: build

build:
	$(CARGO) build --release --locked --features "$(FEATURES)"

install: build
	install -d "$(BINDIR)"
	install -m 0755 target/release/linear "$(BINDIR)/linear"
	@echo "installed linear -> $(BINDIR)/linear"

uninstall:
	rm -f "$(BINDIR)/linear"

clean:
	$(CARGO) clean
