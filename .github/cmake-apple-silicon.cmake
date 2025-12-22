# CMake toolchain for Apple Silicon builds on GitHub Actions
# Disables native CPU detection to avoid i8mm issues

set(GGML_NATIVE OFF CACHE BOOL "Disable native CPU detection" FORCE)
set(GGML_CPU_ARM_MATMUL_INT8 OFF CACHE BOOL "Disable i8mm instructions" FORCE)
