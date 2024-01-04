#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int part1(FILE *);
int part2(FILE *);

typedef struct {
  char *name;
  int value;
} Lookup;

const Lookup lookup[9] = {
    {"one", 1}, {"two", 2},   {"three", 3}, {"four", 4}, {"five", 5},
    {"six", 6}, {"seven", 7}, {"eight", 8}, {"nine", 9},
};
const size_t LOOKUP_SIZE = sizeof(lookup) / sizeof(lookup[0]);

int main() {
  FILE *input;

  input = fopen("input", "r");
  if (input == NULL)
    exit(EXIT_FAILURE);

  printf("Part 1: %d\n", part1(input));
  fclose(input);

  input = fopen("input", "r");
  if (input == NULL)
    exit(EXIT_FAILURE);
  printf("Part 2: %d\n", part2(input));
  fclose(input);

  exit(EXIT_SUCCESS);
}

int part1(FILE *input) {
  char *line = NULL;
  size_t len = 0;

  int first_digit = -1;
  int last_digit = 0;
  int sum = 0;

  while (getline(&line, &len, input) != -1) {
    for (size_t i = 0; line[i] != '\n'; ++i) {
      if (isdigit(line[i])) {
        if (first_digit == -1) {
          first_digit = (int)line[i] - '0';
        }
        last_digit = (int)line[i] - '0';
      }
    }

    sum += (first_digit * 10) + last_digit;
    first_digit = -1;
  }

  free(line);
  return sum;
}

int part2(FILE *input) {
  char *line = NULL;
  size_t len = 0;

  int first_digit = -1;
  int last_digit = 0;
  int sum = 0;

  while (getline(&line, &len, input) != -1) {
    for (size_t i = 0; line[i] != '\n'; i++) {
      for (size_t j = 0; j < LOOKUP_SIZE; j++) {
        // deal with if character is a digit
        if (isdigit(line[i])) {
          if (first_digit == -1) {
            first_digit = (int)line[i] - '0';
          }
          last_digit = (int)line[i] - '0';
          continue;
        }

        // continue if the word can't fit in the remaining characters of the
        // line
        int num_len = strlen(lookup[j].name);
        if (i + num_len > len)
          continue;

        // create substring from the current character with the length of the
        // name of the digit
        char *substr = malloc(num_len + 1);
        strncpy(substr, line + i, num_len);
        substr[num_len] = '\0';

        // compare the substring to the lookup table to see if they match
        if (strcmp(substr, lookup[j].name) == 0) {
          if (first_digit == -1) {
            first_digit = lookup[j].value;
          }
          last_digit = lookup[j].value;
        }
        free(substr);
      }
    }
    sum += (first_digit * 10) + last_digit;
    first_digit = -1;
  }
  free(line);

  return sum;
}
