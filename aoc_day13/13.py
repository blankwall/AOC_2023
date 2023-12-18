def difference(a,b):
	return len(list(filter(lambda x: x[0] != x[1], zip(a,b))))


def is_reflected(board):
	for row in range(1,len(board)):
		before = list(reversed(board[:row]))
		after = board[row:]
		if all([i == v for i,v in zip(before,after)]):
			# print(list(zip(before,after)))
			# for i in zip(before,after):
			# 	print("".join(list(i[0])))
			# 	print("".join(list(i[1])))
			return row
	return 0

def part1(boards):
	summy = 0
	for i in boards:
		horiz_reflect = is_reflected(i)
		if horiz_reflect:
			print("horizontal")
			summy += (horiz_reflect*100)
		else:
			print("vertical")
			new_board = list(zip(*i))
			vert_reflect = is_reflected(new_board)
			if not vert_reflect:
				raise ValueError("No reflection")

			summy += vert_reflect
	return summy


boards = [i.split("\n") for i in open("my_input.txt").read().split("\n\n")]

print(part1(boards))
# print(difference("AED", "ABD"))