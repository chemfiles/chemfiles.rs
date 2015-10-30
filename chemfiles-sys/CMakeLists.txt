add_subdirectory(chemfiles)

# Get default C++ libraries for linkage
file(REMOVE ${PROJECT_BINARY_DIR}/cxx_link_libs.cmake)
file(REMOVE ${PROJECT_BINARY_DIR}/cxx_link_dirs.cmake)

foreach(lib ${CMAKE_CXX_IMPLICIT_LINK_LIBRARIES})
    file(APPEND ${PROJECT_BINARY_DIR}/cxx_link_libs.cmake "${lib}\n")
endforeach()
foreach(dir ${CMAKE_CXX_IMPLICIT_LINK_DIRECTORIES})
    file(APPEND ${PROJECT_BINARY_DIR}/cxx_link_dirs.cmake "${dir}\n")
endforeach()
