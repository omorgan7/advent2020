#include <fstream>
#include <cstdio>
#include <vector>

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

  size_t trees = 0;
  for (size_t x = 0, y = 0; y < height;)
  {
    x += 3;
    x %= width;
    y += 1;
    
    trees += scene[x + y * width] == '#';
  }

  printf("Trees: %zu\n", trees);
}