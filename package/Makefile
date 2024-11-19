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
#   19 November, 2024 - E M Thornber
#   Read package architecture from host system
#
#	EVs required
#	ARCH  - Debian package architecture (armhf, arm64)
#	BFILE - path to canpi executable
#	ODIR  - where to place the .deb file
#	SDIR  - Current source directory
#	VERS  - package version
#
################################################################################

EPM := $(shell which epm)
PKGNAME := $(shell basename $(SDIR) )
PKGDEFN := $(SDIR)/package/$(PKGNAME).epm
RM=$(shell which rm)
TS=`date "+%y%m%d-%H%M"`

all: pkgs

pkgs: portable deb

deb portable :
	sudo $(RM) -f $(ODIR)/$@-*.log
	sudo $(EPM) -a $(ARCH) -f $@ -vv --output-dir $(ODIR) binary=$(BFILE) srcdir=$(SDIR) version=$(VERS) $(PKGNAME) $(PKGDEFN) 2>&1 | tee $(ODIR)/$@-$(TS).log
