//! A compute shader that simulates Conway's Game of Life.
//!
//! Compute shaders use the GPU for computing arbitrary information, that may be independent of what
//! is rendered to the screen.

use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        RenderApp, RenderStage,
    },
};
use std::borrow::Cow;

const SIZE: (u32, u32) = (1920, 1080);
const WORKGROUP_SIZE: u32 = 8;

/// Setup function; creates and initializes everything needed for the game to run
pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        // Initialize new black image
        Extent3d {
            // Bounds of the texture (takes up the whole window, so defaults to 1920×1080 pixels)
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,      // Is a 2D texture
        &[0, 0, 0, 255],           // All pixels should be black with full alpha
        TextureFormat::Rgba8Unorm, // The format of the above pixel data is in 8-bit RGBA
    );
    image.texture_descriptor.usage =    // Panic if any other texture than the following is provided
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image = images.add(image); // Set image to be a mutable Bevy asset reference to the same image

    commands.spawn(SpriteBundle { // Spawn the texture as a sprite enclosed in the SpriteBundle type
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)), // idk why it needs to be a float; what, are you going to have 1/3 of a pixel left over? no. 
            ..default()
        },
        texture: image.clone(), // Clone rendered image's enclosed texture into the sprite texture
        ..default()
    });
    commands.spawn(Camera2dBundle::default()); // Spawn in 2D camera

    commands.insert_resource(GameOfLifeImage(image)); // Insert image as a resource
}

pub struct GameOfLifeComputePlugin;

impl Plugin for GameOfLifeComputePlugin {
    fn build(&self, app: &mut App) {
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugin(ExtractResourcePlugin::<GameOfLifeImage>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<GameOfLifePipeline>() // Add Game of Life Render Pipeline into the render subapp component
            .add_system_to_stage(RenderStage::Queue, queue_bind_group); // Queue binding the generated render pipelines and images as a Bevy BindGroup type resource.

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>(); // Mutable RenderGraph resource creation.
        render_graph.add_node("game_of_life", GameOfLifeNode::default()); // Adds a RenderGraph node for the Game of Life, initialized to the default state of GameOfLife::Loading
        render_graph
            .add_node_edge(
                // Ensures output is run before input (keeps issues from happening with asynchronicity affecting frame calculations afaik)
                "game_of_life",
                bevy::render::main_graph::node::CAMERA_DRIVER,
            )
            .unwrap(); // Panic if it failed to ensure synchronous calculations
    }
}

#[derive(Resource, Clone, Deref, ExtractResource)]
struct GameOfLifeImage(Handle<Image>);

#[derive(Resource)]
struct GameOfLifeImageBindGroup(BindGroup);

/// Boilerplate function to queue binding the given pipeline, gpu images, and game images to a Bevy BindGroup Resource.
fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<GameOfLifePipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    game_of_life_image: Res<GameOfLifeImage>,
    render_device: Res<RenderDevice>,
) {
    let view = &gpu_images[&game_of_life_image.0]; // A reference to a GPU represesntation of the generated image.
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        // BindGroup creation
        label: None, // Debug label; unnecessary in current state so omitted
        layout: &pipeline.texture_bind_group_layout, // Set layout to the render pipeline's default layout
        entries: &[BindGroupEntry {
            binding: 0,                                                 // Binds at index 0
            resource: BindingResource::TextureView(&view.texture_view), // The texture view to bind as a resource
        }],
    });
    commands.insert_resource(GameOfLifeImageBindGroup(bind_group)); // BindGroup compartmentalizing and insertion
}

#[derive(Resource)]
pub struct GameOfLifePipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for GameOfLifePipeline {
    fn from_world(world: &mut World) -> Self {
        // Creates new RenderPipeline using data from the World
        let texture_bind_group_layout = world
            .resource::<RenderDevice>() // Get a reference to the RenderDevice
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                // Make a BindGroup layout for the compute shader using the below descriptor
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,                        // Binds at index 0
                    visibility: ShaderStages::COMPUTE, // Is definitely a compute shader (as it has to calculate each frame on the GPU using the previous render pass's data)
                    ty: BindingType::StorageTexture {
                        // Sets what type of texture is being bound
                        access: StorageTextureAccess::ReadWrite, // Gives R/W access
                        format: TextureFormat::Rgba8Unorm, // Sets texture format to be 8-bit depth RGBA
                        view_dimension: TextureViewDimension::D2, // Sets texture dimensions to be 2D
                    },
                    count: None, // This is a single texture and not an array.
                }],
            });
        let shader = world
            .resource_mut::<AssetServer>()
            .load("shaders/game_of_life.wgsl");
        let mut pipeline_cache = world.resource_mut::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: Some(vec![texture_bind_group_layout.clone()]),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: Some(vec![texture_bind_group_layout.clone()]),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        GameOfLifePipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

enum GameOfLifeState {
    Loading,
    Init,
    Update,
}

struct GameOfLifeNode {
    state: GameOfLifeState,
}

impl Default for GameOfLifeNode {
    fn default() -> Self {
        Self {
            state: GameOfLifeState::Loading,
        }
    }
}

impl render_graph::Node for GameOfLifeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<GameOfLifePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            GameOfLifeState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = GameOfLifeState::Init;
                }
            }
            GameOfLifeState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = GameOfLifeState::Update;
                }
            }
            GameOfLifeState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world.resource::<GameOfLifeImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<GameOfLifePipeline>();

        let mut pass = render_context
            .command_encoder
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            GameOfLifeState::Loading => {}
            GameOfLifeState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            GameOfLifeState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }

        Ok(())
    }
}
