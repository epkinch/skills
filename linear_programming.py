import pulp

TEST_CASES = [[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
              [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
              [0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0],
              [0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0],
              [0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1],
              [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
              [1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0],
              [0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
              [0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1]]

rows = len(TEST_CASES)
cols = len(TEST_CASES[0])
problem = pulp.LpProblem("ECE322", pulp.LpMinimize)
x = pulp.LpVariable.dicts("table", range(rows), lowBound=0, upBound=1, cat=pulp.LpInteger)

problem += pulp.lpSum(x[i] for i in range(rows))

for j in range(cols):
    problem += pulp.lpSum(x[i] * TEST_CASES[i][j] for i in range(rows)) >= 1

problem.solve()

print(f"The chosen tables are out of a total of {rows}:")
for i in range(rows):
    if x[i].value() == 1.0:
        print(i+1, TEST_CASES[i])