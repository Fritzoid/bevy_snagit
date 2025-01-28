use bevy::prelude::*;
use bevy::render::renderer::RenderContext;
use bevy::render::render_graph::{Node, RenderGraph, RenderGraphContext, NodeRunError};

#[derive(Component)]
pub struct Snagit;

pub struct SnagitNode;

impl Node for SnagitNode {
    fn run(
        &self, 
        graph_context: &mut RenderGraphContext, 
        render_context: &mut RenderContext, 
        world: &World,
    ) -> Result<(), NodeRunError> {





        Ok(())
    }
}

pub struct SnagitPlugin;

impl Plugin for SnagitPlugin {
    fn build(&self, app:&mut App) {
        app..add_systems(PreStartup, setup_snagit);
    }
}

fn setup_snagit(mut render_graph: ResMut<RenderGraph>)
{
}