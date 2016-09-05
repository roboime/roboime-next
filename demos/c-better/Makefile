MAIN = demo
OBJS = main.o
CFLAGS = -O2 -std=c99
LDFLAGS = $(CFLAGS) -lm
CC = clang

.PHONY: all clean

all: $(MAIN)

run: $(MAIN)
	cargo run --bin roboime-next-gui -- -pvv --blue=./$(MAIN)

clean:
	rm -f $(MAIN) $(OBJS)

$(MAIN): $(OBJS)
	$(CC) -o $@ $^ $(LDFLAGS)

%.o: %.c
	$(CC) -c $< $(CFLAGS)
