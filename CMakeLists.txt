cmake_minimum_required(VERSION 3.31)
project(arieo_script_test_app LANGUAGES NONE)

include($ENV{ARIEO_BUILDENV_PACKAGE_INSTALL_FOLDER}/cmake/application/application_project.cmake)

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