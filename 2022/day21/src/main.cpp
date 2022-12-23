//
// Created by user on 12/16/22.
//

#include <algorithm>
#include <list>
#include <iostream>
#include <fstream>
#include <sstream>
#include <map>
#include <assert.h>

enum Operand
{
    Add,
    Sub,
    Mul,
    Div,
};

struct Monkey
{
    std::string Name;
    std::string Op_a;
    std::string Op_b;
    Operand     Op_t;
};

int main() {
    std::map<std::string, long int> mm;
    std::vector<Monkey> all_m;
    {
        std::ifstream infile("../../data/day21/input.txt");
        std::string line;
        const char delim = ' ';
        std::vector<Monkey> monkeys;
        // std::map<std::string, long int> mm;
        while (std::getline(infile, line))
        {
            std::istringstream iss(line);
            std::vector<std::string> temp;
            std::string item;
            while (std::getline(iss, item, delim))
            {
                temp.push_back(item);
            }
            assert(temp[0].size() > 2);
            switch (temp.size()) {
                case 2:
                    assert(!mm.contains(temp[0]));
                    mm.insert({temp[0].substr(0, temp[0].size() -1), std::stoi(temp[1])});
                    break;
                case 4:
                    Operand op;
                    switch (temp[2][0]) {
                        case '+':
                            op = Operand::Add;
                            break;
                        case '-':
                            op = Operand::Sub;
                            break;
                        case '*':
                            op = Operand::Mul;
                            break;
                        case '/':
                            op = Operand::Div;
                            break;
                        default:
                            exit(42);
                    }
                    monkeys.push_back({temp[0].substr(0, temp[0].size() -1), temp[1], temp[3], op});
                    break;
                default:
                    exit(100);
            }
        }
        all_m = monkeys;
        std::vector<Monkey> new_monkeys = monkeys;
        while (!mm.contains("root"))
        {
            monkeys = new_monkeys;
            new_monkeys.clear();
            for (auto m: monkeys)
            {
                if (mm.contains(m.Op_a) && mm.contains(m.Op_b) && !mm.contains(m.Name))
                {
                    assert(!mm.contains(m.Name));
                    long int sol = 0;
                    switch (m.Op_t) {
                        case Operand::Add:
                            sol = mm.at(m.Op_a) + mm.at(m.Op_b);
                            break;
                        case Operand::Sub:
                            sol = mm.at(m.Op_a) - mm.at(m.Op_b);
                            break;
                        case Operand::Mul:
                            sol = mm.at(m.Op_a) * mm.at(m.Op_b);
                            break;
                        case Operand::Div:
                            sol = mm.at(m.Op_a) / mm.at(m.Op_b);
                            break;
                    }
                    mm.insert({m.Name, sol});
                } else
                    new_monkeys.push_back(m);
            }
        }
        std::cout << "part 2 solution: " << mm.at("root") << std::endl;
    }
    // Part 2.
    {
        std::vector<Monkey> h_to_r;
        h_to_r.push_back(*std::find_if(all_m.begin(), all_m.end(), [](const Monkey m) {
            return m.Op_a == "humn" || m.Op_b == "humn";
        }));
        while (h_to_r.back().Name != "root")
        {
            h_to_r.push_back(*std::find_if(all_m.cbegin(), all_m.cend(), [&h_to_r](const Monkey m) {
                return h_to_r.back().Name == m.Op_a || h_to_r.back().Name == m.Op_b;
            }));
        }
        long int eq_to = 0;
        if (h_to_r.rbegin()[1].Name == h_to_r.back().Op_a)
            eq_to = mm.at(h_to_r.back().Op_b);
        else
            eq_to = mm.at(h_to_r.back().Op_a);
        h_to_r.pop_back();
        while (h_to_r.size() > 1)
        {
            if (h_to_r.rbegin()[1].Name == h_to_r.back().Op_a)
            switch (h_to_r.back().Op_t)
            {
                case Operand::Add:
                    eq_to = eq_to - mm.at(h_to_r.back().Op_b);
                    break;
                case Operand::Sub:
                    eq_to = eq_to + mm.at(h_to_r.back().Op_b);
                    break;
                case Operand::Mul:
                    eq_to = eq_to / mm.at(h_to_r.back().Op_b);
                    break;
                case Operand::Div:
                    eq_to = eq_to * mm.at(h_to_r.back().Op_b);
                    break;
            }
            else
            switch (h_to_r.back().Op_t)
            {
                case Operand::Add:
                    eq_to = eq_to - mm.at(h_to_r.back().Op_a);
                    break;
                case Operand::Sub:
                    eq_to = mm.at(h_to_r.back().Op_a) - eq_to;
                    break;
                case Operand::Mul:
                    eq_to = eq_to / mm.at(h_to_r.back().Op_a);
                    break;
                case Operand::Div:
                    eq_to = mm.at(h_to_r.back().Op_a) / eq_to;
                    break;
            }
            h_to_r.pop_back();
        }
        if ("humn" == h_to_r.front().Op_a)
        switch (h_to_r.back().Op_t)
        {
            case Operand::Add:
                eq_to = eq_to - mm.at(h_to_r.back().Op_b);
                break;
            case Operand::Sub:
                eq_to = eq_to + mm.at(h_to_r.back().Op_b);
                break;
            case Operand::Mul:
                eq_to = eq_to / mm.at(h_to_r.back().Op_b);
                break;
            case Operand::Div:
                eq_to = eq_to * mm.at(h_to_r.back().Op_b);
                break;
        }
        else
        switch (h_to_r.back().Op_t)
        {
            case Operand::Add:
                eq_to = eq_to - mm.at(h_to_r.back().Op_a);
                break;
            case Operand::Sub:
                eq_to = mm.at(h_to_r.back().Op_a) - eq_to;
                break;
            case Operand::Mul:
                eq_to = eq_to / mm.at(h_to_r.back().Op_a);
                break;
            case Operand::Div:
                eq_to = mm.at(h_to_r.back().Op_a) / eq_to;
                break;
        }
        std::cout << "part 2 solution: "   << eq_to << std::endl;
    }
    return 0;
}

