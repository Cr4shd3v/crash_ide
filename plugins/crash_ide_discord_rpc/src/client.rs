use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, block_on, Task};
use bevy::tasks::futures_lite::future;
use discord_presence::{Client, DiscordError};
use discord_presence::models::Activity;
use crate::config::DiscordRpcConfig;
use crate::status::DiscordRpcActivity;

#[derive(Resource)]
pub struct DiscordRpcClient {
    rpc_client: Arc<Mutex<Client>>,
    start_time: u64,
}

pub(crate) struct SetActivityMarker;

impl DiscordRpcClient {
    pub fn set_activity<F>(&mut self, f: F) -> DiscordTaskType<SetActivityMarker> where F: FnOnce(Activity) -> Activity + Send + 'static {
        let client = self.rpc_client.clone();
        let start_time = self.start_time;

        let pool = AsyncComputeTaskPool::get();
        pool.spawn(async move {
            let mut lock = client.lock().unwrap();
            lock.set_activity(|act| {
                f(act)
                    .timestamps(|time| {
                        time.start(start_time)
                    })
            })?;

            Ok(SetActivityMarker)
        })
    }
}

type DiscordTaskType<T> = Task<Result<T, DiscordError>>;

#[derive(Component)]
pub(super) struct DiscordRpcTask<T>(pub DiscordTaskType<T>);

pub(super) fn init_client(
    mut commands: Commands,
    settings: Res<DiscordRpcConfig>,
) {
    if !settings.active {
        return;
    }

    let pool = AsyncComputeTaskPool::get();

    let task: DiscordTaskType<DiscordRpcClient> = pool.spawn(async move {
        let mut rpc = Client::new(1251218926595997736);

        rpc.on_ready(|_| {
            println!("Discord RPC started");
        }).persist();

        rpc.start();

        rpc.block_until_event(discord_presence::Event::Ready).unwrap();
        let start_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        rpc.set_activity(|act| {
            act.state(DiscordRpcActivity::DEFAULT_ACTIVITY)
                .timestamps(|time| {
                    time.start(start_time)
                })
        }).unwrap();

        Ok(DiscordRpcClient {
            rpc_client: Arc::new(Mutex::new(rpc)),
            start_time,
        })
    });

    commands.spawn(DiscordRpcTask(task));
}

pub(super) fn finish_loading(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DiscordRpcTask<DiscordRpcClient>)>,
) {
    for (entity, mut task) in query.iter_mut() {
        let Some(result) = block_on(future::poll_once(&mut task.0)) else {
            continue;
        };

        match result {
            Ok(client) => {
                commands.insert_resource(client);
            },
            Err(e) => {
                println!("Discord RPC could not be initialized: {}", e);
            }
        }

        commands.entity(entity).despawn();
    }
}