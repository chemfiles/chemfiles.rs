cmake_minimum_required(VERSION 2.8.12)

include(ExternalProject)
ExternalProject_add(chemharp
    PREFIX chemarp
    URL file://${CMAKE_CURRENT_SOURCE_DIR}/chemharp-125dc9.tar.gz
    URL_MD5 d20812c60baf0a72cf103fe21ffe2f4c
    CMAKE_ARGS  -DBUILD_SHARED_LIBS=OFF
                -DBUILD_FRONTEND=OFF
                -DCMAKE_BUILD_TYPE=release
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

file(GLOB libraries ${PROJECT_BINARY_DIR}/lib/*)
install(FILES ${libraries} DESTINATION ${CMAKE_INSTALL_PREFIX})
