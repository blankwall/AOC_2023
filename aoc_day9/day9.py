from itertools import groupby

sequs = [
    kk
    for kk in (j.strip().split(" ") for j in (i for i in open("test.txt").readlines()))
]


def all_equal(iterable):
    g = groupby(iterable)
    return next(g, True) and not next(g, False)


def find_base(seq):
    nr = seq
    parts = [seq]

    while not all_equal(nr):
        nseq = []

        for i in range(1, len(nr)):
            my_diff = nr[i] - nr[i - 1]
            nseq += [my_diff]
        parts += [nseq]
        nr = nseq
    return parts


def process_pt1(seq):
    parts = find_base(seq)
    add_val = 0
    for i in range(len(parts) - 1, -1, -1):
        my_seq = parts[i]

        if add_val == 0:
            add_val = my_seq[len(my_seq) - 1]
        else:
            add_val = add_val + my_seq[len(my_seq) - 1]

        my_seq += [add_val]

    return my_seq[-1]


def process_pt2(seq):
    parts = find_base(seq)

    add_val = -99
    for i in range(len(parts) - 1, -1, -1):
        my_seq = parts[i]

        if add_val == -99:
            add_val = my_seq[0]
        else:
            add_val = my_seq[0] - add_val

        my_seq.insert(0, add_val)

    return my_seq[0]


print(sum([process_pt1([int(j) for j in i]) for i in sequs]))
print(sum([process_pt2([int(j) for j in i]) for i in sequs]))
