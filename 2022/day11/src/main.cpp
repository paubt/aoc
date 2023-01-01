//
// Created by user on 12/16/22.
//
#include <cmath>
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <array>
#include <numeric>
#include <cassert>


enum Operation
{
    Add,
    Mul,
    Sqa,
};

class Item
{
public:
    long int value;
    std::string id;
    std::vector<int> history;
};

class Monkey
{
public:
    int monkey_id;
    std::vector<Item*> items;
    Operation op_type;
    int op_val;
    int test_val;
    int true_dest;
    int false_dest;
    int inspection_count;
};

template<int S>
class MonkeyTeam
{
public:
    std::array<Monkey*,S> monkeys;
    void execute_round(bool part1 = true);
    long int sum_inspection_count();
    long int mod_all;
    void calc_mod_all();
};

template<int S>
void MonkeyTeam<S>::calc_mod_all() {
    mod_all = 1;
    for (auto m: monkeys)
    {
        mod_all *= m->test_val;
    }
}

template<int S>
long int MonkeyTeam<S>::sum_inspection_count() {
    std::vector<long int> t;
    for (auto &m: monkeys)
    {
        t.push_back(m->inspection_count);
    }
    std::sort(t.begin(), t.end());
    std::reverse(t.begin(), t.end());
    auto a = *t.begin();
    auto b = *(++t.begin());
    return a*b;
}

template<int S>
void MonkeyTeam<S>::execute_round(bool part1)
{
    for (int monkey_idx = 0; monkey_idx < S; ++monkey_idx)
    {
        const Monkey* current_monkey = monkeys[monkey_idx];
        while (!current_monkey->items.empty())
        {
            auto item = current_monkey->items.front();
            item->history.push_back(current_monkey->monkey_id);
            monkeys[monkey_idx]->items.erase(monkeys[monkey_idx]->items.begin());
            monkeys[monkey_idx]->inspection_count++;
            switch (current_monkey->op_type) {
                case Operation::Add:
                    item->value += current_monkey->op_val;
                    break;
                case Operation::Mul:
                    item->value *= current_monkey->op_val;
                    break;
                case Operation::Sqa:
                    item->value = item->value * item->value;
                    break;
                default:
                    exit(42);
            }
            if (part1)
                item->value = static_cast<long int>(std::floor(static_cast<float>(item->value) / 3));
            else {
                assert(mod_all != 0);
                item->value = item->value % mod_all;
            }
            if (item->value % current_monkey->test_val == 0)
                monkeys[current_monkey->true_dest]->items.push_back(item);
            else
                monkeys[current_monkey->false_dest]->items.push_back(item);
            // std::cout << reduced_worry_level;
        }
    }
}


int main()
{
    // Part 1.
    {
        MonkeyTeam<8> mt{};
        std::ifstream infile("../../data/day11/input.txt");
        std::string s(std::istreambuf_iterator<char>{infile}, {});
        std::string delimiter = "\n\n";
        size_t pos = 0;
        std::string token;
        int mt_m_idx = 0;
        do {
            (pos = s.find(delimiter));
            token = s.substr(0, pos);
            s.erase(0, pos + delimiter.length());
            std::istringstream iss(token);
            std::string line;
            std::vector<std::vector<std::string>> m_in;
            while (std::getline(iss, line)) {
                std::istringstream iss_line(line);
                std::string ele;
                std::vector<std::string> t_v;
                while (std::getline(iss_line, ele, ' ')) {
                    t_v.push_back(ele);
                }
                m_in.push_back(t_v);
            }
            Operation op_t;
            switch (m_in[2][m_in[2].size() - 2][0]) {
                case '*':
                    op_t = Operation::Mul;
                    break;
                case '+':
                    op_t = Operation::Add;
                    break;
                default:
                    exit(45);
            }
            int op_v = 0;
            if (m_in[2][m_in[2].size() - 1] == "old") {
                if (op_t == Operation::Mul)
                    op_t = Operation::Sqa;
                else {
                    op_t = Operation::Mul;
                    op_v = 2;
                }
            } else {
                op_v = std::stoi(m_in[2][m_in[2].size() - 1]);
            }
            Monkey *monkey_temp = new Monkey{mt_m_idx,
                                             {},
                                             op_t,
                                             op_v,
                                             std::stoi(m_in[3][m_in[3].size() - 1]),
                                             std::stoi(m_in[4][m_in[4].size() - 1]),
                                             std::stoi(m_in[5][m_in[5].size() - 1]),
                                             0,
            };
            m_in[1].erase(m_in[1].begin(), m_in[1].begin() + 4);
            for (const auto& s_nb: m_in[1]) {
                monkey_temp->items.push_back(new Item{std::stoi(s_nb),
                                                      "lel",
                                                      {}});
            }
            mt.monkeys[mt_m_idx++] = monkey_temp;
        } while (pos  != std::string::npos);
        for (int i = 0; i <20; ++i) {
            mt.execute_round();
        }
        std::cout << "part 1: " << mt.sum_inspection_count() << std::endl;
    }
    // Part 2.
    {
        MonkeyTeam<8> mt{};
        std::ifstream infile("../../data/day11/input.txt");
        std::string s(std::istreambuf_iterator<char>{infile}, {});
        std::string delimiter = "\n\n";
        size_t pos = 0;
        std::string token;
        int mt_m_idx = 0;
        do {
            (pos = s.find(delimiter));
            token = s.substr(0, pos);
            s.erase(0, pos + delimiter.length());
            std::istringstream iss(token);
            std::string line;
            std::vector<std::vector<std::string>> m_in;
            while (std::getline(iss, line)) {
                std::istringstream iss_line(line);
                std::string ele;
                std::vector<std::string> t_v;
                while (std::getline(iss_line, ele, ' ')) {
                    t_v.push_back(ele);
                }
                m_in.push_back(t_v);
            }
            Operation op_t;
            switch (m_in[2][m_in[2].size() - 2][0]) {
                case '*':
                    op_t = Operation::Mul;
                    break;
                case '+':
                    op_t = Operation::Add;
                    break;
                default:
                    exit(45);
            }
            int op_v = 0;
            if (m_in[2][m_in[2].size() - 1] == "old") {
                if (op_t == Operation::Mul)
                    op_t = Operation::Sqa;
                else {
                    op_t = Operation::Mul;
                    op_v = 2;
                }
            } else {
                op_v = std::stoi(m_in[2][m_in[2].size() - 1]);
            }
            Monkey *monkey_temp = new Monkey{mt_m_idx,
                                             {},
                                             op_t,
                                             op_v,
                                             std::stoi(m_in[3][m_in[3].size() - 1]),
                                             std::stoi(m_in[4][m_in[4].size() - 1]),
                                             std::stoi(m_in[5][m_in[5].size() - 1]),
                                             0,
            };
            m_in[1].erase(m_in[1].begin(), m_in[1].begin() + 4);
            for (const auto& s_nb: m_in[1]) {
                monkey_temp->items.push_back(new Item{std::stoi(s_nb),
                                                      "lel",
                                                      {}});
            }
            mt.monkeys[mt_m_idx++] = monkey_temp;
        } while (pos  != std::string::npos);
        mt.calc_mod_all();
        for (int i = 0; i <10000; ++i) {
            mt.execute_round(false);
        }
        std::cout << "part 2: " << mt.sum_inspection_count() << std::endl;
    }
    return 0;
}

