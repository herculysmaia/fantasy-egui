// Módulo para inciar banco de dados

// Componente necessários
use std::fs;
use sqlx::SqlitePool;
use tokio::runtime::Runtime;

// Criação das tabelas no banco de dados
pub fn create_tables() -> Result<(), i32> {

    // Verifica se o diretório do banco de dados existe, caso não exista, cria o diretório
    if let Some(mut path) = dirs::document_dir() {
        
        // Subdiretórios até o banco de dados
        path.push("cartola");
        path.push("2025");

        // Verifica se o diretório existe, se não existir, cria o diretório
        if fs::create_dir_all(&path).is_ok() {

            // adiciona nome do banco ao caminho
            path.push("cartolaegui.db");

            // chama função para criar o banco de dados
            let dir = format!("sqlite://{}", path.to_string_lossy()).replace("\\", "/");
            std::fs::File::create(&path).map_err(|_| 102)?;
            
            // Cria um runtime do tokio para executar as operações assíncronas
            let rt = Runtime::new().map_err(|_| 103)?;
            let pool = rt.block_on(SqlitePool::connect(&dir)).map_err(|_| 104)?;
            
            // Cria tabelas no banco de dados
            rt.block_on(async { criar_time(pool.clone()).await })?;
            rt.block_on(async { criar_movimentacoes(pool.clone()).await })?;
            rt.block_on(async { criar_pontuacoes(pool.clone()).await })?;
            rt.block_on(async { criar_participacoes(pool.clone()).await })?;
            
        } else {
            
            // Retorna erro se não conseguir criar o diretório
            return Err(101);

        };
    }

    // retorna Ok
    Ok(())

}

// Função auxiliar para executar uma query no banco de dados
async fn chamar_query(pool: SqlitePool, query: &str) -> Result<(), i32> {

    // Executa a query
    sqlx::query(query)
        .execute(&pool)
        .await
        .map_err(|_| 105)?;

    // Retorna Ok
    Ok(())

}

// Funções para criar as tabela time no banco de dados
async fn criar_time(pool: SqlitePool) -> Result<(), i32> {

    // Query de criação
    let query = "
        CREATE TABLE IF NOT EXISTS times (
            id            INTEGER PRIMARY KEY,
            nome_do_time  TEXT NOT NULL,
            nome_do_dono  TEXT NOT NULL,
            escudo        BLOB,
            perfil        BLOB
        );";

    // Executa a query
    chamar_query(pool, query).await?;

    // Retorna Ok
    Ok(())

}

// Funções para criar as tabela movimentacao no banco de dados
async fn criar_movimentacoes(pool: SqlitePool) -> Result<(), i32> {

    // Query de criação
    let query = "
        CREATE TABLE IF NOT EXISTS movimentacoes (
            id         INTEGER,
            time_id    INTEGER NOT NULL,
            data_dia   INTEGER NOT NULL,
            data_mes   INTEGER NOT NULL,
            valor      INTEGER NOT NULL,
            tipo       INTEGER NOT NULL,
            rodada     INTEGER,
            PRIMARY    KEY(id AUTOINCREMENT),
            FOREIGN    KEY(time_id) REFERENCES times(id)
        );"
    ;

    // Executa a query
    chamar_query(pool, query).await?;

    // Retorna Ok
    Ok(())

}

// Funções para criar as tabela pontuacao no banco de dados
async fn criar_pontuacoes(pool: SqlitePool) -> Result<(), i32> {

    // Query de criação
    let query = "
        CREATE TABLE IF NOT EXISTS pontuacoes (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            time_id         INTEGER NOT NULL,
            pontos          INTEGER NOT NULL,
            rodada          INTEGER NOT NULL,
            classificacao   INTEGER NOT NULL,
            FOREIGN KEY(time_id) REFERENCES times(id)
        );"
    ;

    // Executa a query
    chamar_query(pool, query).await?;

    // Retorna Ok
    Ok(())

}

// Funções para criar as tabela participacao no banco de dados
async fn criar_participacoes(pool: SqlitePool) -> Result<(), i32> {

    // Query de criação
    let query = "
        CREATE TABLE IF NOT EXISTS participacoes (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            time_id         INTEGER NOT NULL,
            rodada          INTEGER NOT NULL,
            ano             INTEGER NOT NULL,
            FOREIGN KEY(time_id) REFERENCES times(id)
        );"
    ;

    // Executa a query
    chamar_query(pool, query).await?;

    // Retorna Ok
    Ok(())

}