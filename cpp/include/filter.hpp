#ifndef __FILTER__
#define __FILTER__

#include <vector>
#include <sstream>
#include <list>

#include <pqxx/pqxx>
#include "crow_all.h"

bool filter_sort_and_page(std::stringstream& query, pqxx::params& prms,
                          crow::json::rvalue& request, pqxx::work& tx,
                          std::list<std::string>& globalSearchableFields);

#endif
