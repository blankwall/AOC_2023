m = open("./my_maze.txt").readlines()


class Maze:
    def __init__(self, m):
        self.galaxy = []
        self.maze = []
        self.expand_row = []
        self.expand_col = []
        for i in m:
            self.maze += [[j for j in i.strip()]]

    def expand(self):
        to_expand_row = []
        to_expand_col = []
        for index, row in enumerate(self.maze):
            if "#" not in row:
                self.expand_row += [index]

        for index, col in enumerate(range(len(self.maze[0]))):
            expand = True
            for row in range(len(self.maze)):
                if self.maze[row][col] == "#":
                    expand = False
                    self.galaxy += [(row, col)]

            if expand:
                self.expand_col += [col]

    def generate_pairs(self):
        pairs = []
        for index, i in enumerate(self.galaxy):
            for j in self.galaxy[index + 1 :]:
                pairs += [(i, j)]
        self.pairs = pairs
        return self.pairs

    def distance_pair(self, p1, p2, f=1):
        final = 0
        for i in self.expand_col:
            if (p1[1] < i and p2[1] >= i) or (p2[1] < i and p1[1] >= i):
                final += f

        for i in self.expand_row:
            if (p1[0] < i and p2[0] >= i) or (p2[0] < i and p1[0] >= i):
                final += f

        x = abs(p1[0] - p2[0])
        y = abs(p1[1] - p2[1])
        return x + y + final

    def solve(self):
        summy = sum([self.distance_pair(*i) for i in self.pairs])
        print(f"Part1: {summy}")
        f = 1000000 - 1
        summy = sum([self.distance_pair(*i, f) for i in self.pairs])
        print(f"Part2: {summy}")

    @staticmethod
    def test():
        maze = Maze(open("./maze.txt").readlines())
        maze.expand()
        maze.generate_pairs()
        assert maze.distance_pair((5, 1), (9, 4)) == 9
        assert maze.distance_pair((0, 3), (8, 7)) == 15
        assert maze.distance_pair((2, 0), (6, 9)) == 17
        assert maze.distance_pair((9, 0), (9, 4)) == 5


Maze.test()
g = Maze(m)
g.expand()
g.generate_pairs()
g.solve()
