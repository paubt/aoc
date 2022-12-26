//
// Created by user on 12/16/22.
//
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <optional>
#include <cassert>

struct Directory;

class File
{
public:
    File(std::string n, Directory* pd, std::string fe, unsigned long fs): name(n), parent_dir(pd), file_ending(fe), file_size(fs){};
    std::string name;
    Directory* parent_dir;
    std::string file_ending;
    unsigned long file_size;

};

class Directory
{
public:
    std::string name;
    Directory* parent_dir;
    std::vector<Directory*> child_dirs;
    std::vector<File> files;
    std::optional<unsigned long> size;
};

unsigned long int calc_size(File* f)
{
    return f->file_size;
}

unsigned long int calc_size(Directory* dir)
{
    unsigned long int sum = 0;
    for (auto sub_dir: dir->child_dirs)
    {
        sum += calc_size(sub_dir);
    }
    for (auto f: dir->files)
    {
        sum += calc_size(&f);
    }
    dir->size = std::make_optional(sum);
    return sum;
}

unsigned long int sum_size_smaller_x(Directory* dir, unsigned long int smaller_x)
{
    unsigned long int sum = 0;
    for (auto sub_dir: dir->child_dirs)
    {
        sum += sum_size_smaller_x(sub_dir, smaller_x);
    }
    if (!dir->size.has_value())
        calc_size(dir);
    if (dir->size.value() < smaller_x)
        sum += dir->size.value();
    return sum;
}

std::vector<Directory*> get_dir_vec(Directory* root_dir)
{
    std::vector<Directory*> all_dirs{root_dir};
    for (auto sub_dir: root_dir->child_dirs)
    {
        auto sub_dir_vec = get_dir_vec(sub_dir);
        all_dirs.insert(all_dirs.cend(), sub_dir_vec.begin(), sub_dir_vec.end());
    }
    return all_dirs;
}

int main() {
    // Part 1.
    {
        std::ifstream infile("../../data/day07/input.txt");
        std::string line;
        std::vector<std::vector<std::string>> instructions;
        while (std::getline(infile, line))
        {
            std::istringstream iss(line);
            std::vector<std::string> v_temp;
            std::string s_temp;
            while(std::getline(iss, s_temp, ' '))
                v_temp.push_back(s_temp);
            instructions.push_back(std::move(v_temp));
        }
        auto* current_dir = new Directory{"root", nullptr, {},{}, std::nullopt};
        auto* root_dir = current_dir;
        for (auto inst: instructions)
        {
            if (inst[0] == "$" && inst[1] == "cd")
            {
                if (inst[2] == "/")
                {
                    current_dir = root_dir;
                    continue;
                }
                if (inst[2] == "..")
                {
                    assert(current_dir->name != "name");
                    current_dir = current_dir->parent_dir;
                    continue;
                }
                auto it = std::find_if(current_dir->child_dirs.cbegin(),
                                    current_dir->child_dirs.cend(),
                                    [&inst](auto& ele){ return ele->name == inst[2];});
                // Unknown Dir -> make new one and move inside it
                if (it == current_dir->child_dirs.cend())
                {
                    auto* new_dir = new Directory{inst[2], current_dir, {}, {}, std::nullopt};
                    current_dir->child_dirs.push_back(current_dir);
                    current_dir = new_dir;
                    continue;
                }
                else // Known Dir -> move into it
                {
                    current_dir = *it;
                    continue;
                }
            }
            else if (inst[0] == "$" && inst[1] == "ls")
            {
                continue;
            }
            else if (inst[0] == "dir")
            {
                assert(current_dir->child_dirs.cend() == std::find_if(current_dir->child_dirs.cbegin(),
                                                                      current_dir->child_dirs.cend(),
                                                                      [&inst](auto& ele){return ele->name == inst[1];}));
                auto* new_dir = new Directory{inst[1], current_dir, {}, {}, std::nullopt};
                current_dir->child_dirs.push_back(new_dir);
            }
            else
            {
                // TODO Here is das mit dem finden des Substrings nach dem Punkt noch Problematisch
                auto f = std::find(inst[1].begin(),inst[1].end(), '.');
                current_dir->files.emplace_back(inst[1],
                                                current_dir,
                                                inst[1].substr( std::distance(inst[1].begin(), f)),
                                                std::stoi(inst[0]));
            }
        }
        // Calc the size of each folder from bottom to top.
        calc_size(root_dir);
        // Sum all directorys with size < 100.000.
        std::cout << "part 1 solution: "<< sum_size_smaller_x(root_dir, 100000) << std::endl;
        // Part Two.
        // Calc needed space.
        auto needed_space = 30000000 - ( 70000000 - root_dir->size.value());
        // Rank by Size.
        auto dir_v = get_dir_vec(root_dir);
        std::sort(dir_v.begin(), dir_v.end(), [](Directory* a, Directory* b) {
            return a->size.value() < b->size.value();
        });
        // Take smallest that satisfies > needed space.
        auto f_it = std::find_if(dir_v.begin(),dir_v.end(), [&needed_space](Directory* dir) {
            return dir->size.value() > needed_space;
        });
        std::cout << "Part 2: " << (*f_it)->size.value() << std::endl;
    }
    return 0;
}

