import networkx as nx
import jimmy_portion as jp

graph = nx.Graph()
graph.add_nodes_from([0,1,2,3])
graph.add_edges_from([(0,1),(1,2),(2,3),(3,0)])

graph_dict = jp.bf_chromo_coloring([*nx.nodes(graph)], [*nx.edges(graph)])

print(graph_dict)

