fn main() {
        // 変数を宣言だけするのも可能だが、最後まで初期化しないと怒られる。
            let a_binding;
                {
                        let x = 2;
                                a_binding = x*x
                                    }
                                        println!("a_binding: {}", a_binding);
                                            let another_binding;
                                                // println!("another_binding: {}",
                                                another_binding);
                                                    another_binding = 1;
                                                        println!("another_binding: {}",
                                                        another_binding);
                                                        }
                }
}
