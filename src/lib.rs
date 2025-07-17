#[cfg(not(windows))]
compile_error!("This crate is designed to work only on the Windows operating system.");

use windows::{
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE},
        Storage::FileSystem::{
            CREATE_ALWAYS, CreateFileW, DeleteFileW, FILE_APPEND_DATA, FILE_ATTRIBUTE_NORMAL,
            FILE_SHARE_NONE, FILE_SHARE_READ, GetFileSizeEx, OPEN_ALWAYS, OPEN_EXISTING, ReadFile,
            WriteFile,
        },
    },
    core::{PCWSTR, Result},
};
use windows_strings::HSTRING;

/// create or overwrite an ads
#[inline]
pub fn set_ads(path: &str, name: &str, src: impl AsRef<[u8]>) -> Result<u32> {
    let stream_path = HSTRING::from(format!("{path}:{name}"));
    let mut written: u32 = 0;
    unsafe {
        let handle = CreateFileW(
            PCWSTR::from_raw(stream_path.as_ptr()),
            GENERIC_WRITE.0,
            FILE_SHARE_NONE,
            None,
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )?;
        WriteFile(handle, Some(src.as_ref()), Some(&mut written), None)?;
        CloseHandle(handle)?;
    }
    Ok(written)
}

/// read an ads
#[inline]
pub fn get_ads(path: &str, name: &str, dst: &mut Vec<u8>) -> Result<()> {
    let stream_path = HSTRING::from(format!("{path}:{name}"));
    unsafe {
        let handle = CreateFileW(
            PCWSTR::from_raw(stream_path.as_ptr()),
            GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )?;
        let mut file_size = 0i64;
        GetFileSizeEx(handle, &mut file_size)?;
        dst.resize(file_size as usize, 0);
        let mut read = 0u32;
        ReadFile(handle, Some(dst.as_mut()), Some(&mut read), None)?;
        CloseHandle(handle)?;
        dst.set_len(read as usize);
        Ok(())
    }
}

/// append an ads
#[inline]
pub fn append_ads(path: &str, name: &str, src: impl AsRef<[u8]>) -> Result<u32> {
    let stream_path = HSTRING::from(format!("{path}:{name}"));
    let mut written = 0;
    unsafe {
        let handle = CreateFileW(
            PCWSTR::from_raw(stream_path.as_ptr()),
            FILE_APPEND_DATA.0,
            FILE_SHARE_NONE,
            None,
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )?;
        WriteFile(handle, Some(src.as_ref()), Some(&mut written), None)?;
        CloseHandle(handle)?;
    }
    Ok(written)
}

/// delete a ads
#[inline]
pub fn delete_ads(path: &str, name: &str) -> Result<()> {
    let stream_path = HSTRING::from(format!("{path}:{name}"));
    unsafe {
        DeleteFileW(PCWSTR::from_raw(stream_path.as_ptr()))?;
    }
    Ok(())
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_set_and_get_ads() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap();
        let stream_name = "test_stream";
        let content = b"hello world";

        // 测试 set_ads
        let written = set_ads(path, stream_name, content).unwrap();
        assert_eq!(written, content.len() as u32);

        // 测试 get_ads
        let mut read_content = Vec::new();
        get_ads(path, stream_name, &mut read_content).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_append_ads() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap();
        let stream_name = "append_stream";
        let initial_content = b"initial data. ";
        let appended_content = b"appended data.";

        // 初始写入
        set_ads(path, stream_name, initial_content).unwrap();

        // 追加数据
        let appended_written = append_ads(path, stream_name, appended_content).unwrap();
        assert_eq!(appended_written, appended_content.len() as u32);

        // 验证追加后的内容
        let mut final_content = Vec::new();
        get_ads(path, stream_name, &mut final_content).unwrap();
        let mut expected_content = Vec::new();
        expected_content.extend_from_slice(initial_content);
        expected_content.extend_from_slice(appended_content);
        assert_eq!(final_content, expected_content);
    }

    #[test]
    fn test_delete_ads() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap();
        let stream_name = "delete_stream";
        let content = b"some data";

        // 先创建 ADS
        set_ads(path, stream_name, content).unwrap();

        // 确认 ADS 存在
        let mut read_content = Vec::new();
        assert!(get_ads(path, stream_name, &mut read_content).is_ok());

        // 删除 ADS
        delete_ads(path, stream_name).unwrap();

        // 确认 ADS 已被删除 (再次读取应该会失败)
        assert!(get_ads(path, stream_name, &mut read_content).is_err());
    }
}
