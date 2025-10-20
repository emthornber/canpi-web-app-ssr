################################################################################
#
#	9th October 2025 - E M Thornber
#	Created
#
################################################################################

GZIP := ${shell which gzip} -9n
POD2MAN := ${shell which pod2man} --center "MERG CANPiCap Documentation"

section5 = canpi-ssr.conf.5.gz
section8 = canpi-ssr.8.gz

all: $(section5) $(section8)

.PHONY: all clean

clean:
	$(RM) *.gz

$(section5): %.gz: %.pod
	$(POD2MAN) --section "5" $< | $(GZIP) > $@

$(section8): %.gz: %.pod
	$(POD2MAN) --section "8" $< | $(GZIP) > $@