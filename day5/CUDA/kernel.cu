#include "cuda_runtime.h"
#include "device_launch_parameters.h"

#include <stdio.h>
#include <cinttypes>
#include <array>
#include <vector>
#include <string>
#include <chrono>
#include <sstream>
#include <fstream>

#define MAX_RANGES_PER_MAP 64

#define TIMESTAMP(name) const auto name = std::chrono::high_resolution_clock::now();

struct MapRange {
    uint32_t src_start{}, src_end{}, dst_start{};
};
struct SeedRange {
    uint32_t start{}, len{};
};

int read_file(std::vector<SeedRange>& seeds, std::vector<std::array<MapRange, 64>>& ranges);

__device__ uint32_t map_src_to_dst(uint32_t src, MapRange ranges[], uint32_t range_idx) {
    MapRange* range = &ranges[range_idx * MAX_RANGES_PER_MAP];
    
    for (uint32_t i = 0U; range[i].src_end != 0; ++i) {
        if (src < range[i].src_start) {
            continue;
        }
    
        if (src > range[i].src_end) {
            continue;
        }
    
        return range[i].dst_start + src - range[i].src_start;
    }

    return src;
}

__global__ void iter_ranges_map(uint32_t* min, SeedRange* seed, MapRange ranges[]) {
    uint32_t index = (blockIdx.x * blockDim.x * blockDim.y) + (threadIdx.y * blockDim.x) + threadIdx.x;

    if (index >= seed->len)
        return;

    uint32_t location = 
    map_src_to_dst(
        map_src_to_dst(
            map_src_to_dst(
                map_src_to_dst(
                    map_src_to_dst(
                        map_src_to_dst(
                            map_src_to_dst(
                                seed->start + index, ranges, 0
                            ), ranges, 1
                        ), ranges, 2
                    ), ranges, 3
                ), ranges, 4
            ), ranges, 5
        ), ranges, 6
    );

    if (location < *min) {
        *min = location;
    }
}

void get_min_for_seed_range(uint32_t* minimum, SeedRange* seed, MapRange ranges[], uint32_t seed_range_len) {
    dim3 threadsPerBlock(8U, 4U); // SM is 32 = 8 x 4
    dim3 blockCount(((seed_range_len - 1U) / (threadsPerBlock.x * threadsPerBlock.y)) + 1U);

    iter_ranges_map<<<blockCount, threadsPerBlock>>>(minimum, seed, ranges);
}

int main() {
    std::vector<SeedRange> seeds{};
    std::vector<std::array<MapRange, MAX_RANGES_PER_MAP>> ranges{}; // End MapRange is filled with zeros 

    if (read_file(seeds, ranges)) {
        printf("failed to read file!\n");
        return 1;
    }

    SeedRange* seeds_device{};
    MapRange* ranges_device{};

    cudaMalloc(&seeds_device, sizeof(SeedRange) * seeds.size());
    cudaMemcpy(seeds_device, seeds.data(), sizeof(SeedRange) * seeds.size(), cudaMemcpyHostToDevice);

    cudaMalloc(&ranges_device, sizeof(MapRange) * MAX_RANGES_PER_MAP * ranges.size());
    cudaMemcpy(ranges_device, ranges.data(), sizeof(MapRange) * MAX_RANGES_PER_MAP * ranges.size(), cudaMemcpyHostToDevice);

    uint32_t minimum = UINT32_MAX;
    uint32_t* minimum_device{};

    cudaMalloc(&minimum_device, sizeof(uint32_t));
    cudaMemcpy(minimum_device, &minimum, sizeof(uint32_t), cudaMemcpyHostToDevice);

TIMESTAMP(start)

    for (uint32_t i{}; i < seeds.size(); ++i) {
        get_min_for_seed_range(minimum_device, &seeds_device[i], ranges_device, seeds[i].len);
    }

    cudaMemcpy(&minimum, minimum_device, sizeof(uint32_t), cudaMemcpyDeviceToHost);

TIMESTAMP(end)

    printf("Time: %.02fs\n", std::chrono::duration<float>(end - start).count());
    printf("Result: %u\n", minimum);

    cudaFree(seeds_device);
    cudaFree(ranges_device);
    cudaFree(minimum_device);
}

int read_file(std::vector<SeedRange>& seeds, std::vector<std::array<MapRange, 64>>& ranges) {
    std::ifstream file("input.txt");

    if (!file.is_open()) {
        printf("failed to open file!\n");
        return 1;
    }

    std::string line{};
    std::getline(file, line); // seeds

    line = line.substr(6);
    std::stringstream ss(line);

    uint32_t n0, n1;
    while ((ss >> n0) && (ss >> n1)) {
        seeds.push_back(SeedRange{ n0, n1 });
    }

    std::getline(file, line); // blank space

    ranges.resize(7);

    // 7 Maps
    for (int i{}; i < 7; ++i) {
        std::getline(file, line); // name

        //printf("%d:\n", i);

        uint32_t j{};
        while (true) {
            if (!std::getline(file, line)) break;

            if (line == "") break;

            uint32_t a, b, c;
            if (!sscanf(line.c_str(), "%u %u %u", &a, &b, &c)) {
                printf("sscanf fail!\n");
                return 1;
            }

            ranges[i][j++] = MapRange{ b, b + c - 1, a };
        }
    }

    return 0;
}