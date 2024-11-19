################################################################################
#
#   19 November, 2024 - E M Thornber
#
################################################################################

LN := $(shell which ln)
RM := $(shell which rm)

all: set32

set32:
	( cd .cargo ; $(RM) -f config.toml ; $(LN) -s config32.toml config.toml )
