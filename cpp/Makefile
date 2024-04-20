# Compiler settings
CC = g++
CFLAGS = -std=c++20 -Wall -Wextra -pedantic
LDFLAGS =

# Source files and executable name
SRCS = $(wildcard *.cpp)
EXE = main

# Target: build and run the executable
all: build run

# Target: build the executable
build: $(SRCS)
	@echo "Building $(EXE)..."
	$(CC) $(CFLAGS) $(SRCS) -o $(EXE) $(LDFLAGS)
	@echo "Build complete."

# Target: run the executable
run:
	@echo ":--- Running $(EXE)..."
	@echo "--------------------------"
	./$(EXE)

# Target: clean up generated files
clean:
	@echo "Cleaning up..."
	rm -f $(EXE)
	@echo "Cleanup complete."

# Target: help message
help:
	@echo "Usage: make [target]"
	@echo "Targets:"
	@echo "  all         : Build and run the executable"
	@echo "  build       : Build the executable"
	@echo "  run         : Run the executable"
	@echo "  clean       : Clean up generated files"
	@echo "  help        : Display this help message"

# Makefile PHONY targets
.PHONY: all build run clean help
