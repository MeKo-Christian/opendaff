cmake_minimum_required( VERSION 3.10 )

# DAFFFileBufferTest - no dependencies other than DAFF
add_executable( DAFFFileBufferTest DAFFFileBufferTest.cpp )
target_link_libraries( DAFFFileBufferTest DAFF )
install( TARGETS DAFFFileBufferTest RUNTIME DESTINATION "bin" )
set_property( TARGET DAFFFileBufferTest PROPERTY FOLDER "DAFFTests" )

# DAFFQtResourceTest - requires Qt5
find_package( Qt5 COMPONENTS Core Widgets )

if( Qt5_FOUND )
	set( CMAKE_AUTOMOC ON )

	# Qt resources
	qt5_add_resources( OPENDAFF_QT_RESOURCE_TEST_RESOURCES "DAFFQtResourceTest.qrc" OPTIONS -compress 5 )

	add_executable( DAFFQtResourceTest DAFFQtResourceTest.cpp ${OPENDAFF_QT_RESOURCE_TEST_RESOURCES} )
	target_link_libraries( DAFFQtResourceTest DAFF Qt5::Core Qt5::Widgets )
	install( TARGETS DAFFQtResourceTest RUNTIME DESTINATION "bin" )

	# Set PATH to run from Visual Studio
	if( MSVC )
		set( USER_FILE DAFFQtResourceTest.vcxproj.user )
		set( OUTPUT_PATH ${CMAKE_CURRENT_BINARY_DIR}/${USER_FILE} )
		list( APPEND DAFFUSERTEMPLATE_PATHES "${_qt5Core_install_prefix}/bin/" )
		configure_file( DAFFUserTemplate.vcxproj.user.proto ${USER_FILE} @ONLY )
	endif( MSVC )
endif( Qt5_FOUND )
