# Core tests (no dependencies)
add_subdirectory( "verification" )
add_subdirectory( "deserializertest" )
add_subdirectory( "tryout" )

# VTK-dependent tests
if( VTK_FOUND )
	add_subdirectory( "vtktest" )
	add_subdirectory( "vtkdaffircarpet" )
endif()

# DAFFViz-dependent tests (requires both VTK and Qt)
if( OPENDAFF_WITH_DAFFVIZ )
	add_subdirectory( "daffviztest" )
	add_subdirectory( "qtdaffviztest" )
endif()

# Qt+VTK-dependent tests
if( Qt5_FOUND AND VTK_FOUND )
	add_subdirectory( "qttest" )
	add_subdirectory( "qtvtktest" )
endif()
