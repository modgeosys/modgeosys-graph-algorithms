import cupy as cp
import numpy as np
import cudf
import cugraph
import networkx as nx
from concurrent.futures import ThreadPoolExecutor
from scipy.sparse.csgraph import floyd_warshall as fw_cpu

def is_gpu_available():
    try:
        cp.cuda.Device(0).compute_capability
        return True
    except cp.cuda.runtime.CUDARuntimeError:
        return False

use_gpu = is_gpu_available()

def manhattan_distance(p1, p2):
    return cp.abs(p1[0] - p2[0]) + cp.abs(p1[1] - p2[1]) if use_gpu else np.abs(p1[0] - p2[0]) + np.abs(p1[1] - p2[1])

def euclidean_distance(p1, p2):
    return cp.sqrt((p1[0] - p2[0])**2 + (p1[1] - p2[1])**2) if use_gpu else np.sqrt((p1[0] - p2[0])**2 + (p1[1] - p2[1])**2)

# Cable properties (example)
cable_types = [
    {'capacity': 10, 'cost': 1},
    {'capacity': 20, 'cost': 1.8},
    {'capacity': 30, 'cost': 2.5}
]

def calculate_optimal_cables(required_current):
    cable_combinations = []
    for cable in cable_types:
        count = (required_current + cable['capacity'] - 1) // cable['capacity']
        total_cost = count * cable['cost']
        cable_combinations.append((total_cost, count, cable['capacity']))
    # Select the combination with the minimum total cost
    optimal_combination = min(cable_combinations, key=lambda x: x[0])
    return optimal_combination[1], optimal_combination[0]  # Return count and total cost


def create_distance_matrix(nodes, edges, distance_func, use_gpu, required_currents):
    n = len(nodes)
    if use_gpu:
        dist_matrix = cp.full((n, n), cp.inf, dtype=cp.float32)
    else:
        dist_matrix = np.full((n, n), np.inf, dtype=np.float32)

    for (u, v, attrs) in edges:
        dist = distance_func(nodes[u], nodes[v])
        required_current = required_currents.get((u, v), 0)
        num_cables, total_cost = calculate_optimal_cables(required_current)
        dist *= total_cost  # Use the total cost as the weight
        if dist < dist_matrix[u, v]:
            dist_matrix[u, v] = dist
            dist_matrix[v, u] = dist
    return dist_matrix


def compute_metric_closure(terminals, nodes, edges, distance_func, use_gpu, required_currents):
    dist_matrix = create_distance_matrix(nodes, edges, distance_func, use_gpu, required_currents)
    sources, targets, weights = [], [], []
    for i in terminals:
        for j in terminals:
            if i != j and dist_matrix[i, j] < (cp.inf if use_gpu else np.inf):
                sources.append(i)
                targets.append(j)
                weights.append(dist_matrix[i, j])

    if use_gpu:
        df = cudf.DataFrame({'src': sources, 'dst': targets, 'weight': weights})
        G = cugraph.Graph()
        G.from_cudf_edgelist(df, source='src', destination='dst', edge_attr='weight')
        dist_matrix_gpu = cugraph.floyd_warshall(G)
        return dist_matrix_gpu
    else:
        dist_matrix_cpu = np.zeros((len(terminals), len(terminals)), dtype=np.float32)
        for i, src in enumerate(terminals):
            for j, dst in enumerate(terminals):
                dist_matrix_cpu[i, j] = dist_matrix[src, dst]
        return fw_cpu(dist_matrix_cpu, directed=False)


def construct_mst(distance_matrix, terminals, use_gpu):
    sources, targets, weights = [], [], []
    if use_gpu:
        dist_matrix = distance_matrix
        n = len(terminals)
    else:
        dist_matrix = distance_matrix
        n = dist_matrix.shape[0]

    for i in range(n):
        for j in range(i + 1, n):
            if dist_matrix[i, j] < (cp.inf if use_gpu else np.inf):
                sources.append(i)
                targets.append(j)
                weights.append(dist_matrix[i, j])

    if use_gpu:
        df = cudf.DataFrame({'src': sources, 'dst': targets, 'weight': weights})
        G = cugraph.Graph()
        G.from_cudf_edgelist(df, source='src', destination='dst', edge_attr='weight')
        mst = cugraph.minimum_spanning_tree(G)
        return mst
    else:
        G = nx.Graph()
        for src, dst, weight in zip(sources, targets, weights):
            G.add_edge(src, dst, weight=weight)
        mst = nx.minimum_spanning_tree(G)
        return mst


def shortest_path(graph, source, target, weight='weight'):
    return nx.shortest_path(graph, source=source, target=target, weight=weight)


def steiner_tree_approximation(nodes, terminals, edges, distance_func_gpu, distance_func_cpu, use_gpu, required_currents):
    # Step 1: Compute metric closure using the specified distance function
    distance_func = distance_func_gpu if use_gpu else distance_func_cpu

    metric_closure = compute_metric_closure(terminals, nodes, edges, distance_func, use_gpu, required_currents)

    # Step 2: Construct MST from the metric closure
    mst = construct_mst(metric_closure, terminals, use_gpu)

    # Step 3: Steiner node addition
    original_graph = nx.MultiGraph()
    for (u, v, attrs) in edges:
        distance = distance_func(nodes[u], nodes[v])
        required_current = required_currents.get((u, v), 0)
        num_cables, total_cost = calculate_optimal_cables(required_current)
        original_graph.add_edge(u, v, weight=distance, num_cables=num_cables, total_cost=total_cost)

    steiner_tree = nx.MultiGraph()

    def add_shortest_path(u, v):
        path = shortest_path(original_graph, u, v)
        for i in range(len(path) - 1):
            u, v = path[i], path[i + 1]
            num_cables = original_graph[u][v][0]['num_cables']
            total_cost = original_graph[u][v][0]['total_cost']
            steiner_tree.add_edge(u, v, weight=original_graph[u][v][0]['weight'], num_cables=num_cables, total_cost=total_cost)

    with ThreadPoolExecutor() as executor:
        futures = [executor.submit(add_shortest_path, u, v) for u, v in mst.edges()]
        for future in futures:
            future.result()

    return steiner_tree


# Example usage
if use_gpu:
    nodes = cp.array([(0, 0), (0, 1), (1, 0), (1, 1), (2, 2)], dtype=cp.float32)
else:
    nodes = np.array([(0, 0), (0, 1), (1, 0), (1, 1), (2, 2)], dtype=np.float32)

terminals = [0, 1, 2, 3, 4]
edges = [
    (0, 1, {'weight': 1}),
    (1, 2, {'weight': 2}),
    (2, 3, {'weight': 3}),
    (3, 4, {'weight': 4}),
    (4, 0, {'weight': 5}),
    (0, 2, {'weight': 1.5}),
    (1, 3, {'weight': 2.5})
]
required_currents = {
    (0, 1): 15,
    (1, 2): 25,
    (2, 3): 20,
    (3, 4): 10,
    (4, 0): 30,
    (0, 2): 20,
    (1, 3): 15
}

steiner_tree_manhattan = steiner_tree_approximation(nodes, terminals, edges, manhattan_distance, manhattan_distance, use_gpu, required_currents)
steiner_tree_euclidean = steiner_tree_approximation(nodes, terminals, edges, euclidean_distance, euclidean_distance, use_gpu, required_currents)
