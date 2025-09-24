mod dag;
mod pouw;
mod net;

fn main() {
    println!("TEQLA Core Node — DAG + PoUW (skeleton)");
    // TODO:
    // - inicializar p2p (libp2p/gossipsub) [ver net.rs]
    // - carregar estado do DAG [ver dag.rs]
    // - laço principal: receber tx, verificar sig PQC (futuro), verificar PoUW, anexar ao DAG, propagar
}