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
    list_google: String,
    ipset_all: String,
    ipset_exclude: String,
    ipset_exclude_user: String,
}

/// Параметры стратегии для запуска winws (без списков — они в сторе).
/// protocols — описание портов (TCP/UDP), modes — типы DPI-desync, args — шаблон аргументов с плейсхолдерами.
#[derive(Clone, Serialize, Deserialize)]
struct ZapretStrategy {
    id: String,
    label: String,
    description: String,
    /// Человекочитаемое описание портов, напр. "TCP: 80, 443, 2053…; UDP: 443, 19294-19344…"
    protocols: String,
    /// Режимы обхода DPI, напр. "fake, multisplit, hostfakesplit"
    modes: String,
    /// Аргументы для winws.exe. Плейсхолдеры: {list_general}, {list_general_user}, {list_exclude}, {list_exclude_user}, {list_google}, {ipset_all}, {ipset_exclude}, {ipset_exclude_user}, {bin}, {game_tcp}, {game_udp}
    args: Vec<String>,
}

/// Аргументы стратегии default (general.bat) с плейсхолдерами для путей к спискам и bin.
fn default_strategy_args() -> Vec<String> {
    vec![
        "--wf-tcp=80,443,2053,2083,2087,2096,8443,{game_tcp}".into(),
        "--wf-udp=443,19294-19344,50000-50100,{game_udp}".into(),
        "--filter-udp=443".into(),
        "--hostlist={list_general}".into(),
        "--hostlist={list_general_user}".into(),
        "--hostlist-exclude={list_exclude}".into(),
        "--hostlist-exclude={list_exclude_user}".into(),
        "--ipset-exclude={ipset_exclude}".into(),
        "--ipset-exclude={ipset_exclude_user}".into(),
        "--dpi-desync=fake".into(),
        "--dpi-desync-repeats=6".into(),
        "--dpi-desync-fake-quic={bin}quic_initial_www_google_com.bin".into(),
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
        "--dpi-desync-split-seqovl-pattern={bin}tls_clienthello_www_google_com.bin".into(),
        "--new".into(),
        "--filter-tcp=443".into(),
        "--hostlist={list_google}".into(),
        "--ip-id=zero".into(),
        "--dpi-desync=multisplit".into(),
        "--dpi-desync-split-seqovl=681".into(),
        "--dpi-desync-split-pos=1".into(),
        "--dpi-desync-split-seqovl-pattern={bin}tls_clienthello_www_google_com.bin".into(),
        "--new".into(),
        "--filter-tcp=80,443".into(),
        "--hostlist={list_general}".into(),
        "--hostlist={list_general_user}".into(),
        "--hostlist-exclude={list_exclude}".into(),
        "--hostlist-exclude={list_exclude_user}".into(),
        "--ipset-exclude={ipset_exclude}".into(),
        "--ipset-exclude={ipset_exclude_user}".into(),
        "--dpi-desync=multisplit".into(),
        "--dpi-desync-split-seqovl=568".into(),
        "--dpi-desync-split-pos=1".into(),
        "--dpi-desync-split-seqovl-pattern={bin}tls_clienthello_4pda_to.bin".into(),
        "--new".into(),
        "--filter-udp=443".into(),
        "--ipset={ipset_all}".into(),
        "--hostlist-exclude={list_exclude}".into(),
        "--hostlist-exclude={list_exclude_user}".into(),
        "--ipset-exclude={ipset_exclude}".into(),
        "--ipset-exclude={ipset_exclude_user}".into(),
        "--dpi-desync=fake".into(),
        "--dpi-desync-repeats=6".into(),
        "--dpi-desync-fake-quic={bin}quic_initial_www_google_com.bin".into(),
        "--new".into(),
        "--filter-tcp=80,443,8443".into(),
        "--ipset={ipset_all}".into(),
        "--hostlist-exclude={list_exclude}".into(),
        "--hostlist-exclude={list_exclude_user}".into(),
        "--ipset-exclude={ipset_exclude}".into(),
        "--ipset-exclude={ipset_exclude_user}".into(),
        "--dpi-desync=multisplit".into(),
        "--dpi-desync-split-seqovl=568".into(),
        "--dpi-desync-split-pos=1".into(),
        "--dpi-desync-split-seqovl-pattern={bin}tls_clienthello_4pda_to.bin".into(),
        "--new".into(),
        "--filter-tcp={game_tcp}".into(),
        "--ipset={ipset_all}".into(),
        "--ipset-exclude={ipset_exclude}".into(),
        "--ipset-exclude={ipset_exclude_user}".into(),
        "--dpi-desync=multisplit".into(),
        "--dpi-desync-any-protocol=1".into(),
        "--dpi-desync-cutoff=n3".into(),
        "--dpi-desync-split-seqovl=568".into(),
        "--dpi-desync-split-pos=1".into(),
        "--dpi-desync-split-seqovl-pattern={bin}tls_clienthello_4pda_to.bin".into(),
        "--new".into(),
        "--filter-udp={game_udp}".into(),
        "--ipset={ipset_all}".into(),
        "--ipset-exclude={ipset_exclude}".into(),
        "--ipset-exclude={ipset_exclude_user}".into(),
        "--dpi-desync=fake".into(),
        "--dpi-desync-repeats=12".into(),
        "--dpi-desync-any-protocol=1".into(),
        "--dpi-desync-fake-unknown-udp={bin}quic_initial_www_google_com.bin".into(),
        "--dpi-desync-cutoff=n2".into(),
    ]
}

