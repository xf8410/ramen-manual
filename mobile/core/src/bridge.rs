//! Minimal JNI bridge. The Android UI receives serialized `UiState` and submits
//! only the selected candidate index; all simulation rules remain in RamenGame.

use std::cell::RefCell;

use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

use crate::{RamenGameAdapter, RamenGameDriver, RamenMobileConfig, TouchSession, UiState};

thread_local! {
    static SESSION: RefCell<Option<TouchSession<RamenGameDriver<RamenGameAdapter>>>> = const { RefCell::new(None) };
}

fn json_state(state: &UiState) -> String {
    serde_json::to_string(state).unwrap_or_else(|error| format!(r#"{{"status":"error","message":{}}}"#, serde_json::to_string(&error.to_string()).unwrap()))
}

fn error_state(message: impl ToString) -> String {
    serde_json::to_string(&serde_json::json!({"status":"error", "message": message.to_string()})).unwrap()
}

fn to_java_string<'local>(env: &mut JNIEnv<'local>, value: String) -> jstring {
    env.new_string(value).map(|s| s.into_raw()).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeStart(mut env: JNIEnv, _class: JClass) -> jstring {
    let result = (|| {
        let adapter = RamenGameAdapter::new(RamenMobileConfig::default());
        let driver = RamenGameDriver::new(adapter);
        let mut session = TouchSession::new(driver);
        session.start().map_err(|e| e.to_string())?;
        let state = session.ui_state();
        SESSION.with(|slot| *slot.borrow_mut() = Some(session));
        Ok::<String, String>(json_state(&state))
    })();
    to_java_string(&mut env, result.unwrap_or_else(error_state))
}

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeSubmit(mut env: JNIEnv, _class: JClass, index: i32) -> jstring {
    let result = SESSION.with(|slot| {
        let mut session = slot.borrow_mut();
        let Some(session) = session.as_mut() else { return Err("游戏尚未启动".to_string()); };
        session.submit(index.max(0) as usize).map_err(|e| format!("提交失败: {:?}", e))?;
        Ok::<String, String>(json_state(&session.ui_state()))
    });
    to_java_string(&mut env, result.unwrap_or_else(error_state))
}

#[no_mangle]
pub extern "system" fn Java_com_umaai_ramen_MainActivity_nativeReset(_env: JNIEnv, _class: JClass) {
    SESSION.with(|slot| *slot.borrow_mut() = None);
}
