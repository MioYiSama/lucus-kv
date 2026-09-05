cmake_minimum_required(VERSION 4.3)
project(lucus-kv-native)

add_library(${PROJECT_NAME} STATIC)
target_sources(${PROJECT_NAME} PRIVATE
  src/main.cpp
)

target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_23)
set_target_properties(${PROJECT_NAME} PROPERTIES
  LINKER_TYPE MOLD
  CXX_EXTENSIONS OFF
  POSITION_INDEPENDENT_CODE ON
)

target_link_libraries(${PROJECT_NAME} PRIVATE
  uring
)
