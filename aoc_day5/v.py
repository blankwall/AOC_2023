f = open("test.txt").readlines()

seeds = [int(i) for i in f[0].split(":")[1].strip().split(" ")]
nseeds = [seeds[0], seeds[0]+seeds[1] ]

### PARSE INPUT
entries = []
entry = []
operate = False
for i in f[1:]:
	j = i.strip()
	if ":" in j:
		if len(entry) > 0:
			entries += [[ ii for ii in entry if len(ii) > 0]]
		entry = []
		operate = True
		continue

	if not operate:
		continue

	vval = [ int(k) for k in j.split(" ") if k != '']
	entry += [vval]
###

def in_range(val1, range1):
	return val1 >= range1[0] and val1 < range1[1]

def range_len(range1):
	return range1[1] - range1[0]

def convert(seeds,seed_to_soil):
	ranges = [seeds]
	for i in seed_to_soil:
		src_range = (i[0], i[0]+i[2])
		shift_dist = i[1] - i[0]

		if src_range[0] <= seeds[0] and src_range[1] > seeds[1]:
			print("whole shift", src_range, seeds, shift_dist)
			seeds[0] += shift_dist
			seeds[1] += shift_dist

		elif src_range[0] <= seeds[1] and src_range[1] >= seeds[0]:
			# range 3 <-> 5
			# ours is 6 <-> 25
			print("partial shift", src_range, seeds, shift_dist)

			if in_range(src_range[0], seeds):
				new_range = [src_range[0]+shift_dist, seeds[1]+shift_dist]
				seeds[1] = src_range[0]

			elif in_range(src_range[1], seeds):
				new_range = [seeds[0]+shift_dist, src_range[1]+shift_dist]
				seeds[0] = src_range[1]

			print(new_range,seeds)

			ranges += [new_range]

	return ranges

seed_to_soil = [
	[50,98,2],
	[52,50,48],
]
# 

all_ranges = [nseeds]
for i in entries:
	for index,ran in enumerate(all_ranges):
		leng = range_len(ran)
		res = convert(ran, i)
		if range_len(res[0]) != leng:
			all_ranges = res
		# print(res, range_len(res[0]))

print(all_ranges)
# print(convert(nseeds,seed_to_soil))


