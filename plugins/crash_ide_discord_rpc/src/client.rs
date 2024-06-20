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
    rpc_client: Option<Arc<Mutex<Client>>>,
    start_time: u64,
}

impl Default for DiscordRpcClient {
    fn default() -> Self {
        Self {
            rpc_client: None,
            start_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

impl DiscordRpcClient {
    pub fn is_some(&self) -> bool {
        self.rpc_client.is_some()
    }

    pub fn set_activity<F>(&mut self, f: F) -> DiscordTaskType<SetActivityMarker> where F: FnOnce(Activity) -> Activity + Send + 'static {
        let client = self.rpc_client.clone().unwrap();
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

    pub(crate) fn stop(&mut self) {
        if let Some(client) = self.rpc_client.take() {
            match Arc::try_unwrap(client) {
                Ok(mutex) => {
                    let inner = mutex.into_inner().unwrap();
                    inner.shutdown().unwrap();
                }
                Err(_) => {}
            }
        }
    }
}

pub(crate) struct SetActivityMarker;

pub(crate) type DiscordTaskType<T> = Task<Result<T, DiscordError>>;

#[derive(Component)]
pub(super) struct DiscordRpcTask<T>(pub DiscordTaskType<T>);

pub(super) fn init_client(
    mut commands: Commands,
    settings: Res<DiscordRpcConfig>,
) {
    if !settings.active {
        return;
    }

    let task: DiscordTaskType<Option<Arc<Mutex<Client>>>> = create_client_task();

    commands.spawn(DiscordRpcTask(task));
}

pub(super) fn create_client_task() -> DiscordTaskType<Option<Arc<Mutex<Client>>>> {
    let pool = AsyncComputeTaskPool::get();
    let task: DiscordTaskType<Option<Arc<Mutex<Client>>>> = pool.spawn(async move {
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

        Ok(Some(Arc::new(Mutex::new(rpc))))
    });

    task
}

pub(super) fn finish_loading(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DiscordRpcTask<Option<Arc<Mutex<Client>>>>)>,
    mut discord_rpc_client: ResMut<DiscordRpcClient>,
) {
    for (entity, mut task) in query.iter_mut() {
        let Some(result) = block_on(future::poll_once(&mut task.0)) else {
            continue;
        };

        match result {
            Ok(client) => {
                discord_rpc_client.rpc_client = client;
            },
            Err(e) => {
                println!("Discord RPC could not be initialized: {}", e);
            }
        }

        commands.entity(entity).despawn();
    }
}