//
// Created by user on 12/16/22.
//
#include <iostream>
#include <fstream>
#include <queue>

int main() {
    // Part 1.
    {
        std::ifstream infile("../../data/day02/input.txt");
         std::string line;
        int current_total = 0;
        while (std::getline(infile, line)) {
            char them = line.front();
            char you = line.back();
            if ((them == 'A' && you == 'X') || (them == 'B' && you == 'Y')|| (them == 'C' && you == 'Z'))
            {
                current_total += 3;
            } else if ((them == 'A' && you == 'Y') || (them == 'B' && you == 'Z') || (them == 'C' && you == 'X'))
            {
                current_total += 6;
            }
            switch (you) {
                case 'Z':
                    current_total += 1;
                case 'Y':
                    current_total += 1;
                default:
                    current_total += 1;
                    break;
            }
        }
        std::cout << "part 1 solution: " << current_total << std::endl;
    }
    // Part 2.
    {
        std::ifstream infile("../../data/day02/input.txt");
        std::string line;
        int current_total = 0;
        while (std::getline(infile, line)) {
            char them = line.front();
            char you = line.back();
            if ((them == 'A' && you == 'X') || (them == 'B' && you == 'Z')|| (them == 'C' && you == 'Y'))
            {
                current_total += 3;
            } else if ((them == 'A' && you == 'Z') || (them == 'B' && you == 'Y') || (them == 'C' && you == 'X'))
            {
                current_total += 2;
            } else
            {
                current_total += 1;
            }
            switch (you) {
                case 'Z':
                    current_total += 3;
                case 'Y':
                    current_total += 3;
                default:
                    break;
            }
        }
        std::cout << "part 2 solution: " << current_total << std::endl;
    }
    return 0;
}

