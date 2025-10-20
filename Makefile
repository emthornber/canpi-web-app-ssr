################################################################################
#
#   18 April, 2024 - E M Thornber
#   Created
#
################################################################################

export SDIR := ${shell pwd}
export BFILE := "$(SDIR)/target/release/canpi-ssr"
export ODIR := "$(SDIR)/package"
export PKGNAME := ${shell python3 extract_value_from_toml_file.py -k name}
export VERS := ${shell python3 extract_value_from_toml_file.py -k version}

all: clean package

.PHONY: all build clean documents manpages release test package

build:
	cargo build

clean:
	cargo clean

documents: \
	changelog.Debian.gz

changelog.Debian.gz: CHANGES.md
	gzip -c $< > $@
 
manpages:
	( cd $@ ; $(MAKE) clean ; $(MAKE) all )

package: release documents manpages
	$(MAKE) -f $@/Makefile pkgs

release:
	cargo build --release

test:
	cargo test -- --test-threads=1