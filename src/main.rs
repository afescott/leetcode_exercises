impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut highest_number = height[0];
        /*         let mut starting_number = 0; */
        let mut end_container_number = 0;
        let mut wave_tracker = Vec::new();
        let mut tracker_id = 0;
        for number in height {
            if number > highest_number {
                println!("hit");
                highest_number = number;
                wave_tracker = Vec::new();
                tracker_id = 0;
            } else if end_container_number < number {
                end_container_number = number;
            }
            wave_tracker.push(number);
            tracker_id += 1;
        }

        println!("tracker: {:?}", tracker_id);
        let mut tracking_val = 0;
        wave_tracker.iter().map(|m| tracking_val += *m);

        (tracker_id - 1) * (tracking_id - 1) as i32
    }
}

fn main() {}
