//! Pong tutorial 1
mod pong;

use crate::pong::Pong;


use amethyst::{
    prelude::*,
    renderer::
    {
        plugins::
        {
            RenderFlat2D,
            RenderToWindow
        },
        
        types::
            DefaultBackend,
            RenderingBundle,
    },
    core::transform::TransformBundle,
    utils::application_root_dir,
};


fn main() -> amethyst::Result<()>
{
    // start logging
    amethyst::start_logger(Default::default());

    // load config files from application root directory
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // define asset directory
    let assets_dir = app_root.join("assets");

    // basic application setup
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;



    // create game instance and run
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    

    // Return the success value
    Ok(())

    
}