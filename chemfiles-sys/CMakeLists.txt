cmake_minimum_required(VERSION 2.8.12)

project(chemfiles-rust CXX)

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

set(CHFL_RUST_PREBUILT_TARGET "" CACHE STRING "Pre-built target to download instead of building the code")
set(CHFL_RUST_PREBUILT_SHA256 "" CACHE STRING "SHA256 sum of the pre-built archive to download")

if (NOT ${CHFL_RUST_PREBUILT_TARGET} STREQUAL "")
    set(PREBUILT_DOWNLOAD_SUCCESS FALSE)
    set(PREBUILT_NAME "chemfiles-static.v${CHEMFILES_VERSION}.${CHFL_RUST_PREBUILT_TARGET}.tar.gz")
    set(PREBUILT_URL "https://github.com/chemfiles/chemfiles-prebuilt/releases/download/v${CHEMFILES_VERSION}/${PREBUILT_NAME}")

    set(PREBUILT_DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/${PREBUILT_NAME})

    message(STATUS "trying to download ${PREBUILT_URL}")
    file(DOWNLOAD
        ${PREBUILT_URL} ${PREBUILT_DESTINATION}
        STATUS DOWNLOAD_STATUS
    )

    list(GET DOWNLOAD_STATUS 0 DOWNLOAD_ERROR_CODE)
    if (DOWNLOAD_ERROR_CODE)
        file(REMOVE ${PREBUILT_DESTINATION})
        message(STATUS "failed to download prebuilt chemfiles, building from sources")
    else()
        set(PREBUILT_DOWNLOAD_SUCCESS TRUE)
    endif()

    if (PREBUILT_DOWNLOAD_SUCCESS)
        file(SHA256 ${PREBUILT_DESTINATION} PREBUILT_SHA256)
        if (NOT ${PREBUILT_SHA256} STREQUAL ${CHFL_RUST_PREBUILT_SHA256})
            message(
                FATAL_ERROR
                "corrupted download of prebuilt chemfiles at '${PREBUILT_DESTINATION}':\n"
                "Expected SHA256: ${CHFL_RUST_PREBUILT_SHA256}\n"
                "Actual SHA256: ${PREBUILT_SHA256}"
            )
        endif()

        # this could use file(ARCHIVE_EXTRACT ...), but that's only available
        # for cmake >= 3.18
        execute_process(
            COMMAND ${CMAKE_COMMAND} -E tar xf ${PREBUILT_DESTINATION}
            WORKING_DIRECTORY ${CMAKE_INSTALL_PREFIX}
        )

        install(CODE "message(STATUS \"nothing to install\")")
        return()
    endif()
endif()

# otherwise, try to build from sources
set(BUILD_SHARED_LIBS OFF)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
add_subdirectory(chemfiles)
