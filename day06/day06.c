#include <math.h>
#include <stdio.h>
#include <stdlib.h>

int part1(FILE *);
int part2(FILE *);
int calcRange(double, double, double);

int main() {
  FILE *input;

  input = fopen("input", "r");
  if (input == NULL)
    exit(EXIT_FAILURE);

  printf("Part 1: %d\n", part1(input));
  fclose(input);

  input = fopen("input2", "r");
  if (input == NULL)
    exit(EXIT_FAILURE);
  printf("Part 2: %d\n", part2(input));
  fclose(input);

  exit(EXIT_SUCCESS);
}

int part1(FILE *input) {
  char *line = NULL;
  size_t len = 0;

  int times[4];
  int distances[4];

  getline(&line, &len, input);
  sscanf(line, "Time: %d %d %d %d", &times[0], &times[1], &times[2], &times[3]);

  getline(&line, &len, input);
  sscanf(line, "Distance: %d %d %d %d", &distances[0], &distances[1],
         &distances[2], &distances[3]);
  const size_t RACES_LENGTH = sizeof(times) / sizeof(int);

  int sum = 1;
  for (size_t i = 0; i < RACES_LENGTH; i++) {
    sum *= calcRange(1.f, -times[i], distances[i]);
  }

  free(line);
  return sum;
}

int part2(FILE *input) {
  char *line = NULL;
  size_t len = 0;

  double time;
  getline(&line, &len, input);
  sscanf(line, "Time: %lf", &time);

  double distance;
  getline(&line, &len, input);
  sscanf(line, "Distance: %lf", &distance);

  free(line);
  return calcRange(1.f, -time, distance);
}

int calcRange(double a, double b, double c) {
  double discriminant = b * b - 4 * a * c;
  double x1, x2;

  if (discriminant > 0) {
    x1 = (-b + sqrt(discriminant)) / (2 * a);
    x2 = (-b - sqrt(discriminant)) / (2 * a);
    return abs((int)(ceil(x1) - ceil(x2)));
  } else {
    return 1;
  }
}
