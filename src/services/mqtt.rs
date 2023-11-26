use mosquitto_client::{Error, Mosquitto, MosqMessage};

#[derive(Clone)]
pub struct MQTT {
    pub mosq: Mosquitto,
}

impl MQTT {
    pub fn new() -> Self {
        let m = Mosquitto::new("api");

        match m.connect("localhost", 1883) {
            Ok(_) => (),
            Err(err) => panic!(
                "Could not connect to Mosquitto. Error code:{} - {}",
                err.error(),
                err.to_string()
            ),
        };
        MQTT { mosq: m }
    }

    pub fn publish(&self, topic: &str, msg: &[u8]) -> Result<i32, Error> {
        self.mosq.publish(topic, msg, 2, true)
    }

    pub fn get_messages(&self, topic: &str) -> Vec<MosqMessage> {
        let matcher = self.mosq.subscribe(topic, 2).unwrap();
        matcher.receive_many(200).unwrap()
    }
}
