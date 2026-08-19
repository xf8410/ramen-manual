//! JNI bridge: Android supplies the extracted data directory, then receives
//! serialized UI snapshots and submits only candidate indices.

use std::cell::RefCell;
use std::path::Path;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jstring};
use jni::JNIEnv;
use crate::{RamenGameAdapter, RamenGameDriver, RamenMobileConfig, TouchSession, UiState};

thread_local! { static DATA_ROOT: RefCell<Option<String>> = const { RefCell::new(None) }; }
thread_local! { static SESSION: RefCell<Option<TouchSession<RamenGameDriver<RamenGameAdapter>>>> = const { RefCell::new(None) }; }
fn json_state(state: &UiState) -> String { serde_json::to_string(state).unwrap_or_else(|e| error_json(e.to_string())) }
fn error_json(message: impl ToString) -> String { serde_json::json!({"status":"error", "message": message.to_string()}).to_string() }
fn java_string<'local>(env: &mut JNIEnv<'local>, value: String) -> jstring { env.new_string(value).map(|s| s.into_raw()).unwrap_or(std::ptr::null_mut()) }

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeSetDataRoot(mut env: JNIEnv, _class: JClass, root: JString) -> jboolean {
    match env.get_string(&root).map(|s| s.to_string_lossy().into_owned()) {
        Ok(path) if Path::new(&path).is_dir() => { DATA_ROOT.with(|slot| *slot.borrow_mut() = Some(path)); 1 }
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeStart(mut env: JNIEnv, _class: JClass) -> jstring {
    let result = (|| {
        let root = DATA_ROOT.with(|slot| slot.borrow().clone()).ok_or_else(|| "尚未设置运行数据目录".to_string())?;
        std::env::set_current_dir(root).map_err(|e| e.to_string())?;
        let driver = RamenGameDriver::new(RamenGameAdapter::new(RamenMobileConfig::default()));
        let mut session = TouchSession::new(driver);
        session.start().map_err(|e| e.to_string())?;
        let state = session.ui_state();
        SESSION.with(|slot| *slot.borrow_mut() = Some(session));
        Ok::<String, String>(json_state(&state))
    })();
    java_string(&mut env, result.unwrap_or_else(error_json))
}

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeSubmit(mut env: JNIEnv, _class: JClass, index: i32) -> jstring {
    let result = SESSION.with(|slot| {
        if index < 0 { return Err("提交索引不能为负数".to_string()); }
        let mut session = slot.borrow_mut();
        let Some(session) = session.as_mut() else { return Err("游戏尚未启动".to_string()); };
        session.submit(index as usize).map_err(|e| format!("提交失败: {:?}", e))?;
        Ok::<String, String>(json_state(&session.ui_state()))
    });
    java_string(&mut env, result.unwrap_or_else(error_json))
}

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeReset(_env: JNIEnv, _class: JClass) {
    SESSION.with(|slot| *slot.borrow_mut() = None);
    DATA_ROOT.with(|slot| *slot.borrow_mut() = None);
}
