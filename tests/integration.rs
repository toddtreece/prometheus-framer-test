use core::*;
use grafana_plugin_sdk::arrow2::array::*;
use utils::generate_range_vectors;

#[test]
fn names() {
    let (time_range, rv) = generate_range_vectors(2, 1);
    let frames = Matrix::from(&rv).to_frames(&time_range);
    assert_eq!("every_0", frames[0].name);
    assert_eq!("every_1", frames[1].name);
}

#[test]
fn single_series() {
    let (time_range, rv) = generate_range_vectors(1, 10);
    let frames = Matrix::from(&rv).to_frames(&time_range);

    let expected_timestamps = (0..=10 * ONE_SECOND_NS)
        .step_by(ONE_SECOND_NS as usize)
        .map(Some)
        .collect::<Vec<Option<i64>>>();

    let timestamps: Vec<Option<i64>> = frames[0].fields()[0]
        .values()
        .as_any()
        .downcast_ref::<PrimitiveArray<i64>>()
        .unwrap()
        .into_iter()
        .map(|v| Some(*v.unwrap()))
        .collect();

    assert_eq!(expected_timestamps, timestamps);

    let expected_values = (0..=10)
        .step_by(1)
        .map(|v| Some(v as f64))
        .collect::<Vec<Option<f64>>>();

    let values: Vec<Option<f64>> = frames[0].fields()[1]
        .values()
        .as_any()
        .downcast_ref::<PrimitiveArray<f64>>()
        .unwrap()
        .into_iter()
        .map(|v| Some(*v.unwrap()))
        .collect();

    assert_eq!(expected_values, values);
}

#[test]
fn multiple_series() {
    let (time_range, rv) = generate_range_vectors(2, 10);
    let frames = Matrix::from(&rv).to_frames(&time_range);

    let expected_values = (0..=10)
        .step_by(1)
        .map(|v| {
            if v % 2 != 0 {
                return None;
            }
            Some(((v / 2) as usize) as f64)
        })
        .collect::<Vec<Option<f64>>>();

    let values: Vec<Option<f64>> = frames[1].fields()[1]
        .values()
        .as_any()
        .downcast_ref::<PrimitiveArray<f64>>()
        .unwrap()
        .into_iter()
        .map(|a| a.copied())
        .collect();

    assert_eq!(expected_values, values);
}
