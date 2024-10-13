################################################################################
#
#   18 April, 2024 - E M Thornber
#   Created
#
################################################################################

export SDIR := ${shell pwd}
export BFILE := "$(SDIR)/target/arm-unknown-linux-gnueabihf/release/canpi-ssr"
export ODIR := "$(SDIR)/package"

all: clean package

.PHONY: all build clean release test package

build:
	cargo build

clean:
	cargo clean

package: release
	VERS=`python3 extract_version.py` $(MAKE) -f $@/Makefile pkgs

release:
	cargo build --release

test:
	cargo test