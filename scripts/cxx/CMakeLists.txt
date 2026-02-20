cmake_minimum_required(VERSION 3.20)

include($ENV{ARIEO_BUILD_ENV_PACKAGES_OUTPUT_DIR}/cmake/build_environment.cmake)

# Declare this script project (name, optional alias, sources and wits)
project(arieo_application_cxx_scripts)
add_custom_target(arieo_application_cxx_scripts ALL)
arieo_script_project(
    greeter_component
    ALIAS arieo::tests::script::greeter_component
    BUILD_DIR
        ${CMAKE_CURRENT_LIST_DIR}/build
    WIT
        ${CMAKE_CURRENT_LIST_DIR}/../wit/arieo_application.wit
    WIT_WORLD
        arieo:application/application
    WIT_DEPENDENCIES
        ${CMAKE_CURRENT_LIST_DIR}/../wit/deps/arieo-module/module.wit
        ${CMAKE_CURRENT_LIST_DIR}/../wit/deps/arieo-sample/sample.wit
    WIT_GEN_DIR
        ${CMAKE_CURRENT_LIST_DIR}/_generated
    INCLUDE_FOLDERS
        ${CMAKE_CURRENT_LIST_DIR}/include
    SOURCES
        ${CMAKE_CURRENT_LIST_DIR}/src/*.cpp
)
