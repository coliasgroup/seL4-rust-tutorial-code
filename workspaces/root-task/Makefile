#
# Copyright 2025, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../../mk/root-task.mk

.PHONY: none
none:

.PHONY: rustdoc
rustdoc:
	SEL4_PREFIX=$(sel4_prefix) \
		cargo doc --target-dir $(TARGET_DIR)
