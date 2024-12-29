use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, State};

struct TimerState {
    remaining_time: Option<Duration>,
    is_running: bool,
    start_time: Option<Instant>,
    total_duration: Option<Duration>,
}

impl Default for TimerState {
    fn default() -> Self {
        TimerState {
            remaining_time: Some(Duration::from_secs(120)),
            is_running: false,
            start_time: Some(Instant::now()),
            total_duration: Some(Duration::from_secs(120)),
        }
    }
}

type SharedTimerState = Arc<Mutex<TimerState>>;

#[tauri::command]
async fn setup_timer(
    timer_state: State<'_, SharedTimerState>,
    duration_secs: f64,
) -> Result<(), ()> {
    let mut state = timer_state.lock().unwrap();
    state.remaining_time = Some(Duration::from_secs_f64(duration_secs));
    state.is_running = true;
    state.start_time = Some(Instant::now());
    state.total_duration = Some(Duration::from_secs_f64(duration_secs));
    Ok(())
}

#[tauri::command]
async fn start_timer(
    timer_state: State<'_, SharedTimerState>,
    app_handle: AppHandle,
) -> Result<(), ()> {
    loop {
        thread::sleep(Duration::from_millis(50));
        let mut remaining = None;
        let mut finished = false;

        {
            let mut state = timer_state.lock().unwrap();
            if state.is_running {
                if let Some(start_time) = state.start_time {
                    if let Some(total_duration) = state.total_duration {
                        let elapsed = Instant::now() - start_time;
                        if let Some(rem) = total_duration.checked_sub(elapsed) {
                            state.remaining_time = Some(rem);
                            remaining = Some(rem.as_secs_f64());
                        } else {
                            state.is_running = false;
                            state.remaining_time = None;
                            finished = true;
                        }
                    }
                }
            }
        }
        if let Some(rem) = remaining {
            app_handle.emit("timer-update", rem).ok();
        }
        if finished {
            println!("Finished!");
            app_handle.emit("timer-finished", ()).ok();
            break;
        }
    }
    Ok(())
}

#[tauri::command]
fn pause_timer(timer_state: State<'_, SharedTimerState>) {
    let mut state = timer_state.lock().unwrap();
    if state.is_running {
        state.is_running = false;
    }
}

#[tauri::command]
fn resume_timer(timer_state: State<'_, SharedTimerState>) {
    let mut state = timer_state.lock().unwrap();
    if !state.is_running {
        let resume_time = Some(Instant::now());
        state.start_time = resume_time;
        state.is_running = true;
        state.total_duration = state.remaining_time;
    }
}

#[tauri::command]
fn cancel_timer(timer_state: State<'_, SharedTimerState>) {
    let mut state = timer_state.lock().unwrap();
    state.is_running = false;
    state.remaining_time = None;
    state.start_time = None;
    state.total_duration = None;
}

pub fn run() {
    let timer_state: SharedTimerState = Arc::new(Mutex::new(TimerState::default()));

    tauri::Builder::default()
        .manage(timer_state.clone())
        .invoke_handler(tauri::generate_handler![
            setup_timer,
            start_timer,
            pause_timer,
            resume_timer,
            cancel_timer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
