//
// Created by user on 12/16/22.
//
#include <iostream>
#include <fstream>
#include <deque>
#include <set>

int main() {
    // Part 1.
    {
        std::ifstream infile("../../data/day06/input.txt");
        std::string line;
        std::deque<char> l3e;
        while (std::getline(infile, line))
        {
            for (int c_idx = 3; c_idx < line.size(); c_idx++) {
                if (line[c_idx] != line[c_idx - 1] && line[c_idx] != line[c_idx - 2] &&
                    line[c_idx] != line[c_idx - 3] &&
                    line[c_idx - 1] != line[c_idx - 2] && line[c_idx - 1] != line[c_idx - 3] &&
                    line[c_idx - 2] != line[c_idx - 3]) {
                    std::cout << "part 1 solution: " << ++c_idx<< std::endl;
                    break;
                }
            }
        }
        std::cout << "part 1 solution: " << std::endl;
    }
    // Part 2.
    {
        std::ifstream infile("../../data/day06/input.txt");
        std::string line;
        std::deque<char> l14e;
        while (std::getline(infile, line)) {
            l14e.clear();
            unsigned int curr_idx = 0;
            for (auto c: line)
            {
                curr_idx++;
                l14e.push_front(c);
                if (l14e.size() < 14)
                    continue;
                if (l14e.size() > 14)
                    l14e.pop_back();
                if (std::set<char>(l14e.begin(), l14e.end()).size() == 14)
                {
                    std::cout << "part 2 solution: "  << curr_idx<< std::endl;
                    break;
                }
            }
        }
    }
    return 0;
}

