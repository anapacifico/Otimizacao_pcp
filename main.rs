mod pcp;
use crate::pcp::{MateriaPrima, Produto, Pedido, otimizar_producao};

fn main() {
    println!("Sistema de Planejamento e Controle de Produção (PCP)");

    let materias_primas: Vec<MateriaPrima> = MateriaPrima::from_json("materiais_primas.json");
    println!("Matérias primas: {:?}", materias_primas);

    let produtos: Vec<Produto> = Produto::from_json("produtos.json");
    println!("Produtos: {:?}", produtos);

    let pedidos: Vec<Pedido> = Pedido::from_json("pedidos.json");
    println!("Pedidos: {:?}", pedidos);

    let pedidos_compra = otimizar_producao(&materias_primas, &produtos, &pedidos);
    println!("Pedidos de Compra Otimizados:");
    for pedido in pedidos_compra {
        println!(
            "Matéria-prima: {}, Quantidade: {}, Data do pedido: {}",
            pedido.nome, pedido.quantidade, pedido.data_pedido
        );
    }
}