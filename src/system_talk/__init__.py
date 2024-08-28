from manim import *
import pydot 
import networkx as nx


class Diagram: 

    def __init__(self, path):
        self._graph = nx.petersen_graph() 


    def generate_layout(self):
        layout = nx.nx_agraph.graphviz_layout(self._graph)

class CreateDiagram(Scene):

    def construct(self):
        diagram = Diagram()


        circle = Square()
        circle.set_fill(PINK, opacity=0.5)

        self.add(circle)

        self.wait()


if __name__ == "__main__":
    example = """
digraph Q {

  node [shape=record];


  nd_1   [label = "Node 1"];
  nd_2   [label = "Node 2"];
  nd_3_a [label = "Above Right Node 3"];
  nd_3_l [label = "Left of Node 3"];
  nd_3   [label = "Node 3"];
  nd_3_r [label = "Right of Node 3"];
  nd_4   [label = "Node 4"];


  nd_3_a -> nd_3_r;
  nd_1 -> nd_2 -> nd_3 -> nd_4;

  subgraph cluster_R {

    {rank=same nd_3_l nd_3 nd_3_r}

    nd_3_l -> nd_3 -> nd_3_r [color=grey arrowhead=none];

  }
}
"""
    diagram = Diagram(path=example)
    print(diagram._graph.__dir__())

    print(diagram._graph)
    print(diagram._graph.edges())
    print( nx.nx_agraph.graphviz_layout(diagram._graph))
