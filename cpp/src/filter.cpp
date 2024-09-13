#include <iostream>
#include <vector>
#include <sstream>
#include <list>

#include <rapidjson/document.h>
#include <pqxx/pqxx>

#include "../include/utils.hpp"
#include "../include/filter.hpp"

using namespace rapidjson;
using namespace std;
using namespace pqxx;

bool filter_sort_and_page(stringstream& query, vector<string>& prms, Document& request, work& tx, list<string>& global_searchable_fields) {
    int start = request["start"].GetInt();
    int end = request["end"].GetInt();
    std::string global_search = request["globalSearch"].GetString();

    query << " WHERE 1 = 1 ";

    if (!global_search.empty()) {
        query << "AND (1 = 0 ";

        for (auto& field: global_searchable_fields) {
            query << "OR position($" << prms.size() + 1 << " in " << tx.esc(field) << ") > 0 ";
        }

        query << ") ";
        prms.push_back(global_search);
    }

    for(auto& el: request["filter"].GetObject()) {
        string field = el.name.GetString();
        auto& filter_value = el.value;

        string filter = filter_value["filter"].GetString();
        string filter_type = filter_value["filterType"].GetString();
        string type = filter_value["type"].GetString();
        auto col = tx.esc(camel_to_snake(field));

        if (!is_valid_col_name(col)) {
            cerr << "Invalid column name: " << col << endl;
            return false;
        }

        if (filter_type != "text") {
            cerr << "Unsupported filter type" << endl;
            return false;
        }

        if (type == "equals") {
            query << "AND (" << col << " = $" << prms.size() + 1 << ") ";
        } else if (type == "notEquals") {
            query << "AND (" << col << " <> $" << prms.size() + 1 << ") ";
        } else if (type == "contains") {
            query << "AND (position($" << prms.size() + 1 << " in " << col << ") > 0) ";
        } else if (type == "notContains") {
            query << "AND (position($" << prms.size() + 1 << " in " << col << ") = 0) ";
        } else if (type == "startsWith") {
            query << "AND (" << col << " LIKE $" << prms.size() + 1 << ") ";
            filter = filter + '%';
        } else if (type == "endsWith") {
            query << "AND (" << col << " LIKE $" << prms.size() + 1 << ") ";
            filter = '%' + filter;
        } else if (type == "blank") {
            query << "AND (" << col << " <> '') IS NOT TRUE ";
        } else if (type == "notBlank") {
            query << "AND (" << col << " <> '') ";
        } else {
            cerr << "Unsupported type" << endl;
            return false;
        }

        if (type != "blank" && type != "notBlank") {
            prms.push_back(filter);
        }
    }

    auto sort = request["sort"].GetArray();
    auto first_sort = sort.Begin();

    if (first_sort != sort.End()) {
        auto first = first_sort->GetObject();
        string colId = first["colId"].GetString();
        string sort = to_lower_case(first["sort"].GetString());

        auto col_snake = tx.esc(camel_to_snake(colId));

        if (!is_valid_col_name(col_snake)) {
            cerr << "Invalid column name: " << col_snake << endl;
            return false;
        }

        if (sort != "asc" && sort != "desc") {
            cerr << "Invalid sort direction" << endl;
            return false;
        }

        // We shouldn't be able to inject anything since only a-z and _ are allowed
        query << "ORDER BY " << col_snake << " " << tx.esc(sort) << " ";
    }

    query << "OFFSET $" << prms.size() + 1 << " "
          << "LIMIT $" << prms.size() + 2 << " ";

    prms.push_back(to_string(start));
    prms.push_back(to_string(end - start));

    return true;
}
