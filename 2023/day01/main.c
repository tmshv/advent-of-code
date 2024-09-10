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

char convertTextToDigit(char *text) {
	if (strcmp(text, "one") == 0) {
		return '1';
	} else if (strcmp(text, "two") == 0) {
		return '2';
	} else if (strcmp(text, "three") == 0) {
		return '3';
	} else if (strcmp(text, "four") == 0) {
		return '4';
	} else if (strcmp(text, "five") == 0) {
		return '5';
	} else if (strcmp(text, "six") == 0) {
		return '6';
	} else if (strcmp(text, "seven") == 0) {
		return '7';
	} else if (strcmp(text, "eight") == 0) {
		return '8';
	} else if (strcmp(text, "nine") == 0) {
		return '9';
	}
	return '0';
}

// kjxone -> k[jxone] -> jxone -> no
// kjxone -> kj[xone] -> xone  -> no
// kjxone -> kjx[one] -> one   -> bingo!
char convertSuffixTextToDigit(char *text, int *offset) {
	char buffer[MAX_LINE_LENGTH];
	int len = strlen(text);
	for (int i = 0; i < len; i++) {
		int size = len - i;
		strncpy(buffer, text + i, size);
		buffer[size] = '\0';

		char digit = convertTextToDigit(buffer);
		if (digit != '0') {
			*offset = i;
			return digit;
		}
	}

	return '0';
}

int partTwo() {
	int records;
	char **input = readInput(&records);

	// Patch input for part two
	// 1. Replace first leter with digit
	// 2. one -> 1ne
	// 3. twone -> 2w1ne
	// 4. jxone6 -> jx1ne6
	//
	// Example of patch:
	// 7pqrstsixteentwone
	// 7pqrst6ixteen2w1ne
	char buffer[MAX_LINE_LENGTH];
	for (int i = 0; i < records; i++) {
		char *line = input[i];

		int digitStartIndex = 0;
		int offset = 0;
		for (int j = 0; j < strlen(line); j++) {
			if (line[j] >= '1' && line[j] <= '9') {
				digitStartIndex = j + 1;
				continue;
			}

			int size = j - digitStartIndex + 1;
			strncpy(buffer, line + digitStartIndex, size);
			buffer[size] = '\0';

			char digit = convertSuffixTextToDigit(buffer, &offset);
			if (digit != '0') {
				digitStartIndex += offset;
				line[digitStartIndex] = digit;

				// Move next iteration to the next index after replaced
				j = digitStartIndex + 1;
			}
		}

		// Check last buffer after iteration stops
		char digit = convertSuffixTextToDigit(buffer, &digitStartIndex);
		if (digit != '0') {
			digitStartIndex += offset;
			line[digitStartIndex] = digit;
		}
	}

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
		result = partTwo();
		printf("Part two: %d\n", result);
		break;
	default:
		fprintf(stderr, "Invalid argument: %s\n", argv[1]);
		return 1;
	}

	return 0;
}
