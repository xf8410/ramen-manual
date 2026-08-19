//! JNI bridge: Android supplies the extracted data directory, then receives
//! serialized UI snapshots and submits only candidate indices.

use std::path::Path;
use std::sync::{Mutex, OnceLock};
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jstring};
use jni::JNIEnv;
use crate::{RamenGameAdapter, RamenGameDriver, RamenMobileConfig, TouchSession, UiState};

type Session = TouchSession<RamenGameDriver<RamenGameAdapter>>;
static DATA_ROOT: OnceLock<Mutex<Option<String>>> = OnceLock::new();
static SESSION: OnceLock<Mutex<Option<Session>>> = OnceLock::new();
fn data_root() -> &'static Mutex<Option<String>> { DATA_ROOT.get_or_init(|| Mutex::new(None)) }
fn session() -> &'static Mutex<Option<Session>> { SESSION.get_or_init(|| Mutex::new(None)) }
fn json_state(state: &UiState) -> String { serde_json::to_string(state).unwrap_or_else(|e| error_json(e.to_string())) }
fn error_json(message: impl ToString) -> String { serde_json::json!({"status":"error", "message": message.to_string()}).to_string() }
fn java_string<'local>(env: &mut JNIEnv<'local>, value: String) -> jstring { env.new_string(value).map(|s| s.into_raw()).unwrap_or(std::ptr::null_mut()) }

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeSetDataRoot(mut env: JNIEnv, _class: JClass, root: JString) -> jboolean {
    match env.get_string(&root).map(|s| s.to_string_lossy().into_owned()) {
        Ok(path) if Path::new(&path).is_dir() => { *data_root().lock().expect("data root mutex") = Some(path); 1 }
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeStart(mut env: JNIEnv, _class: JClass) -> jstring {
    let result = (|| {
        let root = data_root().lock().expect("data root mutex").clone().ok_or_else(|| "尚未设置运行数据目录".to_string())?;
        std::env::set_current_dir(root).map_err(|e| e.to_string())?;
        let driver = RamenGameDriver::new(RamenGameAdapter::new(RamenMobileConfig::default()));
        let mut new_session = TouchSession::new(driver);
        new_session.start().map_err(|e| e.to_string())?;
        let state = new_session.ui_state();
        *session().lock().expect("session mutex") = Some(new_session);
        Ok::<String, String>(json_state(&state))
    })();
    java_string(&mut env, result.unwrap_or_else(error_json))
}

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeSubmit(mut env: JNIEnv, _class: JClass, index: i32) -> jstring {
    let result = (|| {
        if index < 0 { return Err("提交索引不能为负数".to_string()); }
        let mut guard = session().lock().expect("session mutex");
        let current = guard.as_mut().ok_or_else(|| "游戏尚未启动".to_string())?;
        current.submit(index as usize).map_err(|e| format!("提交失败: {:?}", e))?;
        Ok::<String, String>(json_state(&current.ui_state()))
    })();
    java_string(&mut env, result.unwrap_or_else(error_json))
}

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeReset(_env: JNIEnv, _class: JClass) {
    *session().lock().expect("session mutex") = None;
    *data_root().lock().expect("data root mutex") = None;
}
