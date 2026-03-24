add_executable(test_esg test_esg.cpp)
target_link_libraries(test_esg PRIVATE erc8040)
add_test(NAME test_esg COMMAND test_esg)

add_executable(test_compliance test_compliance.cpp)
target_link_libraries(test_compliance PRIVATE erc8040)
add_test(NAME test_compliance COMMAND test_compliance)
