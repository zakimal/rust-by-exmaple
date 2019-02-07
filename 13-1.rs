struct A; // 具象型構造体A
struct S(A); // 具象型構造体S
struct SGen<T>(T); // ジェネリック型構造体SGen

 //　具象型Sを引数に取る関数
fn reg_fn(_s: S) {}

// ジェネリック関数ではない
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}

// ジェネリック関数:任意の型Tに対して用いることができる
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32((SGen(6)));
    generic::<char>(SGen('a'));
    generic(SGen('c'));
}