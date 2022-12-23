//
// Created by user on 12/16/22.
//
#include <iostream>
#include <fstream>

int main() {
    // Part 1.
    {
        std::ifstream infile("../../data/day04/input.txt");
         std::string line;
        int count_fc  = 0;
        while (std::getline(infile, line)) {
            auto elv_a = line.substr(0, line.find(','));
            auto elv_b = line.substr(line.find(',') + 1);
            auto elv_a_start = std::stoi(elv_a.substr(0, elv_a.find('-')));
            auto elv_a_end = std::stoi(elv_a.substr(elv_a.find('-') + 1));
            auto elv_b_start = std::stoi(elv_b.substr(0, elv_b.find('-')));
            auto elv_b_end = std::stoi(elv_b.substr(elv_b.find('-') + 1));
            if (elv_a_start == elv_b_start ||
                elv_a_end == elv_b_end ||
                (elv_a_start < elv_b_start && elv_a_end > elv_b_end) ||
                (elv_b_start < elv_a_start && elv_b_end > elv_a_end))
                count_fc++;
        }
        std::cout << "part 1 solution: " << count_fc << std::endl;
    }
    // Part 2.
    {
        std::ifstream infile("../../data/day04/input.txt");
        std::string line;
        int count_fc  = 0;
        while (std::getline(infile, line)) {
            auto elv_a = line.substr(0, line.find(','));
            auto elv_b = line.substr(line.find(',') + 1);
            auto elv_a_start = std::stoi(elv_a.substr(0, elv_a.find('-')));
            auto elv_a_end = std::stoi(elv_a.substr(elv_a.find('-') + 1));
            auto elv_b_start = std::stoi(elv_b.substr(0, elv_b.find('-')));
            auto elv_b_end = std::stoi(elv_b.substr(elv_b.find('-') + 1));
            if (elv_a_start == elv_b_start ||
                elv_a_end == elv_b_end ||
                (elv_a_start < elv_b_start && elv_a_end >= elv_b_start) ||
                (elv_b_start < elv_a_start && elv_b_end >= elv_a_start))
                count_fc++;
        }
        std::cout << "part 2 solution: "  << count_fc<< std::endl;
    }
    return 0;
}

