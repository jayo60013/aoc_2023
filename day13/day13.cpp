#include <algorithm>
#include <cstdio>
#include <fstream>
#include <vector>

using std::string;
using std::vector;

vector<vector<string>> parseInput(string);
int findRowOfSymmetry(vector<string>, int);
vector<string> transposeBlock(const vector<string>);
bool checkForSmudge(vector<string>, vector<string>);
int part1(vector<vector<string>>);
int part2(vector<vector<string>>);

int main() {
  std::string filename("input");
  vector<vector<string>> input = parseInput(filename);
  printf("Part 1: %d\n", part1(input));
  printf("Part 2: %d\n", part2(input));
}

int part1(vector<vector<string>> input) {

  int sum = 0;
  for (const auto &block : input) {
    sum += findRowOfSymmetry(block, 1) * 100;
    sum += findRowOfSymmetry(transposeBlock(block), 1);
  }

  return sum;
}

int part2(vector<vector<string>> input) {
  int sum = 0;
  for (const auto &block : input) {
    sum += findRowOfSymmetry(block, 2) * 100;
    sum += findRowOfSymmetry(transposeBlock(block), 2);
  }

  return sum;
}

bool checkForSmudge(vector<string> lhs, vector<string> rhs) {
  int differences = 0;
  for (size_t i = 0; i < lhs.size(); ++i) {
    for (size_t j = 0; j < lhs[0].length(); ++j) {
      differences += (int)(lhs[i][j] != rhs[i][j]);
    }
  }
  return differences == 1;
}

int findRowOfSymmetry(vector<string> block, int part) {
  for (size_t i = 1; i < block.size(); ++i) {

    vector<string> top(block.begin(), block.begin() + i);
    vector<string> bot(block.begin() + i, block.end());

    std::reverse(top.begin(), top.end());

    size_t n = std::min(top.size(), bot.size());
    top.resize(n);
    bot.resize(n);

    if (part == 1) {
      if (top == bot)
        return i;
    } else if (part == 2) {
      if (checkForSmudge(top, bot))
        return i;
    }
  }
  return 0;
}

vector<string> transposeBlock(const vector<string> block) {
  size_t maxLength = block[0].length();

  vector<string> t(maxLength, string(block.size(), ' '));

  for (size_t i = 0; i < block.size(); ++i) {
    for (size_t j = 0; j < block[i].length(); ++j) {
      t[j][i] = block[i][j];
    }
  }

  return t;
}

vector<vector<string>> parseInput(string filename) {
  vector<vector<string>> blocks;
  vector<string> block;
  string line;

  std::ifstream input_file(filename);

  while (getline(input_file, line)) {
    if (line.empty()) {
      blocks.push_back(block);
      block.clear();
    } else {
      block.push_back(line);
    }
  }
  blocks.push_back(block);

  input_file.close();
  return blocks;
}
