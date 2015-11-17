BIN = ./bin/
SOURCE = ./src/

PROG = e1 e2 e3 e4 e5 e6 e7 e8
LIST = $(addprefix $(BIN)/, $(PROG))

all: $(LIST)

$(BIN)/%: $(SOURCE)%.c
	gcc -o $@ $<
