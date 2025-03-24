#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Bevy’s built-in plugins
        .add_systems(Startup, spawn_test_sprite) // Add a test object
        .run();
}

// Simple test object (red square) to check if Bevy renders
fn spawn_test_sprite(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()); // Adds a 2D camera

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform::from_scale(Vec3::splat(50.0)), // 50x50 size
        ..Default::default()
    });
}