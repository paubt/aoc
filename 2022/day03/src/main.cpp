//
// Created by user on 12/16/22.
//

#include <algorithm>
#include <iostream>
#include <fstream>
#include <set>
#include <assert.h>

int main() {
    // Part 1.
    {
        std::ifstream infile("../../data/day03/input.txt");
        std::string line;
        std::set<char> present_in_part_one{};
        int sum = 0;
        while (std::getline(infile, line))
        {
            present_in_part_one.clear();
            std::for_each_n(line.cbegin(),
                            (line.length()/2),
                            [&present_in_part_one](const char &c){present_in_part_one.insert(c);});
            auto res = std::find_if(line.begin() + (line.length()/2),
                                    line.end(),
                                    [&present_in_part_one](const char &c) {return present_in_part_one.contains(c);});
            if (std::isupper(*res))
            {
                sum += int(*res)-65+27;
                // std::cout << int(*res)-65+27;
            } else
            {
                sum += int(*res)- 96;
            }
        }
        std::cout << "part 1 solution: "  << sum << std::endl;
    }
    // Part 2.
    {
        std::ifstream infile("../../data/day03/input.txt");
        std::string line;
        int sum = 0;
        std::set<char> present_in_one{};
        std::set<char> present_in_two{};
        while (std::getline(infile, line))
        {
            present_in_one.clear();
            present_in_two.clear();
            std::for_each(line.cbegin(),
                          line.cend(),
                          [&present_in_one](auto &c){present_in_one.insert(c);});
            assert(std::getline(infile, line));
            std::for_each(line.cbegin(),
                          line.cend(),
                          [&present_in_two](auto &c){present_in_two.insert(c);});
            assert(std::getline(infile, line));
            auto res = std::find_if(line.cbegin(),
                                    line.cend(),
                                    [&present_in_two, &present_in_one](auto &c)
                                    {
                                        return present_in_one.contains(c) && present_in_two.contains(c);
                                    });
            if (std::isupper(*res))
            {
                sum += int(*res)-65+27;
                // std::cout << int(*res)-65+27;
            } else
            {
                sum += int(*res)- 96;
            }
        }
        std::cout << "part 2 solution: " << sum << std::endl;
    }
    return 0;
}

