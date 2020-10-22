use std::cmp::{max, Ordering};
use std::collections::BinaryHeap;

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_u32() -> u32 {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv[0].parse::<u32>().unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as an `usize`, for a shorter implementation.
#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

fn warshall_floyd_max(dist: &mut Vec<Vec<Edge>>) -> usize {
    // dist[node] = current shortest distance from `start` to `node`
    // let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MIN).collect();

    for k in 0..adj_list.len() {
        for i in 0..adj_list.len() {
            for j in 0..adj_list.len() {
                dist[i][j].cost = max(dist[i][j].cost, dist[i][j].cost + dist[k][j].cost)
            }
        }
    }
}

// n を与えられると graph を返す関数を用意したい
fn gen_bit_graph(n: u32) -> Vec<Vec<Edge>> {
    // 10進数から進める数を算出する
    fn number_to_dice(number: u32) -> u32 {
        format!("{:b}", number)
            .to_string()
            .chars()
            // .filter(|&x| x == '1')
            .map(|x| x.to_digit(10).unwrap())
            .sum::<u32>()
    }

    // ラベル番号が与えられると 有向エッジを返す。なおすべてのコストは1固定
    fn gen_node(number: u32, _n: u32) -> Vec<Edge> {
        return if number == _n {
            vec![]
        } else if number == 0 {
            vec![Edge { node: 1, cost: 1 }]
        } else if number == 1 {
            vec![Edge { node: 2, cost: 1 }]
        } else {
            // ゴールを超えたら戻るやつ。このあたり微妙
            let dice = number_to_dice(number);
            let negative: u32 = number - dice;
            let positive: u32 = number + dice;
            return if (number + dice) <= _n {
                vec![
                    Edge {
                        node: positive as usize,
                        cost: 1,
                    },
                    Edge {
                        node: negative as usize,
                        cost: 1,
                    },
                ]
            } else {
                vec![Edge {
                    node: negative as usize,
                    cost: 1,
                }]
            };
        };
    }

    (0..=n).map(|x| gen_node(x, n)).collect()
}

fn main() {
    let n: u32 = getline_as_u32();

    // ワーシャルフロイドで全点最大経路を求め、虱潰しに全点のうち最大値を探すのはどうでしょうか

    // This is the directed graph we're going to use.
    // The node numbers correspond to the different states,
    // and the edge weights symbolize the cost of moving
    // from one node to another.
    // Note that the edges are one-way.
    //
    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           10      |               |
    //                   +---------------+
    //
    // The graph is represented as an adjacency list where each index,
    // corresponding to a node value, has a list of outgoing edges.
    // Chosen for its efficiency.
    // let graph = vec![
    //     // Node 0
    //     vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
    //     // Node 1
    //     vec![Edge { node: 3, cost: 2 }],
    //     // Node 2
    //     vec![
    //         Edge { node: 1, cost: 1 },
    //         Edge { node: 3, cost: 3 },
    //         Edge { node: 4, cost: 1 },
    //     ],
    //     // Node 3
    //     vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
    //     // Node 4
    //     vec![],
    // ];

    // assert_eq!(shortest_path(&graph, 0, 1), Some(1));
    // assert_eq!(shortest_path(&graph, 0, 3), Some(3));
    // assert_eq!(shortest_path(&graph, 3, 0), Some(7));
    // assert_eq!(shortest_path(&graph, 0, 4), Some(5));
    // assert_eq!(shortest_path(&graph, 4, 0), None);

    // n が与えられたときの移動コスト1固定、有向グラフを生成する
    let graph = gen_bit_graph(n);

    // ダイクストラ適用。最小移動コスト = 最小移動ステップ数
    match shortest_path(&graph, 0, n as usize) {
        Some(x) => println!("{}", x),
        None => println!("-1"),
    }
}
