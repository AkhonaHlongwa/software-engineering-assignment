pub struct Reservation {
    pub reservation_id: String,
    pub reservation_date: String,
    pub status: String,
}

impl Reservation {
    pub fn new(
        reservation_id: String,
        reservation_date: String,
        status: String,
    ) -> Self {
        Self {
            reservation_id,
            reservation_date,
            status,
        }
    }

    pub fn confirm_reservation(&self) {
        println!("Reservation confirmed");
    }

    pub fn cancel_reservation(&self) {
        println!("Reservation cancelled");
    }
}
