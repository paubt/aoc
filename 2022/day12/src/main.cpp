//
// Created by user on 12/16/22.
//
#include <iostream>
#include <fstream>
#include <vector>
#include <queue>
#include <map>

struct Position
{
    Position(int x_, int y_, int step_count_): x(x_), y(y_), step_count(step_count_){};
    int x;
    int y;
    int step_count;
};

class Compare
{
public:
    bool operator() (Position l, Position r)
    {
        return l.step_count > r.step_count;
    }
};

void part_1()
{
    // Part 1.
    {
        std::ifstream infile("../../data/day12/input.txt");
        std::string line;
        int current_total = 0;
        std::vector<std::vector<int>> height_map;
        std::map<std::pair<int, int>, int> visited_positions;
        std::priority_queue<Position, std::vector<Position>, Compare> open_positions;
        Position target_position = Position(0,0,0);
        int max = 0;
        while (std::getline(infile, line)) {
            std::vector<int> temp;
            for (auto c: line) {
                if (c == 'S') {
                    open_positions.emplace(height_map.size(), temp.size(), 0);
                    visited_positions[{height_map.size(), temp.size()}] = 0;
                    temp.push_back(-1);
                    continue;
                }
                if (c == 'E')
                {
                    target_position.x = height_map.size();
                    target_position.y = temp.size();
                    temp.push_back(int('z') - int('a'));

                    continue;
                }
                temp.push_back(int(c) - int('a'));
            }
            height_map.push_back(temp);
        }
        do {
            auto from_pos = open_positions.top();
            open_positions.pop();
            for (auto [new_x, new_y]: {std::make_pair(from_pos.x - 1, from_pos.y),
                                       {from_pos.x + 1, from_pos.y},
                                       {from_pos.x, from_pos.y - 1},
                                       {from_pos.x, from_pos.y + 1}}) {
                // Check that new position is in range of the array.
                if ( new_x < 0 || new_y < 0 || new_x >= height_map.size() || new_y >= height_map.front().size())
                    continue;
                // Check that new position at least one higher is.
                if (height_map[new_x][new_y] > (height_map[from_pos.x][from_pos.y] + 1))
                    continue;
                if  (new_x == target_position.x && new_y == target_position.y){ // (height_map[new_x][new_y] == -28)

                    std::cout << "Found highest point at position " << new_x << "|" << new_y << " after "
                              << from_pos.step_count + 1 << " steps" << std::endl;
                    return;
                }
                if (!visited_positions.contains({new_x,new_y}))
                {
                    open_positions.emplace(new_x, new_y,from_pos.step_count+1);
                    visited_positions[{new_x,new_y}] =   from_pos.step_count+1;
                }
            }
        } while (!open_positions.empty());
        std::cout << "part 1 solution: " << max << std::endl;
    }
}

void part_2()
{
    // Part 1.
    {
        std::ifstream infile("../../data/day12/input.txt");
        std::string line;
        std::vector<std::vector<int>> height_map;
        std::map<std::pair<int, int>, int> visited_positions;
        std::priority_queue<Position, std::vector<Position>, Compare> open_positions;
        Position target_position = Position(0,0,0);
        int max = 0;
        while (std::getline(infile, line)) {
            std::vector<int> temp;
            for (auto c: line) {
                if (c == 'E') {
                    open_positions.emplace(height_map.size(), temp.size(), 0);
                    visited_positions[{height_map.size(), temp.size()}] = 0;
                    temp.push_back(int('z') - int('a'));
                    continue;
                }
                if (c == 'S')
                {
                    target_position.x = height_map.size();
                    target_position.y = temp.size();
                    temp.push_back(-1);

                    continue;
                }
                temp.push_back(int(c) - int('a'));
            }
            height_map.push_back(temp);
        }
        do {
            auto from_pos = open_positions.top();
            open_positions.pop();
            for (auto [new_x, new_y]: {std::make_pair(from_pos.x - 1, from_pos.y),
                                       {from_pos.x + 1, from_pos.y},
                                       {from_pos.x, from_pos.y - 1},
                                       {from_pos.x, from_pos.y + 1}}) {
                // Check that new position is in range of the array.
                if ( new_x < 0 || new_y < 0 || new_x >= height_map.size() || new_y >= height_map.front().size())
                    continue;
                // Check that new position at least one higher is.
                if (height_map[new_x][new_y] + 1 < (height_map[from_pos.x][from_pos.y]))
                    continue;
                if  (height_map[new_x][new_y] == 0){ // (height_map[new_x][new_y] == -28)
                    std::cout << "Found highest point at position " << new_x << "|" << new_y << " after "
                              << from_pos.step_count + 1 << " steps" << std::endl;
                    return;
                }
                if (!visited_positions.contains({new_x,new_y}))
                {
                    open_positions.emplace(new_x, new_y,from_pos.step_count+1);
                    visited_positions[{new_x,new_y}] =   from_pos.step_count+1;
                }
            }
        } while (!open_positions.empty());
        std::cout << "part 1 solution: " << max << std::endl;
    }
}

int main()
{
    part_1();
    part_2();
    return 0;
}

