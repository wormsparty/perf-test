#ifndef __API__
#define __API__

#include <string>
#include "crow_all.h"

std::string api_list(crow::json::rvalue& request, std::string connection_string);

#endif
