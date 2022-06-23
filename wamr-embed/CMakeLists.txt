
cmake_minimum_required (VERSION 3.5)

project (wamr)

# Reset default linker flags
set (CMAKE_SHARED_LIBRARY_LINK_C_FLAGS "")
set (CMAKE_SHARED_LIBRARY_LINK_CXX_FLAGS "")

set(CMAKE_CXX_STANDARD 14)
set (CMAKE_SHARED_LINKER_FLAGS "-Wl,-U,_get_ext_lib_export_apis")
set (CMAKE_MACOSX_RPATH True)

set (WAMR_BUILD_PLATFORM "darwin")
set (WAMR_BUILD_TARGET "X86_64")
set (WAMR_BUILD_INTERP 0)
set (WAMR_BUILD_FAST_INTERP 1)
set (WAMR_BUILD_AOT 1)
set (WAMR_BUILD_LIBC_BUILTIN 0)
set (WAMR_BUILD_LIBC_WASI 1)
set (WAMR_BUILD_SIMD 1)
set (WAMR_ROOT_DIR ${CMAKE_CURRENT_SOURCE_DIR}/wasm-micro-runtime)


include (${WAMR_ROOT_DIR}/build-scripts/runtime_lib.cmake)

include_directories(${CMAKE_CURRENT_SOURCE_DIR})

add_library(${CMAKE_PROJECT_NAME} STATIC ${WAMR_RUNTIME_LIB_SOURCE} wamr_runtime_wrap.c)

install (TARGETS ${CMAKE_PROJECT_NAME} DESTINATION .)
