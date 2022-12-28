//
// Created by user on 12/16/22.
//
#include <iostream>
#include <fstream>
#include <set>
#include <vector>

struct Point
{
    int x;
    int y;
};

int main() {
    // Part 1.
    {
        std::ifstream infile("../../data/day09/input.txt");
        std::string line;
        std::set<std::pair<int, int>> visited_pos;
        visited_pos.insert({0,0});
        int x_head = 0, y_head = 0, x_tail = 0, y_tail = 0;

        while (std::getline(infile, line))
        {
            // Read in and set all on the edge to true -> visible.
            // std::cout << "== " << line[0] << "  " << std::stoi(line.substr(2)) << "  ==" << std::endl;
            for (int reps = 0; reps < std::stoi(line.substr(2)); ++reps)
            {
                switch (line[0])
                {
                    case 'R':
                        x_head += 1;
                        break;
                    case 'L':
                        x_head -= 1;
                        break;
                    case 'D':
                        y_head -= 1;
                        break;
                    case 'U':
                        y_head += 1;
                        break;
                    default:
                        exit(42);
                }
                // 0° U
                if (x_head == x_tail && std::abs(y_head - y_tail) > 1 && y_head > y_tail)
                    y_tail += 1;
                // 180° D
                else if (x_head == x_tail && std::abs(y_head - y_tail) > 1 && y_head < y_tail)
                    y_tail -= 1;
                // 90° R
                else if (y_head == y_tail && std::abs(x_head - x_tail) > 1 && x_head > x_tail)
                    x_tail += 1;
                // 270° L
                else if (y_head == y_tail && std::abs(x_head - x_tail) > 1 && x_head < x_tail)
                    x_tail -= 1;
                // 22,5° & 67,5°
                else if (y_head > y_tail && x_head > x_tail &&
                         ((std::abs(x_head - x_tail) > 1) || std::abs(y_head - y_tail) > 1))
                {
                    x_tail += 1;
                    y_tail += 1;
                }
                else if (y_head > y_tail && x_head < x_tail &&
                         ((std::abs(x_head - x_tail) > 1) || std::abs(y_head - y_tail) > 1))
                {
                    x_tail -= 1;
                    y_tail += 1;
                }
                else if (y_head < y_tail && x_head > x_tail &&
                         ((std::abs(x_head - x_tail) > 1) || std::abs(y_head - y_tail) > 1))
                {
                    x_tail += 1;
                    y_tail -= 1;
                }
                else if (y_head < y_tail && x_head < x_tail &&
                         ((std::abs(x_head - x_tail) > 1) || std::abs(y_head - y_tail) > 1))
                {
                    x_tail -= 1;
                    y_tail -= 1;
                }
                visited_pos.insert({x_tail, y_tail});
            }
        }
        std::cout << "Part 1: " << visited_pos.size() << std::endl;
    }
    // Part 2.
    {
        const int ROPE_SIZE = 10;
        std::ifstream infile("../../data/day09/input.txt");
        std::string line;
        std::set<std::pair<int, int>> visited_pos;
        visited_pos.insert({0,0});
        std::vector<Point> rope(ROPE_SIZE, {0,0});
        while (std::getline(infile, line))
        {
            // Read in and set all on the edge to true -> visible.
            // std::cout << "== " << line[0] << "  " << std::stoi(line.substr(2)) << "  ==" << std::endl;
            for (int reps = 0; reps < std::stoi(line.substr(2)); ++reps)
            {
                switch (line[0])
                {
                    case 'R':
                        rope[0].x += 1;
                        break;
                    case 'L':
                        rope[0].x -= 1;
                        break;
                    case 'D':
                        rope[0].y -= 1;
                        break;
                    case 'U':
                        rope[0].y += 1;
                        break;
                    default:
                        exit(42);
                }
                for (int rope_idx = 1; rope_idx < rope.size(); ++rope_idx) {
                    {
                        int x_head = rope[rope_idx - 1].x;
                        int y_head = rope[rope_idx - 1].y;
                        int x_tail = rope[rope_idx].x;
                        int y_tail = rope[rope_idx].y;
                    }
                    // 0° U
                    if (rope[rope_idx - 1].x == rope[rope_idx].x && std::abs(rope[rope_idx - 1].y - rope[rope_idx].y) > 1 && rope[rope_idx - 1].y > rope[rope_idx].y)
                        rope[rope_idx].y += 1;
                        // 180° D
                    else if (rope[rope_idx - 1].x == rope[rope_idx].x && std::abs(rope[rope_idx - 1].y - rope[rope_idx].y) > 1 && rope[rope_idx - 1].y < rope[rope_idx].y)
                        rope[rope_idx].y -= 1;
                        // 90° R
                    else if (rope[rope_idx - 1].y == rope[rope_idx].y && std::abs(rope[rope_idx - 1].x - rope[rope_idx].x) > 1 && rope[rope_idx - 1].x > rope[rope_idx].x)
                        rope[rope_idx].x += 1;
                        // 270° L
                    else if (rope[rope_idx - 1].y == rope[rope_idx].y && std::abs(rope[rope_idx - 1].x - rope[rope_idx].x) > 1 && rope[rope_idx - 1].x < rope[rope_idx].x)
                        rope[rope_idx].x -= 1;
                        // 22,5° & 67,5°
                    else if (rope[rope_idx - 1].y > rope[rope_idx].y && rope[rope_idx - 1].x > rope[rope_idx].x &&
                             ((std::abs(rope[rope_idx - 1].x - rope[rope_idx].x) > 1) || std::abs(rope[rope_idx - 1].y - rope[rope_idx].y) > 1))
                    {
                        rope[rope_idx].x += 1;
                        rope[rope_idx].y += 1;
                    }
                    else if (rope[rope_idx - 1].y > rope[rope_idx].y && rope[rope_idx - 1].x < rope[rope_idx].x &&
                             ((std::abs(rope[rope_idx - 1].x - rope[rope_idx].x) > 1) || std::abs(rope[rope_idx - 1].y - rope[rope_idx].y) > 1))
                    {
                        rope[rope_idx].x -= 1;
                        rope[rope_idx].y += 1;
                    }
                    else if (rope[rope_idx - 1].y < rope[rope_idx].y && rope[rope_idx - 1].x > rope[rope_idx].x &&
                             ((std::abs(rope[rope_idx - 1].x - rope[rope_idx].x) > 1) || std::abs(rope[rope_idx - 1].y - rope[rope_idx].y) > 1))
                    {
                        rope[rope_idx].x += 1;
                        rope[rope_idx].y -= 1;
                    }
                    else if (rope[rope_idx - 1].y < rope[rope_idx].y && rope[rope_idx - 1].x < rope[rope_idx].x &&
                             ((std::abs(rope[rope_idx - 1].x - rope[rope_idx].x) > 1) || std::abs(rope[rope_idx - 1].y - rope[rope_idx].y) > 1))
                    {
                        rope[rope_idx].x -= 1;
                        rope[rope_idx].y -= 1;
                    }
                }

                visited_pos.insert({rope.back().x, rope.back().y});
            }
        }
        std::cout << "Part 2: " << visited_pos.size() << std::endl;
    }
    return 0;
}

