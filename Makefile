################################################################################
#
#   18 April, 2024 - E M Thornber
#   Created
#
#	19 November, 2024 - E M Thornber
#	Define ARCH and BFILE taking into account the settings in config.toml
#
################################################################################

export ARCH   := $(shell python3 extract_architecture.py)
export SDIR   := ${shell pwd}
TRIPLE := $(shell python3 extract_triple.py)
export BFILE  := "$(SDIR)/target/$(TRIPLE)/release/canpi-ssr"
export ODIR   := "$(SDIR)/package"
export VERS   := $(shell python3 extract_version.py)

all: clean package

.PHONY: all build clean documents release test package

build:
	cargo build

clean:
	cargo clean

documents: \
	changelog.Debian.gz

changelog.Debian.gz: CHANGES.md
	gzip -c $< > $@
 
package: release documents
	$(MAKE) -f $@/Makefile pkgs

release:
	cargo build --release

test:
	cargo test