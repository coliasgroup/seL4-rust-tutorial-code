#
# Copyright 2025, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../../mk/microkit.mk

.PHONY: none
none:

.PHONY: rustdoc
rustdoc:
	SEL4_INCLUDE_DIRS=$(sel4_include_dirs) \
		cargo doc --target-dir $(TARGET_DIR)
