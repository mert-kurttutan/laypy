from laypy import Digraph

def test_hello():
    graph = Digraph()
    graph.node('A', 'Node A', fillcolor='red')
    graph.render(format='png', file_name='test_hello')