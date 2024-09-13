#ifndef __API__
#define __API__

#include <rapidjson/document.h>

bool api_list(rapidjson::Document& request, rapidjson::Document& response, std::string connection_string);

#endif
