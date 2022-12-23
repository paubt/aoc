//
// Created by user on 12/16/22.
//

#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <set>
#include <algorithm>
#include <numeric>

void print_pos_plan(const std::vector<std::vector<bool>> &pp, unsigned int s)
{
    std::cout << s << std::endl;
    for (auto &l: pp)
    {
        for (auto e: l)
        {
            if (e)
                std::cout << '#';
            else
                std::cout << '.';
        }
        std::cout << std::endl;
    }
    std::cout << std::endl;
}


int main() {
    {
        std::vector<std::vector<bool>> pos_plan;
        std::ifstream infile("../../data/day23/input.txt");
        std::string line;
        while (std::getline(infile, line))
        {
            std::vector<bool> temp;
            for (char c: line)
            {
                switch (c) {
                    case '#':
                        temp.push_back(true);
                        break;
                    case '.':
                        temp.push_back(false);
                        break;
                    default:
                        exit(42);
                }
            }
            pos_plan.push_back(temp);
        }
        unsigned int step = 0;
        unsigned int direction = 0;
        bool something_moved = false;
        do {

            something_moved = false;
            // Add new rows in case an elv is at the border.
            {
                // North.
                if (std::reduce(pos_plan.front().begin(), pos_plan.front().end()) > 0) {
                    auto t = std::vector<bool>(pos_plan.front().size(), false);
                    pos_plan.insert(pos_plan.begin(), t);
                }
                // South.
                if (std::reduce(pos_plan.back().begin(), pos_plan.back().end()) > 0) {
                    auto t = std::vector<bool>(pos_plan.back().size(), false);
                    pos_plan.insert(pos_plan.end(), t);
                }
                // West.
                if (0 < std::transform_reduce(pos_plan.cbegin(), pos_plan.cend(), 0, std::plus{},
                                              [](auto &val) { return val.front(); }))
                    std::for_each(pos_plan.begin(), pos_plan.end(), [](auto &val) { val.insert(val.begin(), false); });
                // East.
                if (0 < std::transform_reduce(pos_plan.cbegin(), pos_plan.cend(), 0, std::plus{},
                                              [](auto &val) { return val.back(); }))
                    std::for_each(pos_plan.begin(), pos_plan.end(), [](auto &val) { val.insert(val.end(), false); });
            }
            std::map<std::pair<int,int>, std::vector<std::pair<int,int>>> propositions_made;
            std::set<std::pair<int,int>> proposed_elves;
            // Add all the isolated elves to proposed_elves -> so they don't move.
            {
                for (int line_idx = 1; line_idx < (pos_plan.size() - 1); line_idx++)
                {
                    for (int row_idx = 1; row_idx < (pos_plan.front().size() - 1); row_idx++)
                    {
                        if (pos_plan[line_idx][row_idx] &&
                            0 == std::reduce(pos_plan[line_idx - 1].begin() + row_idx - 1, pos_plan[line_idx - 1].begin() + row_idx + 2) &&
                            0 == std::reduce(pos_plan[line_idx + 1].begin() + row_idx - 1, pos_plan[line_idx + 1].begin() + row_idx + 2) &&
                            !pos_plan[line_idx][row_idx - 1] &&
                            !pos_plan[line_idx][row_idx + 1])
                        {
                            proposed_elves.emplace(line_idx, row_idx);
                        }
                    }
                }
            }
            for (int dir = 0; dir < 4;  dir++)
            {
                switch ((direction + dir) % 4) {
                    // North.
                    case 0:
                        for (int line_idx = 1; line_idx < (pos_plan.size() - 1); line_idx++)
                        {
                            for (int row_idx = 1; row_idx < (pos_plan.front().size() - 1); row_idx++)
                            {
                                if (pos_plan[line_idx][row_idx] && !proposed_elves.contains({line_idx, row_idx}) &&
                                    0 == std::reduce(pos_plan[line_idx - 1].begin() + row_idx - 1, pos_plan[line_idx - 1].begin() + row_idx + 2))
                                {
                                    if (propositions_made.contains({line_idx - 1, row_idx}))
                                        propositions_made.at({line_idx - 1, row_idx}).emplace_back(line_idx, row_idx);
                                    else
                                        propositions_made.insert({{line_idx - 1, row_idx}, {{line_idx, row_idx}}});
                                    proposed_elves.insert({line_idx, row_idx});
                                }
                            }
                        }
                        break;
                    // South.
                    case 1:
                        for (int line_idx = 1; line_idx < (pos_plan.size() - 1); line_idx++)
                        {
                            for (int row_idx = 1; row_idx < (pos_plan.front().size() - 1); row_idx++)
                            {
                                if (pos_plan[line_idx][row_idx] && !proposed_elves.contains({line_idx, row_idx}) &&
                                    0 == std::reduce(pos_plan[line_idx + 1].begin() + row_idx - 1, pos_plan[line_idx + 1].begin() + row_idx + 2))
                                {
                                    if (propositions_made.contains({line_idx + 1, row_idx}))
                                        propositions_made.at({line_idx + 1, row_idx}).emplace_back(line_idx, row_idx);
                                    else
                                        propositions_made.insert({{line_idx + 1, row_idx}, {{line_idx, row_idx}}});
                                    proposed_elves.insert({line_idx, row_idx});
                                }
                            }
                        }
                        break;
                    // West.
                    case 2:
                        for (int line_idx = 1; line_idx < (pos_plan.size() - 1); line_idx++)
                        {
                            for (int row_idx = 1; row_idx < (pos_plan.front().size() - 1); row_idx++)
                            {
                                if (pos_plan[line_idx][row_idx] && !proposed_elves.contains({line_idx, row_idx}) &&
                                    0 == pos_plan[line_idx-1][row_idx-1] +
                                         pos_plan[line_idx][row_idx-1] +
                                         pos_plan[line_idx+1][row_idx-1])
                                {
                                    if (propositions_made.contains({line_idx, row_idx - 1}))
                                        propositions_made.at({line_idx, row_idx - 1}).emplace_back(line_idx, row_idx);
                                    else
                                        propositions_made.insert({{line_idx, row_idx - 1}, {{line_idx, row_idx}}});
                                    proposed_elves.insert({line_idx, row_idx});
                                }
                            }
                        }
                        break;
                    // East.
                    case 3:
                        for (int line_idx = 1; line_idx < (pos_plan.size() - 1); line_idx++)
                        {
                            for (int row_idx = 1; row_idx < (pos_plan.front().size() - 1); row_idx++)
                            {
                                if (pos_plan[line_idx][row_idx] && !proposed_elves.contains({line_idx, row_idx}) &&
                                    0 == pos_plan[line_idx-1][row_idx+1] +
                                         pos_plan[line_idx][row_idx+1] +
                                         pos_plan[line_idx+1][row_idx+1])
                                {
                                    if (propositions_made.contains({line_idx, row_idx + 1}))
                                        propositions_made.at({line_idx, row_idx + 1}).emplace_back(line_idx, row_idx);
                                    else
                                        propositions_made.insert({{line_idx, row_idx + 1}, {{line_idx, row_idx}}});
                                    proposed_elves.insert({line_idx, row_idx});
                                }
                            }
                        }
                        break;
                    default:
                        exit(420);
                }
            }
            // Iter over the map and move the elv if it's the only one to that tile.
            for (auto const& [key, val]: propositions_made)
            {
                if(val.size() == 1)
                {
                    pos_plan[key.first][key.second] = true;
                    pos_plan[val.front().first][val.front().second] = false;
                    something_moved = true;
                }
            }
            direction += 1;
            if (++step == 10)
            {
                // Remove empty lines front and back (north and south)
                while ( 0 == std::reduce(pos_plan.front().cbegin(), pos_plan.front().cend()))
                    pos_plan.erase(pos_plan.begin());
                while ( 0 == std::reduce(pos_plan.back().cbegin(), pos_plan.back().cend()))
                    pos_plan.pop_back();
                // Remove empty vertical lines west and east
                while (0 == std::transform_reduce(pos_plan.cbegin(), pos_plan.cend(), 0, std::plus{},
                                                  [](auto &val) { return val.front(); }))
                    std::for_each(pos_plan.begin(), pos_plan.end(), [](auto &val) {val.erase(val.begin());});
                while (0 == std::transform_reduce(pos_plan.cbegin(), pos_plan.cend(), 0, std::plus{},
                                                  [](auto &val) { return val.back(); }))
                    std::for_each(pos_plan.begin(), pos_plan.end(), [](auto &val) {val.pop_back();});
                // count the number of elves and subtract from grid surface
                auto empty_tile_count = pos_plan.size()*pos_plan.front().size() -
                                        std::transform_reduce(pos_plan.cbegin(),
                                                              pos_plan.cend(),
                                                              0,
                                                              std::plus{},
                                                              [](auto &val){return std::count(val.begin(), val.end(), true);});
                std::cout << "part 1 solution: "  << empty_tile_count << std::endl;
            }
        } while (something_moved);
        std::cout << "part 2 solution: " << step  << std::endl;
    }
    return 0;
}

