################################################################################
#
#   18 April, 2024 - E M Thornber
#   Created
#
#   04 May, 2024 - E M Thornber
#   Streamlined targets
#
#	11 October, 2024 - E M Thornber
#	Use version string
#
#	21 February, 2025 - E M Thornber
#	Added package name EV
#	Read package architecture from build system
#
#	EVs required
#	BFILE - path to canpi executable
#	ODIR  - where to place the .deb file
#   PKGNAME - package name
#	SDIR  - Current source directory
#	VERS  - package version
#
################################################################################

ARCH := $(shell dpkg-architecture --query DEB_HOST_ARCH)
EPM := $(shell which epm)
PKGDEFN := $(SDIR)/package/$(PKGNAME).epm
RM=/bin/rm -f
TS=`date "+%y%m%d-%H%M"`

all: pkgs

pkgs: portable deb

deb portable :
	sudo $(RM) $(ODIR)/$@-*.log
	sudo $(EPM) -a $(ARCH) -f $@ -vv --output-dir $(ODIR) binary=$(BFILE) srcdir=$(SDIR) version=$(VERS) $(PKGNAME) $(PKGDEFN) 2>&1 | tee $(ODIR)/$@-$(TS).log
