use serde::{Deserialize, Serialize};
use std::fs;
use chrono::NaiveDate;

#[derive(Debug, Deserialize, Serialize)]
pub struct MateriaPrima {
    pub nome: String,
    pub quantidade_disponivel: f64,
    pub tempo_entrega: u32,
    pub custo: f64,
}

impl MateriaPrima {
    pub fn from_json(file_name: &str) -> Vec<MateriaPrima> {
        let data = fs::read_to_string(file_name).expect("Erro ao ler o arquivo de matérias-primas");
        serde_json::from_str(&data).expect("Erro ao desserializar as matérias-primas")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MateriaPrimaSimplificada {
    pub nome: String,
    pub custo: f64,
    pub tempo_entrega: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Produto {
    pub nome: String,
    pub materias_primas: Vec<(MateriaPrimaSimplificada, u32)>, // Matéria prima simplificada e quantidade necessária
    pub tempo_fabricacao: u32,
    pub capacidade_producao: u32,
}

impl Produto {
    pub fn from_json(file_name: &str) -> Vec<Produto> {
        let data = fs::read_to_string(file_name).expect("Erro ao ler o arquivo de produtos");
        serde_json::from_str(&data).expect("Erro ao desserializar os produtos")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pedido {
    pub produto: String,
    pub data_entrega: String,
}

impl Pedido {
    pub fn from_json(file_name: &str) -> Vec<Pedido> {
        let data = fs::read_to_string(file_name).expect("Erro ao ler o arquivo de pedidos");
        serde_json::from_str(&data).expect("Erro ao desserializar os pedidos")
    }
}

#[derive(Debug)]
pub struct PedidoCompra {
    pub nome: String,
    pub quantidade: f64,
    pub data_pedido: NaiveDate,
}

pub fn otimizar_producao(
    materias_primas: &[MateriaPrima],
    produtos: &[Produto],
    pedidos: &[Pedido],
) -> Vec<PedidoCompra> {
    let mut pedidos_compra = Vec::new();

    for pedido in pedidos {
        println!("Processando pedido: {:?}", pedido);
        if let Some(produto) = produtos.iter().find(|p| p.nome == pedido.produto) {
            println!("Produto encontrado: {:?}", produto);
            for (materia_simplificada, quantidade_necessaria) in &produto.materias_primas {
                println!("Processando matéria-prima: {:?}", materia_simplificada);
                if let Some(materia) = materias_primas.iter().find(|m| m.nome == materia_simplificada.nome) {
                    let quantidade_total = (*quantidade_necessaria as f64) * produto.capacidade_producao as f64;
                    println!("Quantidade necessária de {}: {}", materia.nome, quantidade_total);

                    // Validações
                    if quantidade_total > materia.quantidade_disponivel {
                        println!(
                            "Aviso: A quantidade necessária de {} excede a disponível ({})",
                            materia.nome, materia.quantidade_disponivel
                        );
                        continue; // Se a quantidade disponível for menor, ignoramos este item
                    }

                    // Calcular a data de pedido considerando o tempo de entrega
                    let data_entrega = NaiveDate::parse_from_str(&pedido.data_entrega, "%Y-%m-%d").unwrap();
                    let data_pedido = data_entrega - chrono::Duration::days(materia.tempo_entrega as i64);
                    println!("Data de entrega: {}, Data de pedido calculada: {}", data_entrega, data_pedido);

                    pedidos_compra.push(PedidoCompra {
                        nome: materia.nome.clone(),
                        quantidade: quantidade_total,
                        data_pedido,
                    });

                    println!(
                        "Pedido de compra adicionado: Matéria-prima: {}, Quantidade: {}, Data do pedido: {}",
                        materia.nome, quantidade_total, data_pedido
                    );
                } else {
                    println!("Matéria-prima {} não encontrada nas matérias-primas disponíveis.", materia_simplificada.nome);
                }
            }
        } else {
            println!("Produto {} não encontrado na lista de produtos.", pedido.produto);
        }
    }

    pedidos_compra
}