//
// Created by user on 12/16/22.
//
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <optional>
#include <numeric>
#include <cassert>
int main() {
    // Part 1.
    {
        std::ifstream infile("../../data/day08/input.txt");
        std::string line;
        std::vector<std::vector<std::pair<int, bool>>> forest;
        while (std::getline(infile, line)) {
            // Read in and set all on the edge to true -> visible.

            std::vector<std::pair<int, bool>> tree_row;
            for (auto c: line) {
                tree_row.emplace_back(int(c) - '0', false);
            }
            tree_row.front().second = true;
            tree_row.back().second = true;
            forest.push_back(tree_row);
        }
        std::for_each(forest.front().begin(), forest.front().end(), [](auto &ele) { ele.second = true; });
        std::for_each(forest.back().begin(), forest.back().end(), [](auto &ele) { ele.second = true; });
        // Go row-vise
        for (auto &row: forest) {
            auto current_max_height = -1;
            for (auto &[h, p]: row) {
                if (h <= current_max_height)
                    continue;
                current_max_height = h;
                p = true;
                if (current_max_height == 9)
                    break;
            }
            current_max_height = -1;
            for (auto r_it = row.rbegin(); r_it != row.rend(); r_it++) {
                auto &[h, p] = *r_it;
                if (h <= current_max_height)
                    continue;
                current_max_height = h;
                p = true;
                if (current_max_height == 9)
                    break;
            }
        }
        // Go Column wise
        for (int i = 0; i < forest.front().size(); ++i) {
            int current_max_height = -1;
            for (int j = 0; j < forest.size(); ++j) {
                auto &[h, p] = forest[j][i];
                if (h <= current_max_height)
                    continue;
                current_max_height = h;
                p = true;
                if (current_max_height == 9)
                    break;
            }
            current_max_height = -1;
            for (int j = forest.size() - 1; j >= 0; --j) {
                auto &[h, p] = forest[j][i];
                if (h <= current_max_height)
                    continue;
                current_max_height = h;
                p = true;
                if (current_max_height == 9)
                    break;
            }
        }
        auto t = std::transform_reduce(forest.begin(),
                                       forest.end(),
                                       0,
                                       std::plus{},
                                       [](auto &vec) {

                                           return std::count_if(vec.begin(), vec.end(), [](auto &p) {
                                               return p.second;
                                           });
                                       });
        std::cout << "Part 1: " << t << std::endl;
    }
    {
        std::ifstream infile("../../data/day08/input.txt");
        std::string line;
        std::vector<std::vector<int>> forest;
        while (std::getline(infile, line))
        {
            // Read in and set all on the edge to true -> visible.
            std::vector<int> tree_row;
            for (auto c: line)
            {
                tree_row.emplace_back(int(c) - '0');
            }
            forest.push_back(tree_row);
        }
        unsigned int max_sum = 0;
        for (int row_idx = 1; row_idx < forest.size()-1; ++row_idx) {
            for (int col_idx = 1; col_idx < forest.front().size()-1; ++col_idx) {
                const unsigned int center_height = forest[row_idx][col_idx];
                unsigned int row_down = row_idx;
                unsigned int view_down = 0;
                do {
                    row_down++;
                    view_down++;
                } while (row_down < forest.size()-1 && forest[row_down][col_idx] < center_height);
                unsigned int view_up = 0;
                int row_up = row_idx;
                do {
                    row_up--;
                    view_up++;
                } while (row_up > 0 && forest[row_up][col_idx] < center_height);
                unsigned int view_left = 0;
                int col_left = col_idx;
                do {
                    col_left--;
                    view_left++;
                } while (col_left > 0 && forest[row_idx][col_left] < center_height);
                unsigned int col_right = col_idx;
                unsigned int view_right = 0;
                do {
                    col_right++;
                    view_right++;
                } while (col_right < forest.front().size()-1 && forest[row_idx][col_right] < center_height);
                unsigned int product = view_down * view_left * view_right * view_up;
                if (product > max_sum)
                    max_sum = product;
            }
        }
        std::cout << "Part 2: " << max_sum <<  std::endl;
    }
    return 0;
}

