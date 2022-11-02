# compilers and constant flags
CC = g++
CFLAGS = -Wall -Werror -Wextra -Wpedantic -Wvla -Wextra-semi -Wnull-dereference -Wsuggest-override -std=c++17
# constants
SRCDIR = src
TESTDIR = test
EXT = cpp

# dependency
DEPENDENCY =
INC = -I include $(DEPENDENCY)

BASE_BUILDDIR = build
BASE_TARGETDIR = bin
TARGETDIR = $(BASE_TARGETDIR)/release
BUILDDIR = $(BASE_BUILDDIR)/release
CDFLAGS = -O2
LDFLAGS =

# debug mode
ifeq ($(debug), 1)
	TARGETDIR = $(BASE_TARGETDIR)/debug
	BUILDDIR = $(BASE_BUILDDIR)/debug
	CDFLAGS = -Og -g -fsanitize=address,undefined
	LDFLAGS = -fsanitize=address,undefined
endif


SOURCES = $(wildcard $(SRCDIR)/*.$(EXT) $(TESTDIR)/*.$(EXT))
SRC_OBJECTS = $(patsubst $(SRCDIR)/%, $(BUILDDIR)/%, $(SOURCES:.$(EXT)=.o))
TEST_OBJECTS = $(patsubst $(TESTDIR)/%, $(BUILDDIR)/%, $(SRC_OBJECTS))
OBJECTS = $(TEST_OBJECTS)


all: compile link


compile:
	@echo "===> Compiling"

define COMPILE_COMMAND
@mkdir -p $(BUILDDIR)
$(CC) -o $@ -c $< $(CFLAGS) $(CDFLAGS) $(INC) -MMD -MP
endef

# Add header dependency
HEADER_DEPEND := $(patsubst %.o,%.d,$(OBJECTS))
-include $(HEADER_DEPEND)

$(BUILDDIR)/%.o: $(SRCDIR)/%.$(EXT) makefile
	$(COMPILE_COMMAND)

$(BUILDDIR)/%.o: $(TESTDIR)/%.$(EXT) makefile
	$(COMPILE_COMMAND)


link: $(OBJECTS)
	@echo "===> Linking"
	@mkdir -p $(TARGETDIR)
	$(eval MAINOBJECTS = $(shell nm -A $(OBJECTS) | grep 'T main\|T _main' | cut -d ':' -f1))
	@$(foreach MAIN, $(MAINOBJECTS), \
		$(eval TARGET = $(subst $(BUILDDIR), $(TARGETDIR), $(MAIN))) \
		$(eval LINK = $(filter-out $(MAINOBJECTS), $(OBJECTS))) \
		echo "$(CC) -o $(TARGET:.o=) $(MAIN) $(LINK) $(LDFLAGS) $(DEPENDENCY)" ; \
		$(CC) -o $(TARGET:.o=) $(MAIN) $(LINK) $(LDFLAGS) $(DEPENDENCY); \
	)


clean:
	@echo "===> Cleaning"
	@$(RM) -r $(BASE_BUILDDIR) $(BASE_TARGETDIR)


run:
	@$(foreach file, $(wildcard $(TARGETDIR)/*), ./$(file);)


valgrind:
	@$(foreach file, $(wildcard $(TARGETDIR)/*), valgrind ./$(file);)


leaks:
	@$(foreach file, $(wildcard $(TARGETDIR)/*),  leaks -atExit -- ./$(file);)


sanitizer:
	@$(foreach file, $(wildcard $(TARGETDIR)/*),  ./$(file);)


.PHONY: clean run valgrind leaks sanitizer