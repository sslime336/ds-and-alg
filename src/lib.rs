mod adjacency_list;
mod binary_sort_tree;
mod graph;
mod linked_list;
mod route;

/// 生成 `data_count`(10-20) 个范围在 `max_data`(50-100)之间不重复的整数
fn random_i32(data_count: usize, max_data: usize) -> Vec<usize> {
    let mut res = Vec::with_capacity(data_count);
    let mut last_num = data_count;
    while last_num > 0 {
        let rd: f32 = rand::random();
        let next_num = (rd * max_data as f32) as usize;
        if res.contains(&next_num) {
            continue;
        }
        res.push(next_num);
        last_num -= 1;
    }
    res
}

#[test]
fn rp() {
    println!("{:?}", random_i32(10, 50));
}

/// # 实验一
///
/// 设计程序，定义二叉链表，存储二叉排序树，声明并定义相应的函数，实现链式二叉排序树的下列操作：
///
///     1.输入数据个数DataCount（要求在10和20之间）和数据最大值MaxData（在50和100之间）；
///     2.在0和MaxData之间，随机产生DataCount个不重复的整数，按产生先后顺序形成一个数据序列，并输出该序列；
///     3.利用上述数据序列，创建一个二叉排序树；
///     4.设计函数，统计该二叉树的高度；
///     5.设计函数，输出该二叉树的叶子节点；
///     6.设计中序遍历函数，遍历该二叉排序树，输出遍历序列，验证创建的二叉排序树是否正确。
///
#[cfg(test)]
mod exp1 {
    use crate::binary_sort_tree::BinarySortTree;

    #[test]
    fn solution() {
        // let testdata = random_i32(10, 50);
        let testdata = vec![37, 5, 8, 15, 9, 45, 46, 21, 18, 22];
        let mut tree = BinarySortTree::new();
        testdata.into_iter().for_each(|ele| {
            tree.insert(ele).unwrap();
        });

        assert_eq!(tree.height(), 6, "tree height: {}", tree.height());
        assert_eq!(
            tree.leaves(),
            Some(vec![9, 18, 22, 46]),
            "tree leaves: {:?}",
            tree.leaves()
        );
        assert_eq!(
            tree.inorder_traversal().as_str(),
            "5 -> 8 -> 9 -> 15 -> 18 -> 21 -> 22 -> 37 -> 45 -> 46",
            "tree inorder traversal: {}",
            tree.inorder_traversal()
        );
    }
}

/// # 实验二
///
///     1.创建一棵二叉排序树（以下称为源二叉树）；
///     2.从源二叉树拷贝一个二叉树（以下称为二叉树副本）
///     3.通过键盘输入数据，选择查找的目标二叉树（源二叉树和二叉树副本），在目标二叉树中查找是否存在该数据，若存在，则输出提示以及该数据节点的地址，若不存在，则输出提示；
///     4.删除数据操作：通过键盘输入数据；如果二叉树副本中存在该数据，则从二叉树副本中删除该数据节点，并输出中序遍历序列；如果二叉树副本中不存在该数据，输出提示；
///     5.将源二叉树视为一个有向图数据结构，编写函数实现该图的邻接表存储（注意程序需确保该操作不会被重复操作）；
///     6.在步骤5的邻接表的基础上，实现该图的拓扑排序，并输出排序序列。
///
#[cfg(test)]
mod exp2 {
    use crate::graph::Graph;

    macro_rules! graph {
        ($($head:expr=>$next:expr,)*) => {
            {
                let mut graph: Graph<usize> = Graph::new();
                $(
                    graph.push(($head, $next));
                )*
                graph
            }
        };
    }

    #[test]
    fn solution() {
        let dag = graph! {
            1 => 2,
            1 => 3,
            2 => 4,
            2 => 5,
            4 => 5,
            4 => 6,
            6 => 3,
        };
        let adj_list = dag.adjacency_list().unwrap();
        println!("adjacency list:\n {:?}", adj_list);

        let gp = graph! {
            1 => 2,
            1 => 3,
            3 => 1,
            5 => 3,
            5 => 4,
            4 => 2,
            4 => 6,
        };
        let adj_list = gp.adjacency_list().expect("not a DAG");
        println!("adjacency list:\n {:?}", adj_list);
    }
}
