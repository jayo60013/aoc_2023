#include <stdio.h>
#include <stdlib.h>

int part1(FILE *);
int part2(FILE *);

int main() {
  FILE *input;

  input = fopen("input", "r");
  if (input == NULL)
    exit(EXIT_FAILURE);

  printf("Part 1: %d\n", part1(input));
  fclose(input);

  // input = fopen("input", "r");
  // if (input == NULL)
  //   exit(EXIT_FAILURE);
  // printf("Part 2: %d\n", part2(input));
  // fclose(input);

  exit(EXIT_SUCCESS);
}

int part1(FILE *input) {
  char *line = NULL;
  size_t len = 0;
  ssize_t read;

  while ((read = getline(&line, &len, input)) != -1) {
    printf("%s", line);
  }

  if (line)
    free(line);
  return 0;
}

int part2(FILE *input) {
  char *line = NULL;
  size_t len = 0;

  while (getline(&line, &len, input) != -1) {
    printf("%s", line);
  }
  free(line);

  return 0;
}
