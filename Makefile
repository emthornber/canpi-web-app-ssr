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
	cargo test -- --test-threads=1