MAIN = demo
OBJS = main.o
CFLAGS = -O2

.PHONY: all clean

all: $(MAIN)

run: $(MAIN)
	cargo run --bin roboime-next-gui -- -pvv ./$(MAIN)

clean:
	rm -f $(MAIN) $(OBJS)

$(MAIN): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c $<
