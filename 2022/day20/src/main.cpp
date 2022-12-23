//
// Created by user on 12/16/22.
//

#include <algorithm>
#include <list>
#include <iostream>
#include <fstream>

int main() {
    /*
    // Part 1.
    {
        std::ifstream infile("../../data/day20/input.txt");
        std::string line;
        std::vector<int> v{};
        int sum = 0;
        while (std::getline(infile, line))
        {
            v.push_back(std::stoi(line));
        }
        auto length_v = v.size();
        infile.close();
        infile.open("../../data/day20/input.txt");
        while (std::getline(infile, line))
        {
            // Find next index.
            auto idx = std::distance(v.cbegin(), std::find(v.cbegin(), v.cend(), std::stoi(line)));
            if (std::stoi(line) > 0)
            {
                v.erase(v.begin() + idx);
                std::rotate(v.begin(), v.begin() + (std::stoi(line) % length_v), v.end());
                v.insert(v.begin() + idx, std::stoi(line));
                if (idx >= (v.size() - std::stoi(line)))
                    std::rotate(v.rbegin(), v.rbegin() + (std::stoi(line) % length_v) + 1, v.rend());
                else
                    std::rotate(v.rbegin(), v.rbegin() + (std::stoi(line) % length_v) , v.rend());
            }
            else if (std::stoi(line) < 0)
            {
                v.erase(v.begin() + idx);
                std::rotate(v.rbegin(), v.rbegin() + abs(std::stoi(line)), v.rend());
                v.insert(v.begin() + idx, std::stoi(line));
                if (idx <= abs(std::stoi(line)))
                    std::rotate(v.begin(), v.begin() + abs(std::stoi(line))+ 1 , v.end());
                else
                    std::rotate(v.begin(), v.begin() + abs(std::stoi(line)) , v.end());

            }
        }
        auto idx_zero = std::distance(v.cbegin(), std::find(v.cbegin(), v.cend(), 0));
        auto ooo1th = (idx_zero+1000) % v.size();
        auto ooo2th = (idx_zero+2000) % v.size();
        auto ooo3th = (idx_zero+3000) % v.size();
        auto n1 = v.at(ooo1th) + v.at(ooo2th) + v.at(ooo3th) ;
        std::cout << "part 1 solution: "  << n1  << std::endl;
    }
    */
    // Part 1.
    {
        std::ifstream infile("../../data/day20/input.txt");
        std::string line;
        std::list<int> v{};
        while (std::getline(infile, line))
        {
            v.push_back(std::stoi(line));
        }
        auto length_v = v.size();
        infile.close();
        infile.open("../../data/day20/input.txt");
        while (std::getline(infile, line))
        {
            const auto line_value = std::stoi(line);
            auto from = std::find(v.cbegin(), v.cend(), line_value);
            // Find next index.
            const auto from_c = from;
            const auto idx = std::distance(v.cbegin(), from_c);
            unsigned long offset = 0;
            if (line_value < 0)
            {
                offset = ((line_value + (length_v*2)) % length_v) - 1;
            }
            else
            {
                offset = line_value % length_v;
            }
            if (true) //(line_value > 0)
            {
                //const auto offset = line_value % v.size();
                auto to_uncut = idx + offset;
                if (to_uncut >= length_v)
                {
                    to_uncut =  to_uncut % length_v;
                }

                if (to_uncut > idx)
                {
                    v.erase(from);
                    auto to_iter = v.begin();
                    std::advance(to_iter, to_uncut);
                    v.insert(to_iter, line_value);
                }
                else if (idx > to_uncut)
                {
                    auto to_iter = v.begin();
                    std::advance(to_iter, to_uncut + 1);
                    v.insert(to_iter, line_value);
                    v.erase(from);
                }
            }

        }
        auto zero_iter = std::find(v.cbegin(), v.cend(), 0);
        const auto idx_zero = std::distance(v.cbegin(), zero_iter);
        const auto zero_to_end = std::distance(zero_iter, v.cend());
        auto sum = 0;
        auto current_idx = zero_iter;
        for (int i = 0; i < 3;  i++)
        {
            // Calc new index.
            auto new_idx = std::distance(v.cbegin(), current_idx) + 1000;
            if (new_idx >= length_v)
                new_idx = new_idx % length_v;
            current_idx = v.begin();
            std::advance(current_idx, new_idx);
            sum += *current_idx;
        }
        std::cout << "part 1 solution: "  << sum  << std::endl;
    }
    // Part 2.
    {
        std::ifstream infile("../../data/day20/input.txt");
        std::string line;
        int sum = 0;
        while (std::getline(infile, line))
        {

        }
        std::cout << "part 2 solution: " << sum << std::endl;
    }
    return 0;
}

