#include <iostream>

#include "../include/crow_all.h"
#include "../include/api.hpp"
#include "../include/config.hpp"

int main()
{
    auto connection_string = parse_config();

    if (connection_string.empty()) {
        return 1;
    }

    crow::App<crow::CORSHandler> app;
    app.loglevel(crow::LogLevel::Warning);

    auto& cors = app.get_middleware<crow::CORSHandler>();
    cors.global()
        .headers("X-Custom-Header", "Upgrade-Insecure-Requests")
        .methods("POST"_method)
        .prefix("/").origin("localhost");
        
    CROW_ROUTE(app, "/api/list").methods("POST"_method)(
        [connection_string](const crow::request& req) {
            auto request = crow::json::load(req.body);
    
            if (!request) {
                return crow::response(crow::status::BAD_REQUEST);
            }
        
            std::string str = api_list(request, connection_string);
            return crow::response("application/json", str);
        }
    );

    app.port(8000).multithreaded().run();
}
