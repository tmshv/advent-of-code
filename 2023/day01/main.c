#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LINES 10000
#define MAX_LINE_LENGTH 256

char **readInput(int *lineCount) {
	char **lines = (char **)malloc(MAX_LINES * sizeof(char *));
	char line[MAX_LINE_LENGTH];
	*lineCount = 0;

	while (fgets(line, sizeof(line), stdin) != NULL) {
		lines[*lineCount] = (char *)malloc((strlen(line) + 1) * sizeof(char));
		strcpy(lines[*lineCount], line);
		(*lineCount)++;
	}

	return lines;
}

int solve(char **input, int records) {
	int result = 0;
	for (int i = 0; i < records; i++) {
		char first, last;
		int len = strlen(input[i]);
		for (int j = 0; j < len; j++) {
			if (input[i][j] >= '1' && input[i][j] <= '9') {
				first = input[i][j];
				break;
			}
		}

		for (int j = len - 1; j >= 0; j--) {
			if (input[i][j] >= '1' && input[i][j] <= '9') {
				last = input[i][j];
				break;
			}
		}

		char str[3] = {first, last, '\0'};
		int calibration = atoi(str);
		result += calibration;
	}
	return result;
}

int partOne() {
	int records;
	char **input = readInput(&records);
	int result = solve(input, records);

	for (int i = 0; i < records; i++) {
		free(input[i]);
	}
	free(input);

	return result;
}

int getPart(char **argv) {
	if (strcmp(argv[1], "one") == 0) {
		return 1;
	} else if (strcmp(argv[1], "two") == 0) {
		return 2;
	} else {
		return -1;
	}
}

int main(int argc, char **argv) {
	if (argc < 2) {
		fprintf(stderr, "Usage: %s <one|two>\n", argv[0]);
		return 1;
	}

	int result;
	int part = getPart(argv);
	switch (part) {
	case 1:
		result = partOne();
		printf("Part one: %d\n", result);
		break;
	case 2:
		printf("Part two is not implemented yet.\n");
		break;
	default:
		fprintf(stderr, "Invalid argument: %s\n", argv[1]);
		return 1;
	}

	return 0;
}
