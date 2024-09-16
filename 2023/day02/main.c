#include <ctype.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LINE_LENGTH 1000
#define MAX_GAMES 100
#define MAX_CUBES 100
#define MAX_ROLLS 100

const size_t RED = 0;
const size_t GREEN = 1;
const size_t BLUE = 2;

const char *COLORS[] = {"red", "green", "blue"};

typedef size_t Color;
typedef struct Cube {
	size_t count;
	Color color;
} Cube;

typedef struct Roll {
	size_t len;
	Cube *cubes;
} Roll;

typedef struct Game {
	size_t id;
	size_t rolls_len;
	Roll *rolls;
} Game;

typedef struct MatchResult {
	bool ok;
	char *value;
} MatchResult;

typedef struct MatchResultNumber {
	bool ok;
	size_t value;
} MatchResultNumber;

// Define structures for tokens and AST nodes
typedef struct {
	char *type;
	char *value;
} Token;

Token *make_token(const char *type, const char *value) {
	Token *token = (Token *)malloc(sizeof(Token));
	token->type = (char *)malloc(strlen(type) + 1);
	token->value = (char *)malloc(strlen(value) + 1);
	strcpy(token->type, type);
	strcpy(token->value, value);
	return token;
}

typedef struct Parser {
	size_t tokens_total;
	size_t current;
	Token **tokens;
} Parser;

int parser_step(Parser *parser) {
	if (parser->current >= parser->tokens_total) {
		return 1;
	}
	parser->current++;
	return 0;
}

int parser_has_current_token(Parser *parser) { return parser->current < parser->tokens_total; }

Token *parser_get_current_token(Parser *parser) { return parser->tokens[parser->current]; }

void free_token(Token *token) {
	free(token->type);
	free(token->value);
	free(token);
}

// Tokenization
Token **tokenize(const char *game_string, size_t *token_count) {
	Token **tokens = NULL;
	*token_count = 0;
	int i = 0;

	while (game_string[i]) {
		if (isdigit(game_string[i])) {
			char num[16] = {0};
			int index = 0;
			while (isdigit(game_string[i])) {
				num[index++] = game_string[i++];
			}
			tokens = (Token **)realloc(tokens, sizeof(Token *) * (*token_count + 1));
			tokens[*token_count] = make_token("NUMBER", num);
			(*token_count)++;
		} else if (isalpha(game_string[i])) {
			char word[16] = {0};
			int index = 0;
			while (isalpha(game_string[i])) {
				word[index++] = game_string[i++];
			}
			tokens = (Token **)realloc(tokens, sizeof(Token *) * (*token_count + 1));
			tokens[*token_count] = make_token("WORD", word);
			(*token_count)++;
		} else if (strchr(";,:", game_string[i])) {
			char delim[2] = {game_string[i++], '\0'};
			tokens = (Token **)realloc(tokens, sizeof(Token *) * (*token_count + 1));
			tokens[*token_count] = make_token("WORD", delim);
			(*token_count)++;
		} else if (isspace(game_string[i])) {
			i++; // Ignore whitespace
		} else {
			fprintf(stderr, "Unknown character: %c\n", game_string[i]);
			exit(1);
		}
	}
	return tokens;
}

MatchResultNumber match_number(Parser *parser) {
	MatchResultNumber result;
	result.ok = false;
	result.value = 0;

	if (!parser_has_current_token(parser)) {
		return result;
	}

	Token *current_token = parser_get_current_token(parser);
	if (strcmp(current_token->type, "NUMBER") == 0) {
		result.ok = true;
		result.value = strtoul(current_token->value, NULL, 10);
		parser_step(parser);
	}

	return result;
}

MatchResult match_keyword(Parser *parser, const char *value) {
	MatchResult result;
	result.ok = false;

	if (!parser_has_current_token(parser)) {
		return result;
	}

	Token *token = parser_get_current_token(parser);

	if (strcmp(token->type, "WORD") == 0 && strcmp(token->value, value) == 0) {
		result.ok = true;
		result.value = token->value;
		parser_step(parser);
	}

	return result;
}

MatchResult match_any(Parser *parser, char **options, int option_count) {
	MatchResult result;
	result.ok = false;
	for (int i = 0; i < option_count; i++) {
		Token *token = parser_get_current_token(parser);
		if (token && strcmp(token->value, options[i]) == 0) {
			result.ok = true;
			result.value = token->value;

			parser_step(parser);
			return result;
		}
	}
	fprintf(stderr, "Expected one of: ");
	for (int i = 0; i < option_count; i++) {
		fprintf(stderr, "%s", options[i]);
		if (i < option_count - 1)
			fprintf(stderr, ", ");
	}
	return result;
}

// Corrected match_color function using match_any
Color match_color(Parser *parser) {
	MatchResult result = match_any(parser, (char **)COLORS, 3);
	if (!result.ok) {
		return -1;
	}
	if (strcmp(result.value, "red") == 0) {
		return RED;
	}
	if (strcmp(result.value, "green") == 0) {
		return GREEN;
	}
	if (strcmp(result.value, "blue") == 0) {
		return BLUE;
	}
	return -1;
}

Parser *new_parser(Token **tokens, int total) {
	static Parser parser;
	parser.tokens_total = total;
	parser.tokens = tokens;
	parser.current = 0;
	return &parser;
}

