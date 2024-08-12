use bevy::prelude::*;
use bevy_plotting::{draw_gizmo_line_plot_2d_clamped, draw_gizmo_line_plot_2d_raw, points_min_max};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .add_systems(Update, draw)
        .run();
}

#[derive(Resource)]
struct Lines {
    wave: Vec<Vec2>,
    wave_y_scale_ui: [Entity; 2],
    spikey: Vec<Vec2>,
    spikey_y_scale_ui: [Entity; 2],
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());

    macro_rules! text {
        ($text: expr, $x: expr, $y: expr) => {
            cmd.spawn(Text2dBundle {
                text: Text::from_section(
                    $text,
                    TextStyle {
                        color: Color::WHITE.with_alpha(0.8),
                        ..default()
                    },
                ),
                transform: Transform::from_xyz($x, $y, 0.),
                ..default()
            })
        };
    }

    text!("Derived x & y", -400., 0.);
    text!("Derived x\nSet y", 0., 0.);
    text!("Derived x\nSet y\nClamped & filtered", 400., 0.);

    let wave_y_min = text!("0.0", -400. - 100. - 30., 100. + 8.).id();
    let wave_y_max = text!("0.0", -400. - 100. - 30., 300. - 8.).id();

    text!("-1.5", 0. - 100. - 30., 100. + 8.);
    text!("1.5", 0. - 100. - 30., 300. - 8.);

    text!("-0.5", 400. - 100. - 30., 100. + 8.);
    text!("1.5", 400. - 100. - 30., 300. - 8.);

    let spikey_y_min = text!("0.0", -400. - 100. - 30., -300. + 8.).id();
    let spikey_y_max = text!("0.0", -400. - 100. - 30., -100. - 8.).id();

    text!("-4.0", 0. - 100. - 30., -300. + 8.);
    text!("4.0", 0. - 100. - 30., -100. - 8.);

    text!("0.0", 400. - 100. - 30., -300. + 8.);
    text!("4.0", 400. - 100. - 30., -100. - 8.);

    cmd.insert_resource(Lines {
        wave: Vec::new(),
        wave_y_scale_ui: [wave_y_min, wave_y_max],
        spikey: Vec::new(),
        spikey_y_scale_ui: [spikey_y_min, spikey_y_max],
    });
}

fn update(mut lines: ResMut<Lines>, time: Res<Time>) {
    const LINE_MAX_LEN: usize = 200;

    lines.wave.push(Vec2::new(
        time.elapsed_seconds(),
        (time.elapsed_seconds() * 2.).sin(),
    ));
    if lines.wave.len() > LINE_MAX_LEN {
        lines.wave.remove(0);
    }

    lines.spikey.push(Vec2::new(
        time.elapsed_seconds(),
        rand::Rng::sample(&mut rand::thread_rng(), rand_distr::StandardNormal),
    ));
    if lines.spikey.len() > LINE_MAX_LEN {
        lines.spikey.remove(0);
    }
}

