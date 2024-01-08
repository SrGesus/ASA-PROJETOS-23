from pulp import *

prob = LpProblem("Lucro_Brinquedos_e_Pacotes", LpMaximize)

(n, p, limite) = map(lambda s: int(s), input().split())

# Lista de tuplo (l, c, p)
# l - lucro
# c - capacidade
# p - lista de pacotes com brinquedo
brinquedos = []
for x in range(n):
    (l, c) = map(lambda s: int(s), input().split())
    brinquedos.append((l, c, [LpVariable(f"B{x}", 0, c, LpInteger)]))

# Lista de tuplo (l, v)
# l - lucro
# v - variavel Lp
pacotes = []
for x in range(p):
    (i, j, k, l) = map(lambda s: int(s)-1, input().split())
    l += 1
    var = LpVariable(f"P{x}", 0, min(brinquedos[i][1], brinquedos[j][1], brinquedos[k][1]), LpInteger)
    pacotes.append((l, var))
    brinquedos[i][2].append(var)
    brinquedos[j][2].append(var)
    brinquedos[k][2].append(var)

for i in range(n):
    # Restrição Capacidade de cada Brinquedo
    prob += (lpSum(brinquedos[i][2]) <= brinquedos[i][1])
# Restrição Capacidade Total de Brinquedos
prob += lpSum([pacotes[i][1] for i in range(p)]) * 3 + lpSum([brinquedos[i][2][0] for i in range(n)]) <= limite
# Objetivo
prob += lpSum([brinquedos[i][2][0] * brinquedos[i][0] for i in range(n)]) + lpSum([pacotes[i][1] * pacotes[i][0] for i in range(p)]), "LucroTotal"

prob.solve(GLPK(msg=0))

print(int(pulp.value(prob.objective)))
