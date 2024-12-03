#include <iostream>
#include <fstream>
#include <string>
#include <vector>

int main()
{
    std::ifstream file("input.txt");
    std::vector<std::string> lines{};

    std::string _line{};
    while (std::getline(file, _line))
        lines.emplace_back(_line);
   
    uint32_t sum{};

    for (auto& line : lines)
    {
        char left = line[line.find_first_of("123456789")];
        char right = line[line.find_last_of("123456789")];

        sum += ((left - '0') * 10 + (right - '0'));
    }
   
    std::cout << sum << '\n';
}