#include <fstream>
#include <string>
#include <vector>
#include <array>
#include <unordered_set>
#include <iostream>
#include <chrono>
#include <inttypes.h>

const inline bool is_symbol(const char c)
{    
    // Most of characters are '.' which speeds up the process
    if (c == '.')
        return false;

    return !isdigit(c);
}

struct Coord { uint32_t y, x; };

int main()
{
    std::vector<std::string> map{};

    std::ifstream file("input.txt");

    std::string line{};
    while (std::getline(file, line))
        map.emplace_back(line);
  
    uint32_t map_height = map.size();
    uint32_t map_width = map[0].size();

    uint32_t sum{};

    std::vector<Coord> coords{};
    coords.reserve(8);

    for (uint32_t y = 1; y < map_height - 1; ++y)
    {
        for (uint32_t x = 1; x < map_width - 1; ++x)
        {
            if (is_symbol(map[y][x]))
            {
                coords.clear(); // Optimize vector memory usage
               
                if (isdigit(map[y - 1U][x])) // N
                    coords.push_back({y - 1U, x }); 
                else
                {
                    if (isdigit(map[y - 1U][x - 1U])) coords.push_back({ y - 1U, x - 1U }); // NW
                    if (isdigit(map[y - 1U][x + 1U])) coords.push_back({ y - 1U, x + 1U }); // NE
                }
                
                
                if (isdigit(map[y + 1U][x])) 
                    coords.push_back({y + 1U, x }); // S
                else
                {
                    if (isdigit(map[y + 1U][x - 1U])) coords.push_back({ y + 1U, x - 1U }); // SW
                    if (isdigit(map[y + 1U][x + 1U])) coords.push_back({ y + 1U, x + 1U }); // SE
                }
                
                if (isdigit(map[y][x - 1U])) coords.push_back({y, x - 1U}); // W
                if (isdigit(map[y][x + 1U])) coords.push_back({y, x + 1U}); // E
               
                if (coords.size() == 2)
                {
                    uint32_t mult = 1U;
                    for (const auto& [c_y, c_x] : coords)
                    {
                        std::string digits{ "" };

                        // Go to the left (from c_x)
                        for (int32_t x_iter = c_x; x_iter >= 0 && isdigit(map[c_y][x_iter]); --x_iter)
                            digits.insert(digits.begin(), map[c_y][x_iter]);

                        // Go to the right (from c_x+1)
                        for (uint32_t x_iter = c_x + 1; x_iter < map_width && isdigit(map[c_y][x_iter]); ++x_iter)
                            digits.push_back(map[c_y][x_iter]);

                        mult *= std::stoi(digits);
                    }

                    sum += mult;
                }
            }
        }
    }

    std::cout << sum << '\n';
}