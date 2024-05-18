################################################################################
#
#   18 April, 2024 - E M Thornber
#   Created
#
#   04 May, 2024 - E M Thornber
#   Streamlined targets
#
#	EVs required
#	BFILE - path to canpi executable
#	ODIR  - where to place the .deb file
#	SDIR  - Current source directory
#
################################################################################

EPM=/usr/local/bin/epm
PKGNAME := $(shell basename $(SDIR) )
PKGDEFN := $(PKGNAME).epm
RM=/bin/rm -f
TS=`date "+%y%m%d-%H%M"`

all: pkgs

pkgs: portable deb

deb portable :
	sudo $(RM) $(ODIR)/$@-*.log
	sudo $(EPM) -a armhf -f $@ -vv --output-dir $(ODIR) binary=$(BFILE) srcdir=$(SDIR) $(PKGNAME) $(SDIR)/package/$(PKGDEFN) 2>&1 | tee $(ODIR)/$@-$(TS).log