Cube *match_cube(Parser *parser) {
	Cube *cube = malloc(sizeof(Cube));
	MatchResultNumber n = match_number(parser);
	if (!n.ok) {
		return NULL;
	}
	cube->count = n.value;
	cube->color = match_color(parser);
	return cube;
}

Roll *match_roll(Parser *parser) {
	Roll *roll = malloc(sizeof(Roll));
	roll->len = 0;
	roll->cubes = malloc(sizeof(Cube) * MAX_CUBES);

	size_t cube_count = 0;
	while (true) {
		roll->cubes[cube_count] = *match_cube(parser);
		cube_count++;

		MatchResult delim = match_keyword(parser, ",");
		if (!delim.ok) {
			break;
		}
	}
	roll->len = cube_count;
	return roll;
}

void free_roll(Roll *roll) {
	free(roll->cubes);
	free(roll);
}

Game *match_game(Parser *parser) {
	MatchResult tmp;
	Game *game = malloc(sizeof(Game));

	tmp = match_keyword(parser, "Game");
	if (!tmp.ok) {
		return NULL;
	}
	MatchResultNumber id = match_number(parser);
	if (!id.ok) {
		return NULL;
	}
	game->id = id.value;
	tmp = match_keyword(parser, ":");
	if (!tmp.ok) {
		return NULL;
	}

	game->rolls = malloc(sizeof(Roll) * MAX_ROLLS);
	size_t rolls_count = 0;
	while (true) {
		Roll *roll = match_roll(parser);
		if (roll == NULL) {
			break;
		}
		game->rolls[rolls_count++] = *roll;

		MatchResult delim = match_keyword(parser, ";");
		if (!delim.ok) {
			break;
		}
	}
	game->rolls_len = rolls_count;

	return game;
}

void free_game(Game *game) {
	for (int i = 0; i < game->rolls_len; i++) {
		free(game->rolls[i].cubes);
	}

	free(game->rolls);
	free(game);
}

Game **read_input(size_t *count) {
	Game **games = malloc(sizeof(Game *) * MAX_GAMES);
	char line[MAX_LINE_LENGTH];
	*count = 0;

	while (fgets(line, sizeof(line), stdin) != NULL) {
		size_t token_count = 0;
		Token **tokens = tokenize(line, &token_count);

		Parser *parser = new_parser(tokens, token_count);
		Game *game = match_game(parser);

		if (game == NULL) {
			fprintf(stderr, "Failed to parse Game");
			exit(1);
		}

		// Free allocated memory
		for (int i = 0; i < token_count; i++) {
			free_token(tokens[i]);
		}
		free(tokens);

		games[*count] = game;
		*count = *count + 1;
	}

	return games;
}

size_t max(size_t a, size_t b) {
	if (a > b) {
		return a;
	}
	return b;
}

size_t part_one() {
	size_t result = 0;
	size_t games_total = 0;
	Game **games = read_input(&games_total);

	// only 12 red cubes, 13 green cubes, and 14 blue cubes
	const size_t R = 12;
	const size_t G = 13;
	const size_t B = 14;

	for (int g = 0; g < games_total; g++) {
		size_t reds = 0;
		size_t greens = 0;
		size_t blues = 0;
		Game *game = games[g];
		for (int r = 0; r < game->rolls_len; r++) {
			Roll roll = game->rolls[r];
			for (int c = 0; c < roll.len; c++) {
				Cube cube = roll.cubes[c];
				switch (cube.color) {
				case RED:
					reds = max(reds, cube.count);
					break;
				case GREEN:
					greens = max(greens, cube.count);
					break;
				case BLUE:
					blues = max(blues, cube.count);
					break;
				}
			}
		}
		if (reds <= R && greens <= G && blues <= B) {
			result += game->id;
		}
	}

	// Free allocated memory
	for (int i = 0; i < games_total; i++) {
		free_game(games[i]);
	}

	return result;
}

size_t part_two() {
	size_t result = 0;
	size_t games_total = 0;
	Game **games = read_input(&games_total);

	for (int g = 0; g < games_total; g++) {
		size_t reds = 0;
		size_t greens = 0;
		size_t blues = 0;
		Game *game = games[g];
		for (int r = 0; r < game->rolls_len; r++) {
			Roll roll = game->rolls[r];
			for (int c = 0; c < roll.len; c++) {
				Cube cube = roll.cubes[c];
				switch (cube.color) {
				case RED:
					reds = max(reds, cube.count);
					break;
				case GREEN:
					greens = max(greens, cube.count);
					break;
				case BLUE:
					blues = max(blues, cube.count);
					break;
				}
			}
		}
		size_t power = reds * greens * blues;
		result += power;
	}

	// Free allocated memory
	for (int i = 0; i < games_total; i++) {
		free_game(games[i]);
	}

	return result;
}

int get_part(char **argv) {
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
	int part = get_part(argv);
	switch (part) {
	case 1:
		result = part_one();
		printf("Part one: %d\n", result);
		break;
	case 2:
		result = part_two();
		printf("Part two: %d\n", result);
		break;
	default:
		fprintf(stderr, "Invalid argument: %s\n", argv[1]);
		return 1;
	}

	return 0;
}
