f = open("input.txt").readlines()

# seeds = [int(i) for i in f[0].split(":")[1].strip().split(" ")]

seeds = [79,80,81,82,83]

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
	# break

print(seeds)


def convert(seeds,seed_to_soil):
	soil_map = [ i for i in seeds]
	for i in seed_to_soil:
		dest_range = i[0]
		source_range = i[1]
		my_range = i[2]


		range1 = range(dest_range,dest_range+my_range)#soil
		range2 = range(source_range,source_range+my_range) #seed

		for index,j in enumerate(seeds):
			if j in range2:
				old_val = range2.index(j)
				new_val = dest_range + old_val
				# print(new_val)
				soil_map[index] = new_val
	return soil_map

seed_to_soil = [
	[50,98,2],
	[52,50,48],
]

# print(entries[0])

# nseeds = convert(seeds,seed_to_soil)
# print(nseeds)

# gseeds = convert(seeds,entries[0])
# print(entries[0] == seed_to_soil)
# print(gseeds)
# soil_fert = [
# [0 ,15 ,37],
# [37, 52, 2],
# [39, 0 ,15],
# ]

# fert = convert(seeds,soil_fert)
# print(fert)
# print(entries)

# seeds = [81, 53, 57, 52]
# print(entries[2])


for i in entries:
	seeds = convert(seeds,i)
	# print(seeds)
	
print(min(seeds))

# print("SOIL",list(range1))
# print("SEED",list(range2))