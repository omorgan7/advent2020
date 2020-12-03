#include <fstream>
#include <cstdio>
#include <vector>
#include <utility>
#include <array>

int main()
{
  auto input = std::ifstream("input.txt");
  std::vector<char> scene;
  size_t width = 0;
  size_t height = 0;
  for (std::string line; std::getline(input, line);)
  {
    scene.insert(scene.end(), line.begin(), line.end());
    ++height;
    width = std::distance(line.begin(), line.end());
  }

  using p = std::pair<int, int>;
  std::array<p, 5> slopes = {p{1, 1}, p{3, 1}, p{5, 1}, p{7, 1}, p{1, 2}};

  size_t answer = 1;
  for (const auto slope : slopes)
  {
    size_t trees = 0;
    for (size_t x = 0, y = 0; y < height;)
    {
      x += slope.first;
      x %= width;
      y += slope.second;
      
      trees += scene[x + y * width] == '#';
    }
    answer *= trees;
  }

  printf("Trees: %zu\n", answer);
}