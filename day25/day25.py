import networkx as nx

g = nx.Graph()

for line in open("input"):
    left, right = line.split(":")
    for node in right.strip().split():
        g.add_edge(left, node)

g.remove_edges_from(nx.minimum_edge_cut(g))
a, b = nx.connected_components(g)

print(len(a) * len(b))
