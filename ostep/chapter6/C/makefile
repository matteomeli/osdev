BIN = ./bin/
SOURCE = ./src/

PROG = e1 e2 gettimeofday syscall_test
LIST = $(addprefix $(BIN)/, $(PROG))

all: $(LIST)

$(BIN)/%: $(SOURCE)%.c
	gcc -Wextra -Wall -o $@ $<
