from pulp import *

prob = LpProblem("Lucro", LpMaximize)

(t, p, max_brinquedo) = map(lambda s: int(s), input().split())
brinquedos = []
b_var = []
pacotes_brinquedo = []
for i in range(t):
    (l, c) = map(lambda s: int(s), input().split())
    brinquedos.append((l,c))
    var = LpVariable("Brinquedo_" + str(i), 0, c, LpInteger)
    b_var.append(var)
    pacotes_brinquedo.append([var])

p_var = []
pacotes = []
for _ in range(p):
    (i, j, k, l) = map(lambda s: int(s)-1, input().split())
    l += 1
    var = LpVariable("Pacote_" + str(_), 0, min(brinquedos[i][1], brinquedos[j][1], brinquedos[k][1]))
    p_var.append(var)
    pacotes.append(l)
    pacotes_brinquedo[i].append(var)
    pacotes_brinquedo[j].append(var)
    pacotes_brinquedo[k].append(var)

for i in range(t):
    prob += (lpSum(pacotes_brinquedo[i]) <= brinquedos[i][1])

prob += lpSum([b_var[i] * brinquedos[i][0] for i in range(t)] + [p_var[i] * pacotes[i] for i in range(p)]), "LucroTotal"
prob += lpSum([p_var[i] * 3 for i in range(p)] + [b_var[i] for i in range(t)]) <= max_brinquedo

prob.solve()

print(int(pulp.value(prob.objective)))
