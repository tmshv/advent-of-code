#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int partOne() {
	int result = 0;
	// Buffer to hold each line read from stdin
	char line[256];

	// Read lines from stdin until EOF is encountered
	while (fgets(line, sizeof(line), stdin) != NULL) {
		char first;
		char last;

		char c;
		unsigned long len = strlen(line);
		for (int i = 0; i < len; i++) {
			c = line[i];
			if (c >= '1' && c <= '9') {
				first = c;
				break;
			}
		}

		for (int i = len; i >= 0; i--) {
			c = line[i];
			if (c >= '1' && c <= '9') {
				last = c;
				break;
			}
		}

		char str[3] = {first, last, '\0'};
		int calibration = atoi(str);

		printf("%s -> %d\n", line, calibration);

		result += calibration;
	}

	return result;
}

int main() {
	int result;

	result = partOne();
	printf("Part one: %d\n", result);

	return 0;
}
