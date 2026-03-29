use std::{thread, time::Duration};

// Simulate an AI model for edge inference
struct EdgeModel {
    model_name: String,
    version: String,
}

impl EdgeModel {
    fn new(name: &str, ver: &str) -> Self {
        println!("EdgeModel initialized: {} v{}", name, ver);
        EdgeModel { model_name: name.to_string(), version: ver.to_string() }
    }

    fn infer(&self, input_data: &str) -> String {
        println!("Performing inference with {} on: {}", self.model_name, input_data);
        // Simulate inference time
        thread::sleep(Duration::from_millis(30));
        format!("{{ \"prediction\": \"class_A\", \"confidence\": 0.95, \"model\": \"{}\" }}", self.model_name)
    }
}

// Simulate an edge device
struct EdgeDevice {
    device_id: String,
    model: EdgeModel,
}

impl EdgeDevice {
    fn new(id: &str, model: EdgeModel) -> Self {
        println!("EdgeDevice {} online.", id);
        EdgeDevice { device_id: id.to_string(), model }
    }

    fn process_sensor_data(&self, data: &str) {
        println!("Device {} processing sensor data: {}", self.device_id, data);
        let result = self.model.infer(data);
        println!("Device {} inference result: {}", self.device_id, result);
    }
}

fn main() {
    println!("Starting AI Edge Inference Simulation");

    let model = EdgeModel::new("TinyYOLOv3", "1.0.1");
    let device1 = EdgeDevice::new("sensor_001", model);

    let model2 = EdgeModel::new("MobileNetV2", "2.0.0");
    let device2 = EdgeDevice::new("camera_feed_002", model2);

    // Simulate data streams
    device1.process_sensor_data("temperature: 25C, humidity: 60%");
    device2.process_sensor_data("image_frame_123.jpg");

    thread::sleep(Duration::from_secs(1));

    device1.process_sensor_data("vibration: high, pressure: normal");
    device2.process_sensor_data("image_frame_124.jpg");

    println!("AI Edge Inference Simulation Finished");
}
