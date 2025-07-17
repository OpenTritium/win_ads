---

# Rust ADS Utilities

[English](#english) | [中文](#中文)

---

<a name="english"></a>

## English

A **humble Rust wrapper** for the Windows API to manipulate **Alternate Data Streams (ADS)**.

This library does not invent anything new. It is merely a basic wrapper around the low-level Win32 API functions provided by the `windows-rs` crate, making them slightly easier to call from Rust.

> ⚠️ **WARNING: HIGH RISK OF DATA LOSS**
>
> Alternate Data Streams (ADS) are a feature of specific Windows file systems like **NTFS and ReFS**.
>
> If a file containing ADS is moved or copied to **any file system that does not support this feature**, all ADS data will be **silently and permanently deleted, with no warning given**.
>
> **Common operations that will almost certainly cause ADS data loss include:**
> *   Copying to a USB flash drive or SD card (usually formatted as **FAT32** or **exFAT**).
> *   Uploading or downloading via a web browser (HTTP).
> *   Putting it into a standard `.zip` archive.
> *   Sending it as an email attachment.
> *   Copying it to a macOS (APFS, HFS+) or Linux (ext4, Btrfs) system.

## How to use

```rust
use win_ads::*;
fn main() {
    const PATH: &'static str = "./test";
    const STREAM: &'static str = "default";
    set_ads(PATH, STREAM, b"some bytes here").unwrap();
    let mut buf = vec![];
    get_ads(PATH, STREAM, &mut buf).unwrap();
    let s = str::from_utf8(buf.as_slice()).unwrap();
    println!("{s}");
    append_ads(PATH, STREAM, b"foo").unwrap();
    get_ads(PATH, STREAM, &mut buf).unwrap();
    let s = str::from_utf8(buf.as_slice()).unwrap();
    println!("{s}");
    delete_ads(PATH, STREAM).unwrap();
}
```

<a name="chinese"></a>

## 中文

一个针对 Windows API 的**简陋 Rust 包装**，用于操作**备用数据流 (Alternate Data Streams)**。

这个库没有创造任何新东西，它仅仅是对 `windows-rs` crate 提供的底层 Win32 API 函数做了一层最基础的封装，使其在 Rust 中稍微易于调用。

> ⚠️ **警告：数据极易丢失**
>
> 备用数据流 (ADS) 是 **NTFS 和 ReFS** 等特定 Windows 文件系统的一个特性。
>
> 如果一个包含 ADS 的文件被移动或复制到**任何不支持此特性的文件系统**上，所有 ADS 数据都将**被自动且永久地删除，并且系统不会给出任何提示**。
>
> **以下常见操作几乎一定会导致 ADS 数据丢失：**
> *   复制到 U 盘或 SD 卡 (通常是 **FAT32** 或 **exFAT** 格式)。
> *   通过网页上传或下载 (HTTP)。
> *   放进标准的 `.zip` 压缩包。
> *   作为电子邮件附件发送。
> *   拷贝到 macOS (APFS, HFS+) 或 Linux (ext4, Btrfs) 系统。


## 如何使用

```rust
use win_ads::*;
fn main() {
    const PATH: &'static str = "./test";
    const STREAM: &'static str = "default";
    set_ads(PATH, STREAM, b"some bytes here").unwrap();
    let mut buf = vec![];
    get_ads(PATH, STREAM, &mut buf).unwrap();
    let s = str::from_utf8(buf.as_slice()).unwrap();
    println!("{s}");
    append_ads(PATH, STREAM, b"foo").unwrap();
    get_ads(PATH, STREAM, &mut buf).unwrap();
    let s = str::from_utf8(buf.as_slice()).unwrap();
    println!("{s}");
    delete_ads(PATH, STREAM).unwrap();
}
```