fn built_in_strategies() -> Vec<ZapretStrategy> {
    let default_args = default_strategy_args();
    vec![
        ZapretStrategy {
            id: "default".into(),
            label: "Default".into(),
            description: "Основная стратегия general.bat".into(),
            protocols: "TCP: 80, 443, 2053, 2083, 2087, 2096, 8443 + GameFilter; UDP: 443, 19294-19344, 50000-50100 + GameFilter".into(),
            modes: "fake, multisplit".into(),
            args: default_args.clone(),
        },
        ZapretStrategy {
            id: "ALT".into(),
            label: "ALT".into(),
            description: "Альтернативная стратегия general (ALT).bat".into(),
            protocols: "TCP: 80, 443, 2053… + GameFilter; UDP: 443, 19294-19344… + GameFilter".into(),
            modes: "fake, fakedsplit".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "ALT2".into(),
            label: "ALT2".into(),
            description: "Альтернативная стратегия general (ALT2).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "Вариант ALT2".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "ALT3".into(),
            label: "ALT3".into(),
            description: "Альтернативная стратегия general (ALT3).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "fake, hostfakesplit".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "ALT4".into(),
            label: "ALT4".into(),
            description: "Альтернативная стратегия general (ALT4).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "Вариант ALT4".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "ALT5".into(),
            label: "ALT5".into(),
            description: "Альтернативная стратегия general (ALT5).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "Вариант ALT5".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "ALT6".into(),
            label: "ALT6".into(),
            description: "Альтернативная стратегия general (ALT6).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "Вариант ALT6".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "ALT7".into(),
            label: "ALT7".into(),
            description: "Альтернативная стратегия general (ALT7).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "Вариант ALT7".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "ALT8".into(),
            label: "ALT8".into(),
            description: "Альтернативная стратегия general (ALT8).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "Вариант ALT8".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "ALT9".into(),
            label: "ALT9".into(),
            description: "Альтернативная стратегия general (ALT9).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "Вариант ALT9".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "ALT10".into(),
            label: "ALT10".into(),
            description: "Альтернативная стратегия general (ALT10).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "Вариант ALT10".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "ALT11".into(),
            label: "ALT11".into(),
            description: "Альтернативная стратегия general (ALT11).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "Вариант ALT11".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "FAKE_TLS_AUTO".into(),
            label: "FAKE TLS AUTO".into(),
            description: "Стратегия general (FAKE TLS AUTO).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "FAKE TLS AUTO".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "FAKE_TLS_AUTO_ALT".into(),
            label: "FAKE TLS AUTO ALT".into(),
            description: "Стратегия general (FAKE TLS AUTO ALT).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "FAKE TLS AUTO ALT".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "FAKE_TLS_AUTO_ALT2".into(),
            label: "FAKE TLS AUTO ALT2".into(),
            description: "Стратегия general (FAKE TLS AUTO ALT2).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "FAKE TLS AUTO ALT2".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "FAKE_TLS_AUTO_ALT3".into(),
            label: "FAKE TLS AUTO ALT3".into(),
            description: "Стратегия general (FAKE TLS AUTO ALT3).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "FAKE TLS AUTO ALT3".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "SIMPLE_FAKE".into(),
            label: "SIMPLE FAKE".into(),
            description: "Стратегия general (SIMPLE FAKE).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "SIMPLE FAKE".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "SIMPLE_FAKE_ALT".into(),
            label: "SIMPLE FAKE ALT".into(),
            description: "Стратегия general (SIMPLE FAKE ALT).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "SIMPLE FAKE ALT".into(),
            args: vec![],
        },
        ZapretStrategy {
            id: "SIMPLE_FAKE_ALT2".into(),
            label: "SIMPLE FAKE ALT2".into(),
            description: "Стратегия general (SIMPLE FAKE ALT2).bat".into(),
            protocols: "TCP/UDP как в default".into(),
            modes: "SIMPLE FAKE ALT2".into(),
            args: vec![],
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
    lists.list_google = read_list_file(&lists_dir.join("list-google.txt"));
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

fn substitute_strategy_args(
    args: &[String],
    paths: &StrategyPaths,
    game_tcp: &str,
    game_udp: &str,
) -> Vec<String> {
    let bin_suffix = format!("{}\\", paths.bin.display());
    args.iter()
        .map(|arg| {
            let s = arg
                .replace("{list_general}", &paths.list_general.to_string_lossy())
                .replace("{list_general_user}", &paths.list_general_user.to_string_lossy())
                .replace("{list_exclude}", &paths.list_exclude.to_string_lossy())
                .replace("{list_exclude_user}", &paths.list_exclude_user.to_string_lossy())
                .replace("{list_google}", &paths.list_google.to_string_lossy())
                .replace("{ipset_all}", &paths.ipset_all.to_string_lossy())
                .replace("{ipset_exclude}", &paths.ipset_exclude.to_string_lossy())
                .replace("{ipset_exclude_user}", &paths.ipset_exclude_user.to_string_lossy())
                .replace("{bin}", &bin_suffix)
                .replace("{game_tcp}", game_tcp)
                .replace("{game_udp}", game_udp);
            s
        })
        .collect()
}

struct StrategyPaths<'a> {
    bin: &'a PathBuf,
    list_general: &'a PathBuf,
    list_general_user: &'a PathBuf,
    list_exclude: &'a PathBuf,
    list_exclude_user: &'a PathBuf,
    list_google: &'a PathBuf,
    ipset_all: &'a PathBuf,
    ipset_exclude: &'a PathBuf,
    ipset_exclude_user: &'a PathBuf,
}

// Запуск стратегии: strategy_id — id из get_zapret_strategies (или None/auto = default).
#[tauri::command]
fn run_default_strategy(
    app: tauri::AppHandle,
    strategy_id: Option<String>,
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
    let list_google_path =
        write_list_file(&temp_lists_dir, "list-google.txt", &lists_data.list_google)?;
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

    let paths = StrategyPaths {
        bin: &bin,
        list_general: &list_general_path,
        list_general_user: &list_general_user_path,
        list_exclude: &list_exclude_path,
        list_exclude_user: &list_exclude_user_path,
        list_google: &list_google_path,
        ipset_all: &ipset_all_path,
        ipset_exclude: &ipset_exclude_path,
        ipset_exclude_user: &ipset_exclude_user_path,
    };

    let strategies = built_in_strategies();
    let strategy = strategy_id
        .as_ref()
        .filter(|id| !id.is_empty() && id.as_str() != "auto")
        .and_then(|id| strategies.iter().find(|s| s.id == *id))
        .or_else(|| strategies.first());
    let args_template: Vec<String> = strategy
        .map(|s| {
            if s.args.is_empty() {
                default_strategy_args()
            } else {
                s.args.clone()
            }
        })
        .unwrap_or_else(default_strategy_args);
    let args = substitute_strategy_args(&args_template, &paths, &game_tcp, &game_udp);

    let winws = bin.join("winws.exe");
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
