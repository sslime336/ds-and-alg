mod adjacency_list;
mod binary_sort_tree;
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
    use crate::binary_sort_tree::BinarySortTree;

    #[test]
    fn solution() {
        let testdata = vec![37, 5, 8, 15, 9, 45, 46, 21, 18, 22];
        let mut tree = BinarySortTree::new();
        testdata.into_iter().for_each(|ele| {
            tree.insert(ele).unwrap();
        });
        let tree_copycat = tree.clone();

        println!("{}", tree.inorder_traversal());
        let target = 21;
        println!("remove {target}");
        tree.remove(&target).unwrap();
        println!("{}", tree.inorder_traversal());
        let target = 9;
        println!("remove {target}");
        tree.remove(&target).unwrap();
        println!("{}", tree.inorder_traversal());

        println!("adj_list: \n{:?}", tree_copycat.adjacency_list().unwrap());
        println!("topology_sort: {}", tree_copycat.topology_sort());
    }
}

/// # 实验三
///
///     1.创建数据库；
///     2.再各表中输入测试数据（初始化测试数据）；
///     3.查询学分大于2且有先修课程的课程，按课程号递增顺序列出课程号、课程名称、学分和先修课程号；
///     4.查询与运动控制相关的课程（课程名称中包含“运动控制”字样），按课程名称、课程号顺序，列出课程名称、课程号、学分和先修课程号；
///     5.统计与软件相关的课程的数量和总学分（课程名称中包含“软件”或“程序”字样）；
///     6.查询某学生（学号“061806”）所有成绩良好（成绩80及以上）的修读情况，按成绩递减顺序列出课程号、课程名称、成绩和学分；
///     7.统计所有课程的自动化学院学生成绩情况（没有成绩的除外），按平均成绩顺序，列出课程号、课程名称、修读人数、平均成绩、最高成绩、最低成绩；
///     8.查询所有自动化学院学生都修读的课程，按课程号顺序，列出课程号、课程名称；
///     9.添加 “人工智能”的课程信息：课程号为“06C066”、学分3、先修课程为空。
///
#[cfg(test)]
mod exp3 {
    #[test]
    fn solution() {}
}
