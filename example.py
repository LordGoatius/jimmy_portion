import networkx as nx
import jimmy_portion as jp

graph = nx.Graph()
graph.add_nodes_from([0, 1, 2, 3, 4, 5, 6])
graph.add_edges_from([(0, 1), (1, 2), (2, 3), (3, 0), (4, 0), (5, 0), (6, 0), (6, 1), (6, 3), (5, 3)])

graph_dict = jp.bf_chromo_coloring([*nx.nodes(graph)], [*nx.edges(graph)])
graph_dict_greedy = jp.greedy_coloring([*nx.nodes(graph)], [*nx.edges(graph)])
graph_dict_rlf = jp.recursive_largest_first([*nx.nodes(graph)], [*nx.edges(graph)])

print(f"Exact\t{graph_dict}")
print(f"Greedy\t{graph_dict_greedy}")
print(f"RLF\t{graph_dict_rlf}")

test = jp.test(9, 36)

bf = list()
greedy = list()
rlf = list()

# 0 = bf, 1 = greedy, 2 = RLf
for algo, nodes, edges, time in test:
    if algo == 0:
        bf.append((nodes, edges, time))
    elif algo == 1:
        greedy.append((nodes, edges, time))
    elif algo == 2:
        rlf.append((nodes, edges, time))

f = open("data.csv", "w")

f.write("Algo, Node Num, Edge Num, Time\n")

for i in range(len(bf)):
    f.write(f"Brute Force, {bf[i][0]}, {bf[i][1]}, {bf[i][2]}\n")
    f.write(f"Greedy, {greedy[i][0]}, {greedy[i][1]}, {greedy[i][2]}\n")
    f.write(f"RLF, {rlf[i][0]}, {rlf[i][1]}, {rlf[i][2]}\n")

f.close()
