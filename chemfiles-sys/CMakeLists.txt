cmake_minimum_required(VERSION 2.8.11)

add_subdirectory(chemfiles)

# Get default C++ libraries to link
file(WRITE ${PROJECT_BINARY_DIR}/cxx_link_libs.cmake "")
foreach(lib ${CMAKE_CXX_IMPLICIT_LINK_LIBRARIES})
    file(APPEND ${PROJECT_BINARY_DIR}/cxx_link_libs.cmake "${lib}\n")
endforeach()

# Get default C++ search path for libraries
file(WRITE ${PROJECT_BINARY_DIR}/cxx_link_dirs.cmake "")
foreach(dir ${CMAKE_CXX_IMPLICIT_LINK_DIRECTORIES})
    file(APPEND ${PROJECT_BINARY_DIR}/cxx_link_dirs.cmake "${dir}\n")
endforeach()
