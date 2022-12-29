//
// Created by user on 12/16/22.
//
#include <iostream>
#include <fstream>
#include <array>
#include <cassert>
int main() {
    // Part 1.
    {
        std::ifstream infile("../../data/day10/input.txt");
        std::string line;
        long int reg_x = 1;
        long int cycle = 0;
        long int sum = 0;
        while (std::getline(infile, line)) {
            if (line[0] == 'n')
            {
                ++cycle;
                if (cycle == 20 || (cycle - 20) % 40 == 0)
                    sum  += cycle * reg_x;
            }
            else
            {
                long int v = std::stoi(line.substr(4));
                ++cycle;
                if (cycle == 20 || (cycle - 20) % 40 == 0)
                    sum  += cycle * reg_x;
                ++cycle;
                if (cycle == 20 || (cycle - 20) % 40 == 0)
                    sum  += cycle * reg_x;
                reg_x += v;
            }
        }
        std::cout << "Part 1: " << sum <<  std::endl;
    }
    // Part 1.
    {
        std::ifstream infile("../../data/day10/input.txt");
        std::string line;
        long int cycle = 0;
        long int sprite_position = 1;
        std::array<char, 240> row{};
        while (std::getline(infile, line)) {
            if (line[0] == 'n')
            {
                if (cycle == sprite_position -1 || cycle == sprite_position +1 || cycle == sprite_position)
                    row[cycle] = '#';
                else
                    row[cycle] = ' ';
                ++cycle;
                if (cycle % 40 == 0)
                    sprite_position += 40;
            }
            else
            {
                long int v = std::stoi(line.substr(4));
                if (cycle == sprite_position -1 || cycle == sprite_position +1 || cycle == sprite_position)
                    row[cycle] = '#';
                else
                    row[cycle] = ' ';
                ++cycle;
                if (cycle % 40 == 0)
                    sprite_position += 40;
                if (cycle == sprite_position -1 || cycle == sprite_position +1 || cycle == sprite_position)
                    row[cycle] = '#';
                else
                    row[cycle] = ' ';
                ++cycle;
                if (cycle % 40 == 0)
                    sprite_position += 40;
                sprite_position += v;
            }
        }
        std::cout << "Part 2:" << std::endl;
        for (int i = 0; i < 6; ++i) {
            for (int j = 0; j < 40; ++j) {
                std::cout << row[i*40+j];
            }
            std::cout << std::endl;
        }
    }
    return 0;
}

