#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

build_dir := build

.PHONY: none
none:

$(build_dir):
	mkdir -p $(build_dir)

workspaces := \
	root-task \
	microkit

examples := \
    root-task/hello-world \
    root-task/kernel-objects \
    root-task/address-space \
    root-task/serial-device \
    root-task/spawn-thread \
    root-task/spawn-task \
    microkit/hello-world \
    microkit/ipc \
    microkit/shared-memory

.PHONY: clean-each-example test-each-example
clean-each-example test-each-example:
	set -eu; \
	$(foreach example,$(examples), \
		$(MAKE) -C workspaces/$(example) $(subst -each-example,,$@);)

.PHONY: clean
clean:
	rm -rf $(build_dir)

.PHONY: clean-all
clean-all: clean clean-each-example

.PHONY: test
test: test-each-example

.PHONY: fmt
fmt:
	set -eu; \
	$(foreach workspace,$(workspaces), \
		(cd workspaces/$(workspace) && cargo fmt);)

.PHONY: fmt-check
fmt-check:
	set -eu; \
	$(foreach workspace,$(workspaces), \
		(cd workspaces/$(workspace) && cargo fmt --check);)

.PHONY: update
update:
	set -eu; \
	$(foreach workspace,$(workspaces), \
		(cd workspaces/$(workspace) && cargo update);)

.PHONY: check-step
check-step: fmt-check test

rustdoc_dir := $(build_dir)/rustdoc

.PHONY: rustdoc
rustdoc: | $(build_dir)
	set -eu; \
	$(foreach workspace,$(workspaces), \
		$(MAKE) -C workspaces/$(workspace) $@ TARGET_DIR=$(abspath $(rustdoc_dir)/$(workspace);))

.PHONY: prune-rustdoc
prune-rustdoc:
	set -eu; \
	cd $(rustdoc_dir); \
	rm -rf */debug */*/debug

exported_rustdoc_dir := $(build_dir)/exported-rustdoc

.PHONY: exported-rustdoc
exported-rustdoc: rustdoc | $(build_dir)
	rsync -a --delete $(rustdoc_dir)/ $(exported_rustdoc_dir)/ \
		--exclude '/*/debug' \
		--exclude '/*/*/debug' \
		--exclude '/*/.*.json' \
		--exclude '/*/CACHEDIR.TAG'

.PHONY: clean-rustdoc
clean-rustdoc:
	rm -rf $(rustdoc_dir) $(exported_rustdoc_dir)
