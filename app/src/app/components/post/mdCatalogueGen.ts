export class Node<K> {
  data: K;
  children: Node<K>[];

  constructor(data: K) {
    this.data = data;
    this.children = [];
  }

  addChildren(data: K): number {
    return this.children.push(new Node<K>(data)) - 1;
  }

  lastChild(): Node<K> | undefined {
    return this.children[this.children.length - 1];
  }
}

export class Tree<K> {
  root: Node<K>;

  constructor(data: K) {
    this.root = new Node<K>(data);
  }
}

export interface ILine {
  content: string;
  length: number;
}


export const genCatalogue = (lines: string[]): Tree<ILine> => {
  const tree = new Tree({content: '', length: 0});

  let parent = tree.root;
  let current = tree.root;
  const reg = /^#+/;
  const parentHistory = [];
  for (let i = 0; i < lines.length; i++) {
    // console.log(lines[i], current, parent);
    const res = reg.exec(lines[i]);
    if (res && res.length === 1) {
      const sharpLen = res[0].length;
      while (true) {
        if (i === 0) {
          const index = parent.addChildren({length: sharpLen, content: lines[i]});
          parentHistory.push(parent);
          current = parent.children[index];
          break;
        } else if (sharpLen > current.data.length) {
          parentHistory.push(parent);
          parent = current;
          const index = parent.addChildren({length: sharpLen, content: lines[i]});
          current = parent.children[index];
          break;
        } else if (sharpLen === current.data.length) {
          const index = parent.addChildren({length: sharpLen, content: lines[i]});
          current = parent.children[index];
          break;
        } else {
          const mayParent = parentHistory.pop();
          if (mayParent instanceof Node) {
            current = parent;
            parent = mayParent;
          } else {
            throw Error('failed');
          }
        }
      }
    }
  }

  return tree;
};


export const transferToDom = (tree: Tree<ILine>): string => {
  let result = '';
  const shower = (root: Node<ILine>) => {
    result += '<div>';
    const content = root.data.content.replace(/^#+/, '');
    result += content;
    if (root.children.length > 0) {
      result += '<ol>';
    }
    for (const child of root.children) {
      result += '<li>';
      shower(child);
      result += '</li>';
    }
    if (root.children.length > 0) {
      result += '</ol>';
    }
    result += '</div>';
  };

  shower(tree.root);
  return result;
};

export const transferHeadersToDom = (lines: string[]): string => {
  const tree = genCatalogue(lines);
  return transferToDom(tree);
};

