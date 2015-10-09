cmake_minimum_required(VERSION 2.8.12)

include(ExternalProject)
ExternalProject_add(chemfiles
    PREFIX chemfiles
    URL file://${CMAKE_CURRENT_SOURCE_DIR}/chemfiles-c26920.tar.gz
    CMAKE_ARGS  -DBUILD_SHARED_LIBS=OFF
                -DCMAKE_POSITION_INDEPENDENT_CODE=ON
                -DBUILD_FRONTEND=OFF
                -DENABLE_NETCDF=OFF
                -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
                # Setting compilers
                -DCMAKE_C_COMPILER=${CMAKE_C_COMPILER}
                -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
                # Setting installation directory
                -DCMAKE_INSTALL_PREFIX=${PROJECT_BINARY_DIR}
                # Support cross-compilation
                -DCMAKE_SYSTEM_NAME=${CMAKE_SYSTEM_NAME}
                -DCMAKE_FIND_ROOT_PATH=${CMAKE_FIND_ROOT_PATH}
                -DCMAKE_FIND_ROOT_PATH_MODE_LIBRARY=${CMAKE_FIND_ROOT_PATH_MODE_LIBRARY}
                -DCMAKE_FIND_ROOT_PATH_MODE_INCLUDE=${CMAKE_FIND_ROOT_PATH_MODE_INCLUDE}
                -DCMAKE_FIND_ROOT_PATH_MODE_PROGRAM=${CMAKE_FIND_ROOT_PATH_MODE_PROGRAM}
)

file(GLOB libraries ${PROJECT_BINARY_DIR}/lib/*.${CMAKE_STATIC_LIBRARY_SUFFIX})
install(FILES ${libraries} DESTINATION lib)


file(REMOVE ${PROJECT_BINARY_DIR}/cxx_link_libs.cmake)
file(REMOVE ${PROJECT_BINARY_DIR}/cxx_link_dirs.cmake)

foreach(lib ${CMAKE_CXX_IMPLICIT_LINK_LIBRARIES})
    file(APPEND ${PROJECT_BINARY_DIR}/cxx_link_libs.cmake "${lib}\n")
endforeach()
foreach(dir ${CMAKE_CXX_IMPLICIT_LINK_DIRECTORIES})
    file(APPEND ${PROJECT_BINARY_DIR}/cxx_link_dirs.cmake "${dir}\n")
endforeach()
