use std::path::Path;

/// 清理超过保留天数的旧文件
///
/// 返回删除的文件数量。
pub fn cleanup_old_files(directory: &Path, retention_days: u32) -> Result<usize, String> {
    if !directory.exists() {
        return Ok(0);
    }

    let cutoff = chrono::Local::now() - chrono::Duration::days(retention_days as i64);
    let mut deleted_count = 0usize;

    let entries = std::fs::read_dir(directory).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // 从文件修改时间判断
        if let Ok(metadata) = path.metadata() {
            if let Ok(modified) = metadata.modified() {
                let modified_time = chrono::DateTime::<chrono::Local>::from(modified);
                if modified_time < cutoff {
                    if let Err(e) = std::fs::remove_file(&path) {
                        clash_verge_logging::logging!(
                            error,
                            clash_verge_logging::Type::Core,
                            "SNC: failed to delete old file {}: {}",
                            path.display(),
                            e
                        );
                    } else {
                        deleted_count += 1;
                    }
                }
            }
        }
    }

    Ok(deleted_count)
}

/// 按日期清理指定日期的历史文件
pub fn cleanup_by_date(directory: &Path, date: &str) -> Result<(), String> {
    if !directory.exists() {
        return Ok(());
    }

    let entries = std::fs::read_dir(directory).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.contains(date) {
                if let Err(e) = std::fs::remove_file(&path) {
                    clash_verge_logging::logging!(
                        error,
                        clash_verge_logging::Type::Core,
                        "SNC: failed to delete file {}: {}",
                        path.display(),
                        e
                    );
                }
            }
        }
    }

    Ok(())
}
