// Comtrole de módulos do banco de dados

// Lista de módulos do banco de dados
mod migrate;

// re-exporta as funções dos módulos
pub use migrate::create_tables;