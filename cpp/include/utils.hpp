#ifndef __UTILS__
#define __UTILS__

#include <string>
#include <set>

std::string camel_to_snake(const std::string& camelCaseStr);
std::string snake_to_camel(const std::string& snake_case_str);
bool is_valid_col_name(const std::string& str);
std::string to_lower_case(const std::string& str);

#endif