#include "../include/utils.hpp"

#include <string>
#include <algorithm>
#include <set>

std::string camel_to_snake(const std::string& camelCaseStr) {
    std::string snake_case;

    for (char ch : camelCaseStr) {
        if (isupper(ch)) {
            snake_case += '_';
            snake_case += tolower(ch);
        } else {
            snake_case += ch;
        }
    }

    return snake_case;
}

std::string snake_to_camel(const std::string& snake_case_str) {
    std::string camel_case_str;
    bool to_upper = false;

    for (char ch : snake_case_str) {
        if (ch == '_') {
            to_upper = true;
        } else {
            if (to_upper) {
                camel_case_str += toupper(ch);
                to_upper = false;
            } else {
                camel_case_str += ch;
            }
        }
    }

    return camel_case_str;
}

bool is_valid_col_name(const std::string& str) {
    return std::all_of(str.begin(), str.end(), [](char ch) {
        return (ch >= 'a' && ch <= 'z') || ch == '_' || (ch >= '0' && ch <= '9');
    });
}

std::string to_lower_case(const std::string& str) {
    std::string lower_str = str;
    std::transform(lower_str.begin(), lower_str.end(), lower_str.begin(), ::tolower);
    return lower_str;
}
