cmake_minimum_required(VERSION 3.31)
project(arieo_script_test_app LANGUAGES NONE)

# Allow override via command line, otherwise use default
if(NOT DEFINED ARIEO_APPS_OUTPUT_DIRECTORY)
    set(ARIEO_APPS_OUTPUT_DIRECTORY "$ENV{VS_WORKSPACE}/_output/applications")
endif()

# Get build configuration (default to Debug if not set)
if(NOT DEFINED ARIEO_BUILD_CONFIG)
    set(ARIEO_BUILD_CONFIG "Debug")
endif()

include(${CMAKE_CURRENT_LIST_DIR}/../cmake/application_project.cmake)

add_custom_target(ArieoApplications ALL)

arieo_application_project(
    arieo_script_test_app
    CONTENT_FOLDER
        ${CMAKE_CURRENT_SOURCE_DIR}/content
    SCRIPT_FOLDER
        ${CMAKE_CURRENT_SOURCE_DIR}/scripts
    MANIFEST_FILE
        ${CMAKE_CURRENT_SOURCE_DIR}/manifest/app.manifest.yaml
)