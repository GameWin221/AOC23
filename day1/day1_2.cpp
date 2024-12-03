#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <array>
#include <unordered_map>

int main()
{
    std::ifstream file("input.txt");

    std::vector<std::string> lines{};

    std::unordered_map<std::string, uint32_t> word_to_num{
        {"one", 1U},
        {"two", 2U},
        {"three", 3U},
        {"four", 4U},
        {"five", 5U},
        {"six", 6U},
        {"seven", 7U},
        {"eight", 8U},
        {"nine", 9U},
    };

    std::string _line{};
    while (std::getline(file, _line))
        lines.emplace_back(_line);
   
    size_t sum{};

    for (auto& line : lines)
    {
        // 1. Find all positions of one, two, three, four...
        std::array<std::vector<size_t>, 9> positions{};
        
        for (const auto& [word, num] : word_to_num)
        {
            size_t j{}, pos{};
            while ((pos = line.find(word, j)) != std::string::npos)
            { 
                positions[num-1U].push_back(pos);
                j = pos + word.size(); // + 1
            }
        }

        // 2. Replace all occurences
        for (uint_fast32_t i = 0U; i < 9U; ++i)
            for (const auto& pos : positions[i])
                line[pos + 1] = '0' + (char)i + 1;

        // 3. Find first and last
        char left = line[line.find_first_of("123456789")];
        char right = line[line.find_last_of("123456789")];

        sum += (size_t)((left - '0') * 10 + (right - '0'));
    }
   
    std::cout << sum << '\n';
}