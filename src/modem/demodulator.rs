pub fn detect_bfsk(freq: f32, carrier: f32, deviation: f32, threshold: f32) -> i8 {
    let in_low_range =
        move |res: f32| -> bool { res >= (carrier - threshold) && res <= carrier + threshold };

    let in_high_range = move |res: f32| -> bool {
        res >= (carrier + deviation - threshold) && res <= carrier + deviation + threshold
    };

    if in_low_range(freq) {
        0
    } else if in_high_range(freq) {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod recorder_test {

    use crate::{
        bytes::encode_u8,
        config::ModemConfig,
        modem::modulator::psk,
        synthesizer::{read_wave, save_wave},
    };

    const SAMPLERATE: u32 = 100;
    const BAUDRATE: u16 = 10u16;

    #[test]
    fn test_generate_psk() {
        let send_data = "a";
        let mut config = ModemConfig::default();
        config.set_samplerate(SAMPLERATE);
        config.set_baudrate(BAUDRATE);
        // let mut trans = Transmitter::new(config.clone());
        let send_data = encode_u8(send_data);
        let data = psk(send_data, SAMPLERATE, config.latency() as usize);
        save_wave("psktest.wav", data.clone(), config.samplerate as u32);
    }

    // #[test]
    // fn test_recorder() {
    //     let latency = 1.0 / BAUDRATE as f32 * SAMPLERATE as f32;
    //     println!("latency: {}", latency);
    //     let mut wave = read_wave("psktest.wav");
    //     let mut cur;
    //     let mut prev;
    //     let mut x: Vec<f32> = vec![];
    //     let mut amps = vec![];
    //     let mut angles = vec![];
    //     loop {
    //         if wave.len() <= 0 {
    //             break;
    //         }
    //         cur = wave[latency as usize..].to_vec();
    //         prev = wave[0..latency as usize - 1].to_vec();

    //         for p in prev.iter_mut() {
    //             if p == &0.0 {
    //                 *p = *p + 1e-9;
    //             }
    //         }
    //         for (c, p) in cur.iter().zip(prev) {
    //             let o = Complex32::new(c / p, c / p);
    //             x.push(o.im.atan2(o.re));
    //         }
    //         for w in wave.clone() {
    //             let comp = Complex32::new(w, w);
    //             amps.push(comp.re.hypot(comp.im));
    //             angles.push(comp.im.atan2(comp.re));
    //         }
    //         wave = wave[latency as usize..].to_vec();
    //     }

    //     let first_idx = x.clone().iter().position(|&r| r.abs() > PI / 2.0).unwrap();
    //     println!("first index: {}", first_idx);
    //     let image_width = 2160;
    //     let image_height = 720;
    //     // 描画先を指定。画像出力する場合はBitMapBackend
    //     let root = BitMapBackend::new("plot.png", (image_width, image_height)).into_drawing_area();

    //     // 背景を白にする
    //     root.fill(&WHITE).unwrap();

    //     let caption = "Sample Plot";
    //     let font = ("sans-serif", 20);
    //     let y_length: i32 = 1000;

    //     let mut chart = ChartBuilder::on(&root)
    //         .caption(caption, font.into_font()) // キャプションのフォントやサイズ
    //         .margin(10) // 上下左右全ての余白
    //         .x_label_area_size(16) // x軸ラベル部分の余白
    //         .y_label_area_size(42) // y軸ラベル部分の余白
    //         .build_cartesian_2d(
    //             // x軸とy軸の数値の範囲を指定する
    //             0..y_length,   // x軸の範囲
    //             -2.5f32..1f32, // y軸の範囲
    //         )
    //         .unwrap();
    //     chart.configure_mesh().draw().unwrap();
    //     let mut y = vec![];
    //     for i in 0..y_length {
    //         y.push(i as i32);
    //     }

    //     // 折れ線グラフの定義＆描画
    //     let amps_series = LineSeries::new(
    //         y.iter()
    //             .zip(amps[first_idx..y_length as usize + first_idx].iter())
    //             .map(|(x, y)| (*x, *y)),
    //         RGBColor(255, 0, 0),
    //     );

    //     let angles_series = LineSeries::new(
    //         y.iter()
    //             .zip(angles[first_idx..y_length as usize + first_idx].iter())
    //             .map(|(x, y)| (*x, *y)),
    //         RGBColor(0, 255, 0),
    //     );

    //     let angles_point_series = y
    //         .iter()
    //         .zip(angles[first_idx..y_length as usize + first_idx].iter())
    //         .map(|(x, y)| Circle::new((*x, *y), 4, &RED));

    //     chart.draw_series(amps_series).unwrap();
    //     chart.draw_series(angles_point_series).unwrap();
    //     chart.draw_series(angles_series).unwrap();
    //     return;
    // }
}
