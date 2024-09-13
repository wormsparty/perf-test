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

bool api_list(Document& request, Document& response, string connection_string) {
    std::vector<string> prms;
    std::stringstream query;

    connection cx(connection_string);

    if (!cx.is_open()) {
        cerr << "Can't open database" << endl;
        return false;
    }

    work tx(cx);

    query << "SELECT *, COUNT(*) OVER() AS total "
          << "FROM entity ";

    if (!filter_sort_and_page(query, prms, request, tx, global_searchable_fields)) {
        return false;
    }

    Value rows(kArrayType);

    result r = tx.exec_params(query.str(), prepare::make_dynamic_params(prms));
    auto line = r.begin();
    int total = 0;
    std::vector<std::string> column_to_camel;

    if (line != r.end()) {
        total = line["total"].as<int>();

        for (auto const &field: line) {
            column_to_camel.push_back(snake_to_camel(field.name()));
        }

        for (; line != r.end(); line++) {
            Value row(kObjectType);
            int i = 0;

            for (auto const &field: line) {
                Value key(column_to_camel[i++].c_str(), response.GetAllocator());
                Value val;
                val.SetString(StringRef(strdup(field.c_str())));
                row.AddMember(key, val, response.GetAllocator());
            }

            rows.PushBack(row, response.GetAllocator());
        }
    }

    tx.commit();
    response.SetObject();

    Value data_key("data", response.GetAllocator());
    response.AddMember("data", rows, response.GetAllocator());

    Value total_key("total", response.GetAllocator());
    Value total_val(total);
    response.AddMember(total_key, total_val, response.GetAllocator());

    return true;
}

