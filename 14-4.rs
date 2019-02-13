// ライフタイムは、コンパイラが、プログラムに登場するすべての借用に問題がないかを確認するために用いられる仕組み。
// 変数のライフタイムは「作成されてから廃棄されるまで」。スコープと同時に語られることが多いが、スコープとライフタイムは
// 同じものではない。

// 以下では、変数の作成から破棄までのライフタイムを線で示しています。
// `i`は最長のライフタイムを持ち、そのスコープは`borrow1`および`borrow2`
// のスコープを完全に包含します。`borrow1`と`borrow2`の存続期間は一切重なりません。

fn main() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }

}