fn draw(lines: Res<Lines>, mut gizmos: Gizmos, mut text: Query<&mut Text>) {
    // Wave derived bounds
    {
        let point_count = 50;
        let last_n_index = lines.wave.len().saturating_sub(point_count);
        let (min_xy, max_xy) = points_min_max(lines.wave.iter().skip(last_n_index))
            .unwrap_or((Vec2::splat(-10.), Vec2::splat(10.)));

        let bounds = Rect::from_center_size(Vec2::new(-400., 200.), Vec2::new(200., 200.));

        let [mut min_text, mut max_text] = text.many_mut(lines.wave_y_scale_ui);
        min_text.sections[0].value = format!("{:.1}", min_xy.y);
        max_text.sections[0].value = format!("{:.1}", max_xy.y);

        draw_bounds(&mut gizmos, bounds);

        draw_gizmo_line_plot_2d_raw(
            lines.wave.iter().skip(last_n_index),
            &mut gizmos,
            bounds,
            min_xy,
            max_xy,
            Srgba::GREEN.into(),
        );
    }

    // Wave set y
    {
        let (mut min_xy, mut max_xy) =
            points_min_max(lines.wave.iter()).unwrap_or((Vec2::splat(-10.), Vec2::splat(10.)));

        min_xy.y = -1.5;
        max_xy.y = 1.5;

        let bounds = Rect::from_center_size(Vec2::new(0., 200.), Vec2::new(200., 200.));

        draw_bounds(&mut gizmos, bounds);

        draw_gizmo_line_plot_2d_raw(
            lines.wave.iter(),
            &mut gizmos,
            bounds,
            min_xy,
            max_xy,
            Srgba::GREEN.into(),
        );
    }

    // Wave set y clamped
    {
        let (mut min_xy, mut max_xy) =
            points_min_max(lines.wave.iter()).unwrap_or((Vec2::splat(-10.), Vec2::splat(10.)));

        min_xy.y = -0.5;
        max_xy.y = 1.5;

        let bounds = Rect::from_center_size(Vec2::new(400., 200.), Vec2::new(200., 200.));

        draw_bounds(&mut gizmos, bounds);

        draw_gizmo_line_plot_2d_clamped(
            lines.wave.iter(),
            &mut gizmos,
            bounds,
            min_xy,
            max_xy,
            Srgba::GREEN.into(),
        );
    }

    // Spikey derived bounds
    {
        let point_count = 50;
        let last_n_index = lines.spikey.len().saturating_sub(point_count);
        let (min_xy, max_xy) = points_min_max(lines.spikey.iter().skip(last_n_index))
            .unwrap_or((Vec2::splat(-10.), Vec2::splat(10.)));

        let bounds = Rect::from_center_size(Vec2::new(-400., -200.), Vec2::new(200., 200.));

        let [mut min_text, mut max_text] = text.many_mut(lines.spikey_y_scale_ui);
        min_text.sections[0].value = format!("{:.1}", min_xy.y);
        max_text.sections[0].value = format!("{:.1}", max_xy.y);

        draw_bounds(&mut gizmos, bounds);

        draw_gizmo_line_plot_2d_raw(
            lines.spikey.iter().skip(last_n_index),
            &mut gizmos,
            bounds,
            min_xy,
            max_xy,
            Srgba::GREEN.into(),
        );
    }

    // Spikey set y
    {
        let (mut min_xy, mut max_xy) =
            points_min_max(lines.spikey.iter()).unwrap_or((Vec2::splat(-10.), Vec2::splat(10.)));

        min_xy.y = -4.;
        max_xy.y = 4.;

        let bounds = Rect::from_center_size(Vec2::new(0., -200.), Vec2::new(200., 200.));

        draw_bounds(&mut gizmos, bounds);

        draw_gizmo_line_plot_2d_raw(
            lines.spikey.iter(),
            &mut gizmos,
            bounds,
            min_xy,
            max_xy,
            Srgba::GREEN.into(),
        );
    }

    // Spikey set y clamped
    {
        let (mut min_xy, mut max_xy) =
            points_min_max(lines.spikey.iter()).unwrap_or((Vec2::splat(-10.), Vec2::splat(10.)));

        min_xy.y = 0.;
        max_xy.y = 4.;

        let bounds = Rect::from_center_size(Vec2::new(400., -200.), Vec2::new(200., 200.));

        draw_bounds(&mut gizmos, bounds);

        draw_gizmo_line_plot_2d_clamped(
            lines.spikey.iter(),
            &mut gizmos,
            bounds,
            min_xy,
            max_xy,
            Srgba::GREEN.into(),
        );
    }
}

fn draw_bounds(gizmos: &mut Gizmos, bounds: Rect) {
    gizmos.rect_2d(
        bounds.center(),
        0.,
        bounds.size() + Vec2::splat(2.0),
        Color::WHITE.with_alpha(0.1),
    );
}
