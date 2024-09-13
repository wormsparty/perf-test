
#include <string>
#include <sstream>
#include <iostream>

#include <libconfig.h++>

#include "../include/config.hpp"

using namespace std;
using namespace libconfig;

std::string parse_config() {
    try {
        Config cfg;
        cfg.readFile("config.cfg");

        string dbname = cfg.lookup("dbname");
        string user = cfg.lookup("user");
        string password = cfg.lookup("password");
        string host = cfg.lookup("host");
        string port = cfg.lookup("port");

        stringstream ss;
        ss << "dbname = " << dbname << " "
           << "user = " << user << " "
           << "password = " << password << " "
           << "host = " << host << " "
           << "port = " << port << " ";

        return ss.str();
    } catch(const FileIOException &fioex) {
        cerr << "Error while reading config.cfg file." << endl;
        return "";
    }
}
