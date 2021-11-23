use ndarray::Array2;
use num::Complex;

use plotters::prelude::*;

pub fn plot_probabilities_register1(partial_register: &Array2::<Complex<f64>>, description: String) {
    let mut probabilities = Vec::new();
    let mut max_pct = 0;

    // calculate all probabilities
    for i in 0..partial_register.len() {
        let p = partial_register[[i as usize, 0]].norm() * partial_register[[i as usize, 0]].norm();
        let pct = (p * 100.0).round() as i32;
        probabilities.push(pct);
        if pct > max_pct {
            max_pct = pct;
        }
    }

    // https://plotters-rs.github.io/book/basic/basic_data_plotting.html
    let filename = format!("images/{}.png", description);
    // deref and ref to go from String to &str
    // https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
    let root_area = BitMapBackend::new(&*filename, (600, 400))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("register 1", ("sans-serif", 40))
        .build_cartesian_2d((0..probabilities.len()).into_segmented(), 0..max_pct)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = probabilities;

    ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x + 1);
        let mut bar = Rectangle::new([(x0, 0), (x1, *y)], BLUE.filled());
        bar.set_margin(0, 0, 1, 1);
        bar
    }))
        .unwrap();

    println!("Done");

}