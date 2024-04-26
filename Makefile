################################################################################
#
#   18 April, 2024 - E M Thornber
#   Created
#
################################################################################

export SDIR := ${shell pwd}
export BFILE := "$(SDIR)/target/arm-unknown-linux-gnueabihf/release/canpi-ssr"
export ODIR := "$(SDIR)/package"

SUBDIRS = package

all: clean package

.PHONY: all build clean release test $(SUBDIRS)

build:
	cargo build

clean:
	cargo clean

$(SUBDIRS):
	$(MAKE) -f $@/Makefile

package: release

release:
	cargo build --release

test:
	cargo test