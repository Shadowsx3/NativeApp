# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html.
# For more examples on how to use CMake, see https://github.com/android/ndk-samples.

# Sets the minimum CMake version required for this project.
cmake_minimum_required(VERSION 3.22.1)

# Declares the project name. The project name can be accessed via ${ PROJECT_NAME},
# Since this is the top level CMakeLists.txt, the project name is also accessible
# with ${CMAKE_PROJECT_NAME} (both CMake variables are in-sync within the top level
# build script scope).
project("nativeapp")

# Set the path to your third_party directory
set(THIRD_PARTY_DIR ${CMAKE_SOURCE_DIR}/third_party)

# Use the actual Android ABI names directly
set(ARCH ${CMAKE_ANDROID_ARCH_ABI})

message(STATUS "Building for architecture: ${ARCH}")

# --- reset any cached NOTFOUND values (important) ---
unset(CURL_INCLUDE_DIR CACHE)
unset(OPENSSL_INCLUDE_DIR CACHE)
unset(CURL_LIBRARY CACHE)
unset(SSL_LIBRARY CACHE)
unset(CRYPTO_LIBRARY CACHE)

# --- roots based on ABI ---
set(CURL_ROOT    "${THIRD_PARTY_DIR}/build/curl/${ARCH}")
set(OPENSSL_ROOT "${THIRD_PARTY_DIR}/build/openssl/${ARCH}")

# --- expected exact locations ---
set(_CURL_INC_HINT    "${THIRD_PARTY_DIR}/curl/include/curl")
set(_OPENSSL_INC_HINT "${THIRD_PARTY_DIR}/openssl/include")
set(_CURL_LIB_DIR     "${CURL_ROOT}/lib")
set(_OPENSSL_LIB_DIR  "${OPENSSL_ROOT}/lib")

# --- diagnostics to confirm what cmake sees ---
message(STATUS "Expecting CURL include at: ${_CURL_INC_HINT}")
message(STATUS "Expecting OPENSSL include at: ${_OPENSSL_INC_HINT}")
message(STATUS "Expecting CURL lib dir at: ${_CURL_LIB_DIR}")
message(STATUS "Expecting OPENSSL lib dir at: ${_OPENSSL_LIB_DIR}")

# --- prefer exact path if present; otherwise fallback to find_* ---

# CURL headers
if(EXISTS "${_CURL_INC_HINT}/curl.h")
    set(CURL_INCLUDE_DIR "${_CURL_INC_HINT}")
else()
    find_path(CURL_INCLUDE_DIR
            NAMES curl.h
            PATHS "${CURL_ROOT}/include"
            PATH_SUFFIXES include/curl
            NO_DEFAULT_PATH
    )
endif()

# OPENSSL headers
if(EXISTS "${_OPENSSL_INC_HINT}/openssl/aes.h")
    set(OPENSSL_INCLUDE_DIR "${_OPENSSL_INC_HINT}")
else()
    find_path(OPENSSL_INCLUDE_DIR
            NAMES ssl.h
            PATHS "${OPENSSL_ROOT}/include"
            PATH_SUFFIXES openssl
            NO_DEFAULT_PATH
    )
endif()

# CURL lib
if(EXISTS "${_CURL_LIB_DIR}/libcurl.a")
    set(CURL_LIBRARY "${_CURL_LIB_DIR}/libcurl.a")
else()
    find_library(CURL_LIBRARY
            NAMES libcurl.a include/curl
            PATHS "${_CURL_LIB_DIR}"
            NO_DEFAULT_PATH
    )
endif()

# OPENSSL libs
if(EXISTS "${_OPENSSL_LIB_DIR}/libssl.a")
    set(SSL_LIBRARY "${_OPENSSL_LIB_DIR}/libssl.a")
else()
    find_library(SSL_LIBRARY
            NAMES libssl.a ssl
            PATHS "${_OPENSSL_LIB_DIR}"
            NO_DEFAULT_PATH
    )
endif()

if(EXISTS "${_OPENSSL_LIB_DIR}/libcrypto.a")
    set(CRYPTO_LIBRARY "${_OPENSSL_LIB_DIR}/libcrypto.a")
else()
    find_library(CRYPTO_LIBRARY
            NAMES libcrypto.a crypto
            PATHS "${_OPENSSL_LIB_DIR}"
            NO_DEFAULT_PATH
    )
endif()

# --- fail early with clear messages (but only after trying both methods) ---
if (NOT CURL_INCLUDE_DIR)
    message(FATAL_ERROR "curl.h not found. Checked: ${_CURL_INC_HINT} and ${CURL_ROOT}/include[/curl]")
endif()
if (NOT OPENSSL_INCLUDE_DIR)
    message(FATAL_ERROR "ssl.h not found. Checked: ${_OPENSSL_INC_HINT} and ${OPENSSL_ROOT}/include[/openssl]")
endif()
if (NOT CURL_LIBRARY)
    message(FATAL_ERROR "libcurl not found in: ${_CURL_LIB_DIR}")
endif()
if (NOT SSL_LIBRARY)
    message(FATAL_ERROR "libssl not found in: ${_OPENSSL_LIB_DIR}")
endif()
if (NOT CRYPTO_LIBRARY)
    message(FATAL_ERROR "libcrypto not found in: ${_OPENSSL_LIB_DIR}")
endif()

# --- include parents (…/include), not the leaf subdir ---
get_filename_component(CURL_INCLUDE_PARENT    "${CURL_INCLUDE_DIR}"    DIRECTORY) # .../include
get_filename_component(OPENSSL_INCLUDE_PARENT "${OPENSSL_INCLUDE_DIR}" DIRECTORY) # .../include

add_library(${CMAKE_PROJECT_NAME} SHARED
        # List C/C++ source files with relative paths to this CMakeLists.txt.
        native-lib.cpp)

target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE
        "${CURL_INCLUDE_PARENT}"
        "${OPENSSL_INCLUDE_PARENT}"
)


# Add compiler definitions that might be needed for curl
target_compile_definitions(${CMAKE_PROJECT_NAME} PRIVATE
        CURL_STATICLIB  # Important when linking curl statically
)

# Specifies libraries CMake should link to your target library.
target_link_libraries(${CMAKE_PROJECT_NAME}
        ${CURL_LIBRARY}
        ${SSL_LIBRARY}
        ${CRYPTO_LIBRARY}
        android
        log
        z  # zlib for compression
)

# Optional: Set C++ standard if needed
set_target_properties(${CMAKE_PROJECT_NAME} PROPERTIES
        CXX_STANDARD 17
        CXX_STANDARD_REQUIRED ON
)