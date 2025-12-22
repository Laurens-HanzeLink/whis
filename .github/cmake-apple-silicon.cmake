# CMake toolchain for Apple Silicon builds on GitHub Actions
#
# Problem: GitHub Actions Apple Silicon runners have a CPU detection mismatch.
# The cmake runtime test `GGML_MACHINE_SUPPORTS_i8mm` fails (CPU doesn't support i8mm),
# but cmake still enables `GGML_CPU_ARM_MATMUL_INT8` based on compile-time detection.
# This causes a build error:
#   "always_inline function 'vmmlaq_s32' requires target feature 'i8mm',
#    but would be inlined into function that is compiled without support for 'i8mm'"
#
# Solution: Disable native CPU detection entirely, which prevents the mismatch.
# This produces slightly less optimized binaries (no CPU-specific optimizations)
# but ensures consistent builds across all Apple Silicon machines.
#
# This file is referenced via CMAKE_TOOLCHAIN_FILE env var in release.yml.
# See: https://github.com/ggerganov/whisper.cpp/issues/2292 (related)
#
# To investigate if this is still needed:
#   1. Check if GitHub updates their Apple Silicon runner CPUs
#   2. Check if whisper.cpp/ggml fixes the detection logic

set(GGML_NATIVE OFF CACHE BOOL "Disable native CPU detection" FORCE)
set(GGML_CPU_ARM_MATMUL_INT8 OFF CACHE BOOL "Disable i8mm instructions" FORCE)
