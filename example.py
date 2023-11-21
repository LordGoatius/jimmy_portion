import networkx as nx
import jimmy_portion as jp

graph = nx.Graph()
graph.add_nodes_from([0, 1, 2, 3])
graph.add_edges_from([(0, 1), (1, 2), (2, 3), (3, 0)])

graph_dict = jp.bf_chromo_coloring([*nx.nodes(graph)], [*nx.edges(graph)])
graph_dict_greedy = jp.greedy_coloring([*nx.nodes(graph)], [*nx.edges(graph)])

print(f"Exact\t{graph_dict}")
print(f"Greedy\t{graph_dict_greedy}")

