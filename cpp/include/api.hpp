#ifndef __API__
#define __API__

#include <string>
#include <rapidjson/document.h>

std::string api_list(rapidjson::Document& request, std::string connection_string);

#endif
