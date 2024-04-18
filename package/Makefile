################################################################################
#
#   18 April, 2024 - E M Thornber
#   Created
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

all: clean pkgs

clean:
	sudo $(RM) $(ODIR)/*.log

pkgs: portable deb

portable:
	sudo $(EPM) -a armhf -f portable -vv --output-dir $(ODIR) binary=$(BFILE) srcdir=$(SDIR) $(PKGNAME) $(SDIR)/package/$(PKGDEFN) 2>&1 | tee $(ODIR)/portable-$(TS).log

deb:
	sudo $(EPM) -a armhf -f deb      -vv --output-dir $(ODIR) binary=$(BFILE) srcdir=$(SDIR) $(PKGNAME) $(SDIR)/package/$(PKGDEFN) 2>&1 | tee $(ODIR)/deb-$(TS).log

