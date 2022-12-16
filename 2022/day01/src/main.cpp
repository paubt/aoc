//
// Created by user on 12/16/22.
//
#include <iostream>
#include <fstream>
#include <queue>

int main() {
    // Part 1.
    {
        std::ifstream infile("../../data/day01/input.txt");
         std::string line;
        int current_total = 0;
        int max = 0;
        while (std::getline(infile, line)) {
            if (line == "") {
                if (current_total > max) {
                    max = current_total;
                }
                current_total = 0;
            } else {
                current_total += std::stoi(line);
            }
        }
        std::cout << "part 1 solution: " << max << std::endl;
    }
    // Part 2.
    {
        std::ifstream infile("../../data/day01/input.txt");
        std::string line;
        int current_total = 0;
        std::priority_queue<int> q;
        while (std::getline(infile, line)) {
            if (line == "") {
                if (current_total > 0) {
                    q.push(current_total);
                }
                current_total = 0;
            } else {
                current_total += std::stoi(line);
            }
        }
        if (current_total > 0)
            q.push(current_total);
        int total = 0;
        for (int i = 0; i < 3; i++)
        {
            total += q.top();
            q.pop();
        }
        std::cout << "part 2 solution: " << total << std::endl;
    }
    return 0;
}

