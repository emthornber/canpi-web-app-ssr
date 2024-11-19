################################################################################
#
#   19 November, 2024 - E M Thornber
#
################################################################################

LN := $(shell which ln)
RM := $(shell which rm)

all: set64

set64:
	( cd .cargo ; $(RM) -f config.toml ; $(LN) -s config64.toml config.toml )
