#include <iostream>
#include <vector>
#include <sstream>
#include <list>

#include <pqxx/pqxx>

#include "../include/crow_all.h"
#include "../include/api.hpp"
#include "../include/utils.hpp"
#include "../include/filter.hpp"

using namespace std;
using namespace pqxx;

list<string> global_searchable_fields = {
    "colonne_1",
    "colonne_2",
};

std::string api_list(crow::json::rvalue& request, string connection_string) {
    params prms;
    std::stringstream query;

    connection cx(connection_string);

    if (!cx.is_open()) {
        cerr << "Can't open database" << endl;
        return "";
    }

    work tx(cx);

    query << "SELECT *, COUNT(*) OVER() AS total "
          << "FROM entity ";

    if (!filter_sort_and_page(query, prms, request, tx, global_searchable_fields)) {
        return "";
    }

    int total = 0;
    vector<crow::json::wvalue> lines{};

    auto query_result = tx.query<int, string, string, int>(query.str(), prms);
    auto it = query_result.begin();
    
    if (it != query_result.end()) {
        total = std::get<3>(*it);
    
        for (; it != query_result.end(); ++it) {
            auto&& line = *it;
            
            lines.push_back(crow::json::wvalue({
                {"id", std::get<0>(line)},
                {"colonne_1", std::get<1>(line)},
                {"colonne_2", std::get<2>(line)},
            }));
        }
    }

    crow::json::wvalue response({{"data", lines}, {"total", total}});
    tx.commit();

    return response.dump();
}

