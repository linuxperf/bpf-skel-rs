BCC_LIBBPF_TOOLS := ../../third_party/bcc/libbpf-tools
OUTPUT := $(CURDIR)/.output

ARCH_INCLUDE := /usr/include/$(shell gcc -print-multiarch)

BCC_BPFCFLAGS := -g -O2 -Wall -Werror=undef -I$(ARCH_INCLUDE)

BPF_SRCS := $(notdir $(wildcard $(BCC_LIBBPF_TOOLS)/*.bpf.c))
BPF_OBJS := $(addprefix $(OUTPUT)/,$(BPF_SRCS:.bpf.c=.bpf.o))

.PHONY: all
all: $(BPF_OBJS)

$(OUTPUT)/%.bpf.o:
	$(MAKE) -C $(BCC_LIBBPF_TOOLS) \
		OUTPUT=$(OUTPUT) \
		BPFCFLAGS="$(BCC_BPFCFLAGS)" \
		$@

clean:
	rm -rf $(OUTPUT)
