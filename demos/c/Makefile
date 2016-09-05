MAIN = demo
OBJS = main.o
CFLAGS = -O2 -lm

.PHONY: all clean

all: $(MAIN)

clean:
	rm -f $(MAIN) $(OBJS)

$(MAIN): $(OBJS)
	$(CC) -o $@ $^ $(CFLAGS)

%.o: %.c
	$(CC) -c $< $(CFLAGS)
