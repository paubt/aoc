//
// Created by user on 12/16/22.
//
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <cassert>
#include <numeric>


int main() {
    // Part 1.

    {
        std::ifstream infile("../../data/day05/input.txt");
        std::string line;
        // if test
        // std::vector<std::vector<char>> cargo_stacks {{'Z', 'N'}, {'M','C','D'},{'P'}};
        // if real
        std::vector<std::vector<char>> cargo_stacks {{'R','N','F','V','L','J','S','M'},
                                                     {'P','N','D','Z','F','J','W','H'},
                                                     {'W','R','C','D','G'},
                                                     {'N','B','S'},
                                                     {'M','Z','W','P','C','B','F','M'},
                                                     {'P','R','M','W'},
                                                     {'R','T','N','G','L','S','W'},
                                                     {'Q','T','H','F','N','B','V'},
                                                     {'L','M','H','Z','N','F'}};
        while (std::getline(infile, line)) {
            auto iss = std::istringstream(line);
            std::string temp_s;
            std::vector<std::string> inst;
            while (std::getline(iss, temp_s, ' '))
                inst.push_back(temp_s);
            for (int i = 0; i < std::stoi(inst.at(1)); i++)
            {
                assert(cargo_stacks.at(std::stoi(inst.at(3)) - 1).size() > 0);
                const char ele = cargo_stacks.at(std::stoi(inst.at(3)) - 1).back();
                cargo_stacks.at(std::stoi(inst.at(3)) - 1).pop_back();
                cargo_stacks.at(std::stoi(inst.at(5)) - 1).push_back(ele);
            }
        }
        std::cout << "part 1 solution: ";
        std::for_each(cargo_stacks.cbegin(), cargo_stacks.cend(),
                      [](auto &v){std::cout << v.back();});
        std::cout << std::endl;
    }
    // Part 2.
    {
        // if test
        // std::vector<std::vector<char>> cargo_stacks {{'Z', 'N'}, {'M','C','D'},{'P'}};
        // if real
        std::vector<std::vector<char>> cargo_stacks {{'R','N','F','V','L','J','S','M'},
                                                     {'P','N','D','Z','F','J','W','H'},
                                                     {'W','R','C','D','G'},
                                                     {'N','B','S'},
                                                     {'M','Z','W','P','C','B','F','M'},
                                                     {'P','R','M','W'},
                                                     {'R','T','N','G','L','S','W'},
                                                     {'Q','T','H','F','N','B','V'},
                                                     {'L','M','H','Z','N','F'}};
        std::ifstream infile("../../data/day05/input.txt");
        std::string line;
        while (std::getline(infile, line)) {
            auto iss = std::istringstream(line);
            std::string temp_s;
            std::vector<std::string> inst;
            while (std::getline(iss, temp_s, ' '))
                inst.push_back(temp_s);
            int move_amount = std::stoi(inst.at(1));
            int from_idx = std::stoi(inst.at(3))-1;
            auto &from_vec = cargo_stacks.at(from_idx);
            int to_idx = std::stoi(inst.at(5))-1;
            auto &to_vec = cargo_stacks.at(to_idx);
            if (move_amount <= cargo_stacks.at(from_idx).size())
                auto t = 0;
            auto it = std::next(from_vec.rbegin(), move_amount);
            std::move(from_vec.end() - move_amount,
                      from_vec.end(),
                      std::back_inserter(to_vec));
            from_vec.resize(from_vec.size() - move_amount);
        }
        std::cout << "part 2 solution: ";
        std::for_each(cargo_stacks.cbegin(), cargo_stacks.cend(),
                      [](auto &v){std::cout << v.back();});
        std::cout << std::endl;
    }
    return 0;
}

