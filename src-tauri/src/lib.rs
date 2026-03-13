use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

#[derive(Clone, Serialize, Deserialize, Default)]
struct ZapretLists {
    list_general: String,
    list_general_user: String,
    list_exclude: String,
    list_exclude_user: String,
    ipset_all: String,
    ipset_exclude: String,
    ipset_exclude_user: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct ZapretStrategy {
    id: String,
    label: String,
    description: String,
}

fn built_in_strategies() -> Vec<ZapretStrategy> {
    vec![
        ZapretStrategy {
            id: "default".into(),
            label: "Default".into(),
            description: "Основная стратегия general.bat".into(),
        },
        ZapretStrategy {
            id: "ALT".into(),
            label: "ALT".into(),
            description: "Альтернативная стратегия general (ALT).bat".into(),
        },
        ZapretStrategy {
            id: "ALT2".into(),
            label: "ALT2".into(),
            description: "Альтернативная стратегия general (ALT2).bat".into(),
        },
        ZapretStrategy {
            id: "ALT3".into(),
            label: "ALT3".into(),
            description: "Альтернативная стратегия general (ALT3).bat".into(),
        },
        ZapretStrategy {
            id: "ALT4".into(),
            label: "ALT4".into(),
            description: "Альтернативная стратегия general (ALT4).bat".into(),
        },
        ZapretStrategy {
            id: "ALT5".into(),
            label: "ALT5".into(),
            description: "Альтернативная стратегия general (ALT5).bat".into(),
        },
        ZapretStrategy {
            id: "ALT6".into(),
            label: "ALT6".into(),
            description: "Альтернативная стратегия general (ALT6).bat".into(),
        },
        ZapretStrategy {
            id: "ALT7".into(),
            label: "ALT7".into(),
            description: "Альтернативная стратегия general (ALT7).bat".into(),
        },
        ZapretStrategy {
            id: "ALT8".into(),
            label: "ALT8".into(),
            description: "Альтернативная стратегия general (ALT8).bat".into(),
        },
        ZapretStrategy {
            id: "ALT9".into(),
            label: "ALT9".into(),
            description: "Альтернативная стратегия general (ALT9).bat".into(),
        },
    ]
}

#[tauri::command]
fn get_zapret_strategies() -> Vec<ZapretStrategy> {
    built_in_strategies()
}

fn lists_dir(base: &Path) -> PathBuf {
    base.join("lists")
}

fn bin_dir(base: &Path) -> PathBuf {
    base.join("bin")
}

fn ensure_file_with_default(path: &Path, default_content: &str) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("failed to create dir {:?}: {e}", parent))?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|e| format!("failed to create file {:?}: {e}", path))?;

    file.write_all(default_content.as_bytes())
        .map_err(|e| format!("failed to write file {:?}: {e}", path))?;

    Ok(())
}

// Аналог :load_user_lists из service.bat
fn ensure_user_lists(base: &Path) -> Result<(), String> {
    let lists = lists_dir(base);

    let ipset_exclude_user = lists.join("ipset-exclude-user.txt");
    let list_general_user = lists.join("list-general-user.txt");
    let list_exclude_user = lists.join("list-exclude-user.txt");

    ensure_file_with_default(&ipset_exclude_user, "203.0.113.113/32\r\n")?;
    ensure_file_with_default(&list_general_user, "domain.example.abc\r\n")?;
    ensure_file_with_default(&list_exclude_user, "domain.example.abc\r\n")?;

    Ok(())
}

// Аналог :tcp_enable
fn enable_tcp_timestamps() -> Result<(), String> {
    Command::new("netsh")
        .args(["interface", "tcp", "set", "global", "timestamps=enabled"])
        .output()
        .map_err(|e| format!("failed to run netsh: {e}"))?;

    Ok(())
}

#[tauri::command]
fn get_zapret_lists(state: State<'_, Mutex<ZapretLists>>) -> Result<ZapretLists, String> {
    let guard = state
        .lock()
        .map_err(|_| "failed to lock zapret lists state".to_string())?;
    Ok(guard.clone())
}

