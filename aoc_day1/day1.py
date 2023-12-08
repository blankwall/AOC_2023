data = open("input.txt").read()

numbers = [
"zero",
"one",
"two",
"three",
"four",
"five",
"six",
"seven",
"eight",
"nine",
]

def get_first(ii):
	for index,i in enumerate(ii):
		if i.isdigit():
			return int(i)
		for k in numbers:
			l = len(k)
			if k in ii[:index+1]:
				return numbers.index(k)

def get_last(ii):
	for index in range(len(ii)-1,-1,-1):
		i = ii[index]
		if i.isdigit():
			return int(i)
		for k in numbers:
			l = len(k)
			if k in ii[index:len(ii)]:
				return numbers.index(k)



# print(get_first("onetwo9two3"))
# print(get_last("onetwo9twothrfee"))
d = "two1nine"
g = "eightwothree"
f = "abcone2threexyz"
# print(get_first(f))

out = 0

from functools import reduce
print(reduce(lambda x,y: x + get_first(y)*10 + get_last(y), data.split("\n"),0))

for i in data.split("\n"):
	out += get_first(i)*10 + get_last(i)

print(out)