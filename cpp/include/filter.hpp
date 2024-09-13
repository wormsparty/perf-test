#ifndef __FILTER__
#define __FILTER__

#include <vector>
#include <sstream>
#include <list>

#include <rapidjson/document.h>
#include <pqxx/pqxx>

bool filter_sort_and_page(std::stringstream& query, std::vector<std::string>& prms,
                          rapidjson::Document& request, pqxx::work& tx,
                          std::list<std::string>& globalSearchableFields);

#endif