#[tauri::command]
fn update_zapret_lists(
    state: State<'_, Mutex<ZapretLists>>,
    lists: ZapretLists,
) -> Result<(), String> {
    let mut guard = state
        .lock()
        .map_err(|_| "failed to lock zapret lists state".to_string())?;
    *guard = lists;
    Ok(())
}

fn read_list_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

#[tauri::command]
fn apply_zapret_preset(
    app: tauri::AppHandle,
    state: State<'_, Mutex<ZapretLists>>,
    name: String,
) -> Result<ZapretLists, String> {
    // пока один пресет "original", читаем списки из ресурсов zapret/lists
    if name != "original" {
        return Err(format!("unknown preset: {name}"));
    }

    let zapret_base: PathBuf = app
        .path()
        .resolve("zapret", tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("failed to resolve zapret resources dir: {e}"))?;
    let lists_dir = lists_dir(&zapret_base);

    let mut lists = ZapretLists::default();
    lists.list_general = read_list_file(&lists_dir.join("list-general.txt"));
    lists.list_general_user = read_list_file(&lists_dir.join("list-general-user.txt"));
    lists.list_exclude = read_list_file(&lists_dir.join("list-exclude.txt"));
    lists.list_exclude_user = read_list_file(&lists_dir.join("list-exclude-user.txt"));
    lists.ipset_all = read_list_file(&lists_dir.join("ipset-all.txt"));
    lists.ipset_exclude = read_list_file(&lists_dir.join("ipset-exclude.txt"));
    lists.ipset_exclude_user = read_list_file(&lists_dir.join("ipset-exclude-user.txt"));

    {
        let mut guard = state
            .lock()
            .map_err(|_| "failed to lock zapret lists state".to_string())?;
        *guard = lists.clone();
    }

    Ok(lists)
}

