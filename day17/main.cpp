#include <iostream>
#include <cstdio>
#include <fstream>
#include <cstring>
#include <string>
#include <vector>
#include <cinttypes>
#include <unordered_set>

// https://www.reddit.com/r/adventofcode/comments/18k9ne5/2023_day_17_solutions/
// https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/12.py
// https://adventofcode.com/2023/day/17
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Using_a_priority_queue

// Look into 'Priority queue' for Djikstra

#define XY(x, y) ((uint32_t)x | ((uint32_t)y << 16u))
#define X(xy) (0xFFFF & ((uint32_t)xy))
#define Y(xy) (0xFFFF & ((uint32_t)xy >> 16u))
#define IDX(xy, w) (Y(xy) * (w) + X(xy))

int main() {
    std::vector<std::vector<uint32_t>> heatmap{};

    std::string line{};
    while(std::getline(std::cin, line)) {
        std::vector<uint32_t> data(line.size());
        for (uint32_t i{}; i < static_cast<uint32_t>(data.size()); ++i) {
            data[i] = static_cast<uint32_t>(line[i] - '0');
        }

        heatmap.push_back(std::move(data));
    }

    uint32_t map_w = heatmap[0].size();
    uint32_t map_h = heatmap.size();
    uint32_t node_count = map_w * map_h;

    // Djikstra
    std::vector<uint32_t> dist(node_count, UINT32_MAX);
    std::vector<uint32_t> prev(node_count, UINT32_MAX);
    dist[0] = 0;

    uint32_t target_xy = XY(map_w - 1u, map_h - 1u);

    std::unordered_set<uint32_t> Q{};
    for (uint32_t y{}; y < map_h; ++y) {
        for (uint32_t x{}; x < map_w; ++x) {
            Q.insert(XY(x, y));
        }
    }

    uint32_t last_dir = UINT32_MAX;
    uint32_t steps_in_dir{};
    while (!Q.empty()) {
        uint32_t uxy = 0;
        uint32_t min_d = UINT32_MAX;
        for (const auto &other_xy : Q) {
            uint32_t i = IDX(other_xy, map_w);
            if(dist[i] < min_d) {
                min_d = dist[i];
                uint32_t y = i / map_w;
                uint32_t x = i - y * map_w;
                uxy = XY(x, y);
            }
        }

        // Reached the target
        if (uxy == target_xy) {
            break;
        }

        Q.erase(uxy);

        uint32_t x = X(uxy);
        uint32_t y = Y(uxy);
        uint32_t u = IDX(uxy, map_w);

        uint32_t v{};
        if (x >= 1u) {
            uint32_t v = IDX(XY(x-1u, y), map_w);
            uint32_t alt = dist[u] + heatmap[y][x-1u];
            if (alt < dist[v]) {
                dist[v] = alt;
                prev[v] = uxy;
            }
        }
        if (x < map_w - 1u) {
            uint32_t v = IDX(XY(x+1u, y), map_w);
            uint32_t alt = dist[u] + heatmap[y][x+1u];
            if (alt < dist[v]) {
                dist[v] = alt;
                prev[v] = uxy;
            }
        }
        if (y >= 1u) {
            uint32_t v = IDX(XY(x, y-1u), map_w);
            uint32_t alt = dist[u] + heatmap[y-1u][x];
            if (alt < dist[v]) {
                dist[v] = alt;
                prev[v] = uxy;
            }
        }
        if (y < map_h - 1u) {
            uint32_t v = IDX(XY(x, y+1u), map_w);
            uint32_t alt = dist[u] + heatmap[y+1u][x];
            if (alt < dist[v]) {
                dist[v] = alt;
                prev[v] = uxy;
            }
        }
    }

    std::vector<std::string> debug{};
    for (const auto &row : heatmap) {
        std::string line(row.size(), '\0');
        for (size_t i{}; i < line.size(); ++i) {
            line[i] = static_cast<char>(row[i] + '0');
        }

        debug.push_back(std::move(line));
    }

    uint32_t p = prev[IDX(target_xy, map_w)];
    while(p != UINT32_MAX) {
        uint32_t y = Y(p);
        uint32_t x = X(p);
        debug[y][x] = 'X';

        p = prev[IDX(p, map_w)];
    }

    for (const auto &line : debug) {
        std::cout << line << '\n';
    }
}