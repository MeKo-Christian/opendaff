cmake_minimum_required( VERSION 3.10 )

add_executable( VerificationTest VerificationTest.cpp )
target_link_libraries( VerificationTest DAFF )
install( TARGETS VerificationTest RUNTIME DESTINATION "bin" )
set_property( TARGET VerificationTest PROPERTY FOLDER "DAFFTests" )
