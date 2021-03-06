#include <cstdio>
#include <fstream>
#include <vector>
#include <string>

std::vector<int> numbersFromStream(std::ifstream& s)
{
    std::vector<int> numbers;
    for (std::string line; std::getline(s, line);)
    {
        numbers.push_back(std::stoi(line));
    }

    return numbers;
}

int main()
{

    auto s = std::ifstream("input.txt");
    const auto numbers = numbersFromStream(s);

    for (size_t i = 0, I = numbers.size(); i < I; ++i)
    {
        for (size_t j = i + 1; j < I; ++j)
        {
            for (size_t k = j + 1; k < I; ++k)
            {
                const auto sum = numbers[i] + numbers[j] + numbers[k];
                if (sum == 2020) {
                    printf("Numbers: %zu %zu %zu produced: %zu\n", numbers[i], numbers[j], numbers[k], numbers[i] * numbers[j] * numbers[k]);
                    return EXIT_SUCCESS;
                }
            }
        }
    }

    printf("No matching numbers found?\n");
}