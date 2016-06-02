MAIN = demo
OBJS = main.o
CFLAGS = -O2

.PHONY: all clean

all: $(MAIN)

clean:
	rm -f $(MAIN) $(OBJS)

$(MAIN): $(OBJS)
	$(CXX) $(CFLAGS) -o $@ $^

%.o: %.cpp
	$(CXX) $(CFLAGS) -c $<
