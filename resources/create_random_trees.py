import random

from anytree import Node, RenderTree


def create_sub_tree(parent: Node, depth, max_children):
    if depth == 0:
        return
    
    parent = Node('', parent)
    count_children = random.randint(1, max_children)
    
    for i in range(0,count_children):
        create_sub_tree(parent, depth -1, max_children)


def create_tree(depth: int, max_children: int):
    root = Node('')

    count_children = random.randint(1, max_children)
    
    for i in range(0,count_children):
        create_sub_tree(root, depth, max_children)
    
    return root


def to_parenthesis(tree:Node):
    parenthesis = '('
    
    for child in tree.children:
        parenthesis += to_parenthesis(child)
    
    parenthesis += ')'
    
    return parenthesis


if __name__ == '__main__':
    random.seed()

    for depth, max_children in [(5, 5), (10, 2), (5, 10), (100, 5)]:
        tree = create_tree(depth, max_children)
    
        #print(to_parenthesis(tree))
    
        file_name = 'parenthesis_depth_{}_max_children_{}.txt'.format(depth,max_children)
    
        with open(file_name, 'w') as file:
            file.write(to_parenthesis(tree))