#include <iostream>
#include <vector>
#include <sstream>
#include <list>

#include <rapidjson/rapidjson.h>
#include <pqxx/pqxx>

#include "../include/api.hpp"
#include "../include/utils.hpp"
#include "../include/filter.hpp"

using namespace rapidjson;
using namespace std;
using namespace pqxx;

list<string> global_searchable_fields = {
    "colonne_1",
    "colonne_2",
};

std::string api_list(Document& request, string connection_string) {
    std::vector<string> prms;
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

    result r = tx.exec_params(query.str(), prepare::make_dynamic_params(prms));
    auto line = r.begin();
    int total = 0;

    stringstream response {};
    response << "{\"data\":[";

    if (line != r.end()) {
        total = line["total"].as<int>();

        for (; line != r.end(); line++) {
            response << "{";

            for (auto const &field: line) {
                if (strcmp(field.name(), "total") != 0) {
                    response << "\"" << field.name() << "\":";

                    if (strcmp(field.name(), "id") != 0) {
                        response << "\"" << field.c_str() << "\",";
                    } else {
                        response << field.c_str() << ",";
                    }
                }
            }

            // To back 1 char to remove the last comma
            response.seekp(-1, response.cur);
            response << "},";            
        }
        
        // To back 1 char to remove the last comma
        response.seekp(-1, response.cur);
    }

    tx.commit();

    response << "],\"total\":" << total << "}";
    return response.str();
}

