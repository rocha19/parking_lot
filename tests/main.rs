#[cfg(test)]
mod tests {
    use parking_lot::{
        domain::usecase::enter_parking_lot::EnterParkingLot,
        infra::repository::parking_lot_repository_in_momory::ParkingLotRepositoryInMemory,
    };

    #[tokio::test]
    async fn enter_parking_lot() {
        let parkign_lot_repository_in_memory = ParkingLotRepositoryInMemory {};
        let parking_lot = EnterParkingLot::new(Box::new(parkign_lot_repository_in_memory))
            .await
            .execute("shopping_mall".to_string())
            .await;
        assert_eq!(parking_lot.code, "shopping_mall");
    }
}
