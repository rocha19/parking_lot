CREATE TABLE ParkingLot (
    code TEXT PRIMARY KEY,
    capacity INTEGER NOT NULL,
    open_hour INTEGER NOT NULL,
    close_hour INTEGER NOT NULL,
    occupied_spaces INTEGER NOT NULL
);

CREATE TABLE ParkedCar (
    code TEXT NOT NULL,
    plate TEXT NOT NULL,
    date TEXT NOT NULL,
    FOREIGN KEY (code) REFERENCES ParkingLot(code)
);

CREATE INDEX idx_parkedcar_code ON ParkedCar(code);
CREATE INDEX idx_parkedcar_plate ON ParkedCar(plate);
CREATE INDEX idx_parkedcar_date ON ParkedCar(date);
