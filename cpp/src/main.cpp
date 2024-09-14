#include <iostream>

#include <httplib.h>
#include <rapidjson/document.h>
#include <rapidjson/writer.h>
#include <rapidjson/stringbuffer.h>

#include "../include/api.hpp"
#include "../include/config.hpp"

using namespace rapidjson;

int main()
{
    auto connection_string = parse_config();

    if (connection_string.empty()) {
        return 1;
    }

    httplib::Server svr;

    svr.Options(R"(\*)", [](const auto&, auto& res) {
        res.set_header("Allow", "GET, POST, HEAD, OPTIONS");
        res.set_header("Access-Control-Allow-Origin", "*");
        res.set_header("Access-Control-Allow-Headers", "X-Requested-With, Content-Type, Accept, Origin, Authorization");
        res.set_header("Access-Control-Allow-Methods", "OPTIONS, GET, POST, HEAD");
    });

    svr.Post("/api/list", [&connection_string](const httplib::Request &req, httplib::Response &res) {
        try {
            Document request;
            request.Parse(req.body.c_str());

            std::string response = api_list(request, connection_string);

            if (response.empty()) {
                return;
            }

            res.set_content(response, "application/json");
            res.set_header("Access-Control-Allow-Origin", "*");
        } catch(const std::exception &exc) {
            std::cerr << "Error occurred: " << exc.what() << std::endl;
            throw;
        }
    });

    std::cout << "Listening..." << std::endl;
    svr.listen("0.0.0.0", 8000);
}
