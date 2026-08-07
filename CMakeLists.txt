cmake_minimum_required(VERSION 3.28)
project(ticket_native_app)

find_package(ECM 6.0 REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})
include(KDEInstallDirs)
include(ECMUninstallTarget)
include(ECMFindQmlModule)

find_program(CARGO_EXECUTABLE cargo REQUIRED)
find_program(QMAKE6_EXECUTABLE qmake6 REQUIRED)

ecm_find_qmlmodule(org.kde.kirigami REQUIRED)
find_package(KF6 REQUIRED COMPONENTS QQC2DesktopStyle)

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_BUILD_FLAGS "")
    set(CARGO_OUTPUT_DIR "debug")
else()
    set(CARGO_BUILD_FLAGS "--release")
    set(CARGO_OUTPUT_DIR "release")
endif()

add_custom_target(ticket_native_app ALL
    COMMAND ${CMAKE_COMMAND} -E env QMAKE=${QMAKE6_EXECUTABLE}
            ${CARGO_EXECUTABLE} build -p ticket-kde ${CARGO_BUILD_FLAGS}
            --target-dir ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

install(
    PROGRAMS ${CMAKE_CURRENT_BINARY_DIR}/${CARGO_OUTPUT_DIR}/admitpress
    DESTINATION ${KDE_INSTALL_BINDIR}
)

install(
    FILES packaging/fedora/app.ticket.native.desktop
    DESTINATION ${KDE_INSTALL_APPDIR}
)

install(
    FILES packaging/fedora/app.ticket.native.svg
    DESTINATION ${KDE_INSTALL_ICONDIR}/hicolor/scalable/apps
)

install(
    FILES packaging/fedora/app.ticket.native.metainfo.xml
    DESTINATION ${KDE_INSTALL_METAINFODIR}
)
