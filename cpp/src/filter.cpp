#include <iostream>
#include <vector>
#include <sstream>
#include <list>

#include <pqxx/pqxx>

#include "../include/crow_all.h"
#include "../include/utils.hpp"
#include "../include/filter.hpp"

using namespace std;
using namespace pqxx;

bool filter_sort_and_page(stringstream& query, params& prms, crow::json::rvalue& request, work& tx, list<string>& global_searchable_fields) {
    int start(request["start"]);
    int end(request["end"]);
    std::string global_search(request["globalSearch"]);

    query << " WHERE 1 = 1 ";

    if (!global_search.empty()) {
        query << "AND (1 = 0 ";

        for (auto& field: global_searchable_fields) {
            query << "OR position($" << prms.size() + 1 << " in " << tx.esc(field) << ") > 0 ";
        }

        query << ") ";
        prms.append(global_search);
    }

    for(auto& el: request["filter"].keys()) {
        string field = el;
        auto& filter_value = request["filter"][field];

        string filter(filter_value["filter"]);
        string filter_type(filter_value["filterType"]);
        string type(filter_value["type"]);
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
            prms.append(filter);
        }
    }

    auto sort = request["sort"];

    if (sort.size() > 0) {
        auto first = sort[0];
        string col_id(first["colId"]);
        string sort_with_case(first["sort"]);
        string sort_lower(to_lower_case(sort_with_case));

        auto col_snake = tx.esc(camel_to_snake(col_id));

        if (!is_valid_col_name(col_snake)) {
            cerr << "Invalid column name: " << col_snake << endl;
            return false;
        }

        if (sort_lower != "asc" && sort_lower != "desc") {
            cerr << "Invalid sort direction" << endl;
            return false;
        }

        // We shouldn't be able to inject anything since only a-z and _ are allowed
        query << "ORDER BY " << col_snake << " " << tx.esc(sort_lower) << " ";
    }

    query << "OFFSET $" << prms.size() + 1 << " "
          << "LIMIT $" << prms.size() + 2 << " ";

    prms.append(to_string(start));
    prms.append(to_string(end - start));

    return true;
}
