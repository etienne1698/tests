cmake_minimum_required(VERSION 3.16)
project(Engine3D)

# Définir le standard C++
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Chemins vers Dawn
set(DAWN_DIR ${CMAKE_SOURCE_DIR}/third_party/dawn)
set(DAWN_INCLUDE_DIR ${DAWN_DIR}/include)
set(DAWN_LIB_DIR ${DAWN_DIR}/out/Release)

# Ajouter les fichiers source
add_executable(Engine3D src/main.cpp)

# Inclure les fichiers headers de Dawn
target_include_directories(Engine3D PRIVATE ${DAWN_INCLUDE_DIR})

# Lier les bibliothèques de Dawn
target_link_directories(Engine3D PRIVATE ${DAWN_LIB_DIR})
target_link_libraries(Engine3D PRIVATE
    dawn
    dawn_proc
)

# Options pour le compilateur
if (MSVC)
    target_compile_options(Engine3D PRIVATE /W4)
else()
    target_compile_options(Engine3D PRIVATE -Wall -Wextra -pedantic)
endif()