// Запуск основной стратегии general.bat без использования .bat файла
// Параметры game_filter_tcp/game_filter_udp можно передавать из UI (то, что раньше шло из GameFilterTCP/GameFilterUDP)
#[tauri::command]
fn run_default_strategy(
    app: tauri::AppHandle,
    game_filter_tcp: Option<String>,
    game_filter_udp: Option<String>,
) -> Result<(), String> {
    // bin\ лежит в ресурсах приложения: resources/zapret/bin
    let zapret_base: PathBuf = app
        .path()
        .resolve("zapret", tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("failed to resolve zapret resources dir: {e}"))?;
    let bin = bin_dir(&zapret_base);

    // temp dir для генерации списков из стора
    let temp_lists_dir: PathBuf = app
        .path()
        .resolve("zapret-lists", tauri::path::BaseDirectory::Temp)
        .map_err(|e| format!("failed to resolve temp zapret lists dir: {e}"))?;
    fs::create_dir_all(&temp_lists_dir)
        .map_err(|e| format!("failed to create temp lists dir: {e}"))?;

    if !bin.join("winws.exe").exists() {
        return Err(format!("winws.exe not found in {:?}", bin));
    }

    // Подготовка списков по умолчанию и включение TCP timestamps, как делали general.bat + service.bat
    ensure_user_lists(&zapret_base)?;
    let _ = enable_tcp_timestamps();

    // забираем актуальные списки из стора
    let lists_state: State<Mutex<ZapretLists>> = app.state::<Mutex<ZapretLists>>();
    let lists_data = {
        let guard = lists_state
            .lock()
            .map_err(|_| "failed to lock zapret lists state".to_string())?;
        guard.clone()
    };

    // helper для записи строки в файл
    fn write_list_file(dir: &Path, name: &str, content: &str) -> Result<PathBuf, String> {
        let path = dir.join(name);
        let mut file = fs::File::create(&path)
            .map_err(|e| format!("failed to create list file {:?}: {e}", path))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("failed to write list file {:?}: {e}", path))?;
        Ok(path)
    }

    // генерим файлы из стора
    let list_general_path =
        write_list_file(&temp_lists_dir, "list-general.txt", &lists_data.list_general)?;
    let list_general_user_path =
        write_list_file(&temp_lists_dir, "list-general-user.txt", &lists_data.list_general_user)?;
    let list_exclude_path =
        write_list_file(&temp_lists_dir, "list-exclude.txt", &lists_data.list_exclude)?;
    let list_exclude_user_path =
        write_list_file(&temp_lists_dir, "list-exclude-user.txt", &lists_data.list_exclude_user)?;
    let ipset_all_path =
        write_list_file(&temp_lists_dir, "ipset-all.txt", &lists_data.ipset_all)?;
    let ipset_exclude_path =
        write_list_file(&temp_lists_dir, "ipset-exclude.txt", &lists_data.ipset_exclude)?;
    let ipset_exclude_user_path = write_list_file(
        &temp_lists_dir,
        "ipset-exclude-user.txt",
        &lists_data.ipset_exclude_user,
    )?;

    let game_tcp = game_filter_tcp.unwrap_or_default();
    let game_udp = game_filter_udp.unwrap_or_default();

    let mut wf_tcp = vec!["80", "443", "2053", "2083", "2087", "2096", "8443"];
    if !game_tcp.trim().is_empty() {
        wf_tcp.push(&game_tcp);
    }

    let mut wf_udp = vec!["443", "19294-19344", "50000-50100"];
    if !game_udp.trim().is_empty() {
        wf_udp.push(&game_udp);
    }

    let wf_tcp_arg = wf_tcp.join(",");
    let wf_udp_arg = wf_udp.join(",");

    let winws = bin.join("winws.exe");

    // Собираем аргументы в один список, чтобы передать их в PowerShell / Start-Process -Verb RunAs
    let args: Vec<String> = vec![
        format!("--wf-tcp={}", wf_tcp_arg),
        format!("--wf-udp={}", wf_udp_arg),
        "--filter-udp=443".into(),
        format!("--hostlist={}", list_general_path.display()),
        format!("--hostlist={}", list_general_user_path.display()),
        format!("--hostlist-exclude={}", list_exclude_path.display()),
        format!("--hostlist-exclude={}", list_exclude_user_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_user_path.display()),
        "--dpi-desync=fake".into(),
        "--dpi-desync-repeats=6".into(),
        format!(
            "--dpi-desync-fake-quic={}",
            bin.join("quic_initial_www_google_com.bin").display()
        ),
        "--new".into(),
        "--filter-udp=19294-19344,50000-50100".into(),
        "--filter-l7=discord,stun".into(),
        "--dpi-desync=fake".into(),
        "--dpi-desync-repeats=6".into(),
        "--new".into(),
        "--filter-tcp=2053,2083,2087,2096,8443".into(),
        "--hostlist-domains=discord.media".into(),
        "--dpi-desync=multisplit".into(),
        "--dpi-desync-split-seqovl=681".into(),
        "--dpi-desync-split-pos=1".into(),
        format!(
            "--dpi-desync-split-seqovl-pattern={}",
            bin.join("tls_clienthello_www_google_com.bin").display()
        ),
        "--new".into(),
        "--filter-tcp=443".into(),
        format!("--hostlist={}", ipset_all_path.display()), // при необходимости замени на отдельное поле
        "--ip-id=zero".into(),
        "--dpi-desync=multisplit".into(),
        "--dpi-desync-split-seqovl=681".into(),
        "--dpi-desync-split-pos=1".into(),
        format!(
            "--dpi-desync-split-seqovl-pattern={}",
            bin.join("tls_clienthello_www_google_com.bin").display()
        ),
        "--new".into(),
        "--filter-tcp=80,443".into(),
        format!("--hostlist={}", list_general_path.display()),
        format!("--hostlist={}", list_general_user_path.display()),
        format!("--hostlist-exclude={}", list_exclude_path.display()),
        format!("--hostlist-exclude={}", list_exclude_user_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_user_path.display()),
        "--dpi-desync=multisplit".into(),
        "--dpi-desync-split-seqovl=568".into(),
        "--dpi-desync-split-pos=1".into(),
        format!(
            "--dpi-desync-split-seqovl-pattern={}",
            bin.join("tls_clienthello_4pda_to.bin").display()
        ),
        "--new".into(),
        "--filter-udp=443".into(),
        format!("--ipset={}", ipset_all_path.display()),
        format!("--hostlist-exclude={}", list_exclude_path.display()),
        format!("--hostlist-exclude={}", list_exclude_user_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_user_path.display()),
        "--dpi-desync=fake".into(),
        "--dpi-desync-repeats=6".into(),
        format!(
            "--dpi-desync-fake-quic={}",
            bin.join("quic_initial_www_google_com.bin").display()
        ),
        "--new".into(),
        "--filter-tcp=80,443,8443".into(),
        format!("--ipset={}", ipset_all_path.display()),
        format!("--hostlist-exclude={}", list_exclude_path.display()),
        format!("--hostlist-exclude={}", list_exclude_user_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_user_path.display()),
        "--dpi-desync=multisplit".into(),
        "--dpi-desync-split-seqovl=568".into(),
        "--dpi-desync-split-pos=1".into(),
        format!(
            "--dpi-desync-split-seqovl-pattern={}",
            bin.join("tls_clienthello_4pda_to.bin").display()
        ),
        "--new".into(),
        format!("--filter-tcp={}", game_tcp),
        format!("--ipset={}", ipset_all_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_user_path.display()),
        "--dpi-desync=multisplit".into(),
        "--dpi-desync-any-protocol=1".into(),
        "--dpi-desync-cutoff=n3".into(),
        "--dpi-desync-split-seqovl=568".into(),
        "--dpi-desync-split-pos=1".into(),
        format!(
            "--dpi-desync-split-seqovl-pattern={}",
            bin.join("tls_clienthello_4pda_to.bin").display()
        ),
        "--new".into(),
        format!("--filter-udp={}", game_udp),
        format!("--ipset={}", ipset_all_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_path.display()),
        format!("--ipset-exclude={}", ipset_exclude_user_path.display()),
        "--dpi-desync=fake".into(),
        "--dpi-desync-repeats=12".into(),
        "--dpi-desync-any-protocol=1".into(),
        format!(
            "--dpi-desync-fake-unknown-udp={}",
            bin.join("quic_initial_www_google_com.bin").display()
        ),
        "--dpi-desync-cutoff=n2".into(),
    ];

    // Собираем строку для PowerShell и запускаем winws.exe с запросом прав администратора (UAC)
    let args_joined = args.join(" ");
    let ps_command = format!(
        "Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs",
        winws.display(),
        args_joined
    );

    Command::new("powershell")
        .current_dir(&bin)
        .args(["-NoProfile", "-Command", &ps_command])
        .spawn()
        .map_err(|e| format!("failed to start winws.exe via PowerShell: {e}"))?;

    Ok(())
}

// Остановка zapret (winws.exe) через taskkill
#[tauri::command]
fn stop_zapret() -> Result<(), String> {
    Command::new("cmd")
        .arg("/C")
        .arg("taskkill /F /IM winws.exe")
        .spawn()
        .map_err(|e| format!("failed to stop winws.exe: {e}"))?;

    Ok(())
}

// Проверка: запущен ли сейчас winws.exe
#[tauri::command]
fn is_zapret_running() -> Result<bool, String> {
    let output = Command::new("cmd")
        .arg("/C")
        .arg("tasklist /FI \"IMAGENAME eq winws.exe\"")
        .output()
        .map_err(|e| format!("failed to run tasklist: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.contains("winws.exe"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(ZapretLists::default()))
        .invoke_handler(tauri::generate_handler![
            get_zapret_strategies,
            get_zapret_lists,
            update_zapret_lists,
            apply_zapret_preset,
            run_default_strategy,
            stop_zapret,
            is_zapret_running
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
