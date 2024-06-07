#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use parking_lot::{
        domain::{
            repository::parking_lot_repository::ParkingLotRepository,
            usecase::{enter_parking_lot::EnterParkingLot, get_parking_lot::GetParkingLot},
        },
        infra::repository::{
            parking_lot_repository_in_memory::ParkingLotRepositoryInMemory,
            parking_lot_repository_sqlite::ParkingLotRepositorySqlite,
        },
    };

    #[tokio::test]
    async fn enter_parking_lot() {
        let parking_lot_repository_in_memory = ParkingLotRepositoryInMemory::new();
        let repository: Arc<RwLock<dyn ParkingLotRepository>> =
            Arc::new(RwLock::new(parking_lot_repository_in_memory));
        let enter_parking_lot = EnterParkingLot::new(repository.clone());
        let get_parking_lot = GetParkingLot::new(repository);

        let parking_lot_before_enter = get_parking_lot.execute("shopping_mall".to_string()).await;
        assert_eq!(parking_lot_before_enter.occupied_spaces, 0);

        let date =
            NaiveDateTime::parse_from_str("2022-01-01T9:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let _ = enter_parking_lot
            .execute("shopping_mall".to_string(), "ABC1234".to_string(), date)
            .await;

        let parking_lot_after_enter = get_parking_lot.execute("shopping_mall".to_string()).await;
        assert_eq!(parking_lot_after_enter.occupied_spaces, 1);
    }

    #[tokio::test]
    async fn closed_parking_lot() {
        let parking_lot_repository_in_memory = ParkingLotRepositoryInMemory::new();
        let repository: Arc<RwLock<dyn ParkingLotRepository>> =
            Arc::new(RwLock::new(parking_lot_repository_in_memory));
        let enter_parking_lot = EnterParkingLot::new(repository.clone());
        let get_parking_lot = GetParkingLot::new(repository);

        let parking_lot_before_enter = get_parking_lot.execute("shopping_mall".to_string()).await;
        assert_eq!(parking_lot_before_enter.occupied_spaces, 0);

        let date =
            NaiveDateTime::parse_from_str("2022-01-01T23:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let _ = enter_parking_lot
            .execute("shopping_mall".to_string(), "ABC1234".to_string(), date)
            .await;

        let parking_lot_after_enter = get_parking_lot.execute("shopping_mall".to_string()).await;
        assert_eq!(parking_lot_after_enter.occupied_spaces, 1);
    }

    #[tokio::test]
    async fn full_parking_lot() {
        let parking_lot_repository_in_memory = ParkingLotRepositoryInMemory::new();
        let repository: Arc<RwLock<dyn ParkingLotRepository>> =
            Arc::new(RwLock::new(parking_lot_repository_in_memory));
        let enter_parking_lot = EnterParkingLot::new(repository.clone());
        let get_parking_lot = GetParkingLot::new(repository);

        let parking_lot_before_enter = get_parking_lot.execute("shopping_mall".to_string()).await;
        assert_eq!(parking_lot_before_enter.occupied_spaces, 0);

        let date =
            NaiveDateTime::parse_from_str("2022-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        for i in 0..10 {
            let plate_number = format!("ABC123{}", i);
            let _ = enter_parking_lot
                .execute("shopping_mall".to_string(), plate_number, date)
                .await;
        }

        let parking_lot_after_enter = get_parking_lot.execute("shopping_mall".to_string()).await;
        assert_eq!(parking_lot_after_enter.occupied_spaces, 1);
    }

    #[tokio::test]
    async fn get_parking_lot() {
        // Criação da tabela parking_lots antes de executar o teste
        let connection = ParkingLotRepositorySqlite::new("./parking_lot.sqlite")
            .establish_connection()
            .expect("Failed to establish connection");
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS parking_lots (
                code TEXT PRIMARY KEY,
                capacity INTEGER,
                open_hour INTEGER,
                close_hour INTEGER,
                occupied_spaces INTEGER
            )",
                [],
            )
            .expect("Failed to create table");

        // Adição do estacionamento
        let parking_lot_repository_sqlite = ParkingLotRepositorySqlite::new("./parking_lot.sqlite");
        parking_lot_repository_sqlite.add_parking_lot();

        // Execução do teste
        let repository: Arc<RwLock<dyn ParkingLotRepository>> =
            Arc::new(RwLock::new(parking_lot_repository_sqlite));
        let get_parking_lot = GetParkingLot::new(repository);

        assert_eq!(
            get_parking_lot
                .execute("shopping_mall".to_string())
                .await
                .occupied_spaces,
            0
        );
    }
}
