cmake_minimum_required (VERSION 2.8.11)
project (cpplib)

add_library (mighty mighty.cpp)
target_include_directories (mighty PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

install (TARGETS mighty DESTINATION .)

add_executable (legacy legacy.cpp)
target_link_libraries (legacy LINK_PUBLIC mighty)

install (TARGETS legacy DESTINATION .)
