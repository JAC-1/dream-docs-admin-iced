# Dream Docs — Admin (Iced)

The administrator-facing native desktop application for a secure, end-to-end encrypted document management system built for a Japanese study abroad program.

## Overview

Dream Docs is the first generation of the document delivery platform. This repository is the admin-side application — a native desktop app written in Rust using the [Iced](https://github.com/iced-rs/iced) GUI crate, used by administrators to review, manage, and download student document submissions. The companion student-facing application is a separate Next.js web app where students authenticate and securely upload documents.

Administrators use this tool to log in with a master password, browse enrolled students, view their document submissions by type, and download decrypted files to a local folder. All documents are encrypted end-to-end by the client before upload — this application holds the RSA private key needed to unwrap each document's symmetric key and recover the plaintext file.

## Features

- **Password-protected login** — master password decrypts embedded AES-256-GCM-encrypted environment files to load database credentials at runtime
- **Student roster** — view all enrolled students with their class and program assignments
- **Per-student document view** — browse all submitted documents for a student with live status indicators (New / Pending / Approved / Declined)
- **Document status management** — approve, decline, or mark documents as pending directly from the document table
- **On-demand decryption and download** — fetch encrypted files from Turso, unwrap the per-file symmetric key using the RSA private key, decrypt with AES-256-GCM, and save to a user-selected folder
- **Batch download** — download all documents for a student at once; deduplicates by document type and keeps the most recent submission
- **Structured file naming** — downloaded files are named by student and document type for easy organisation
- **Japanese font support** — Noto Sans JP embedded for rendering student names and program data in Japanese

## Tech Stack

| Layer                  | Technology                          |
| ---------------------- | ----------------------------------- |
| Language               | Rust (Edition 2021)                 |
| GUI Framework          | Iced v0.13                          |
| Asymmetric Encryption  | OpenSSL — RSA-OAEP with SHA-256     |
| Symmetric Encryption   | Ring — AES-256-GCM                  |
| Student & Metadata DB  | Supabase (PostgreSQL via PostgREST) |
| Encrypted File Storage | Turso (LibSQL / SQLite)             |
| Async Runtime          | Tokio                               |
| File Dialogs           | rfd (Rust File Dialog)              |
| Serialisation          | serde + serde_json                  |
| Date/Time              | chrono                              |
| Error Handling         | anyhow                              |
| Font                   | Noto Sans JP (embedded)             |

## Decryption Model

1. Admin enters the master password at login
2. The password is used with PBKDF2 (100,000 iterations) to derive a key that decrypts the embedded encrypted `.env` file (AES-256-GCM), loading Supabase and Turso credentials into memory
3. When a document is downloaded, its RSA-OAEP-wrapped symmetric key is fetched from Supabase (`file_keys` table)
4. The embedded RSA private key unwraps the symmetric key
5. The encrypted file blob is fetched from Turso, and the symmetric key decrypts it with AES-256-GCM (IV and auth tag embedded in the blob)
6. The plaintext file is written to the admin's chosen local folder

## Project Structure

```
src/
  main.rs                       # Application entry point and top-level state machine
  components/
    navbar.rs                   # Navigation bar
    login.rs                    # Login form
    views.rs                    # View routing
    students_table.rs           # Student roster table
    student_profile_info.rs     # Student profile panel
    document_table.rs           # Per-student document list
    custom_button_example.rs    # Button style examples
  models/
    supabase_models/            # Structs for Supabase table rows
      student.rs
      student_profile_data.rs
      file.rs                   # Document metadata
      file_key.rs               # Encrypted key record
      class.rs
      program.rs
    turso_models/
      file.rs                   # Encrypted file blob record
  operations/
    supabase_opp.rs             # Supabase REST queries
    turso_opp.rs                # Turso database operations
    secret_opp.rs               # RSA + AES decryption logic
    login_opp.rs                # Password auth and env file decryption
    file_opp.rs                 # File save and naming logic
  types/
    file_status.rs              # FileStatus enum (New, Pending, Approved, Declined)
    task_types.rs               # TaskType enum (document categories)
    active_status.rs            # User active status
  styles/
    button_styles.rs            # Iced button style definitions
  custom_settings/
    window_settings.rs          # Window size and title config
  fonts/
    NotoSansJP-Regular.ttf      # Embedded Japanese font
    NotoSansJP-Bold.ttf
```

## Screenshots

Screenshots coming soon.

## Context

This admin application was built as the counterpart to the Dream Docs student web client — a Next.js app where Japanese high school students submit encrypted documents as part of a Canadian study abroad application process. The split architecture (Next.js web client + Rust native admin) later motivated a full rewrite into dd-mk2, which consolidates everything into a single Next.js application.

---

## 日本語 / Japanese

# Dream Docs — 管理者アプリ（Iced）

カナダ留学プログラム向けの、安全なエンドツーエンド暗号化ドキュメント管理システムの管理者向けネイティブデスクトップアプリケーションです。

## 概要

Dream Docs はドキュメント配信プラットフォームの第1世代です。このリポジトリは管理者向けアプリケーションであり、Rust と [Iced](https://github.com/iced-rs/iced) GUI クレートを使用して構築されたネイティブデスクトップアプリです。管理者はこのアプリを使って、学生の提出書類を確認・管理・ダウンロードします。対応する学生向けアプリケーションは別途 Next.js で構築された Web アプリで、学生が認証・書類の安全なアップロードを行います。

管理者はマスターパスワードでログインし、在籍学生の一覧を閲覧し、学生ごとの提出書類をタイプ別に確認して、復号したファイルをローカルフォルダにダウンロードできます。すべての書類はアップロード前にクライアント側で暗号化されており、このアプリケーションが各ファイルの対称鍵をアンラップして平文を復元するための RSA 秘密鍵を保持しています。

## 主な機能

- **パスワード保護ログイン** — マスターパスワードで AES-256-GCM 暗号化された環境ファイルを復号し、データベース認証情報をメモリに展開
- **学生名簿** — クラスおよびプログラム情報付きで在籍学生を一覧表示
- **学生別書類一覧** — 提出書類をタイプ別に表示し、ステータス（新規 / 審査中 / 承認済 / 却下）をリアルタイムで確認
- **書類ステータス管理** — 書類テーブルから直接承認・却下・審査中への変更が可能
- **オンデマンド復号・ダウンロード** — Turso から暗号化ファイルを取得し、RSA 秘密鍵でファイル単位の対称鍵をアンラップ、AES-256-GCM で復号してフォルダに保存
- **一括ダウンロード** — 学生のすべての書類を一括ダウンロード（書類タイプで重複排除、最新のみ保持）
- **構造化されたファイル名** — 学生名と書類タイプに基づいたファイル名で保存
- **日本語フォント対応** — Noto Sans JP を埋め込み、学生名やプログラム情報を日本語で表示

## 技術スタック

| レイヤー                 | 技術                               |
| ------------------------ | ---------------------------------- |
| 言語                     | Rust（Edition 2021）               |
| GUI フレームワーク       | Iced v0.13                         |
| 非対称暗号化             | OpenSSL — RSA-OAEP（SHA-256）      |
| 対称暗号化               | Ring — AES-256-GCM                 |
| 学生・メタデータ DB      | Supabase（PostgreSQL / PostgREST） |
| 暗号化ファイルストレージ | Turso（LibSQL / SQLite）           |
| 非同期ランタイム         | Tokio                              |
| ファイルダイアログ       | rfd（Rust File Dialog）            |
| シリアライズ             | serde + serde_json                 |
| 日付・時刻               | chrono                             |
| エラーハンドリング       | anyhow                             |
| フォント                 | Noto Sans JP（埋め込み）           |

## 復号モデル

1. 管理者がログイン時にマスターパスワードを入力
2. パスワードは PBKDF2（100,000 イテレーション）で鍵導出され、埋め込まれた暗号化 `.env` ファイル（AES-256-GCM）を復号して Supabase・Turso の認証情報をメモリに展開
3. 書類のダウンロード時、RSA-OAEP でラッピングされた対称鍵を Supabase（`file_keys` テーブル）から取得
4. 埋め込まれた RSA 秘密鍵で対称鍵をアンラップ
5. 暗号化ファイルの blob を Turso から取得し、対称鍵で AES-256-GCM 復号（IV と認証タグは blob に埋め込まれている）
6. 平文ファイルを管理者が選択したローカルフォルダに保存

## プロジェクト構成

```
src/
  main.rs                       # アプリエントリーポイントとトップレベルのステートマシン
  components/
    navbar.rs                   # ナビゲーションバー
    login.rs                    # ログインフォーム
    views.rs                    # ビュールーティング
    students_table.rs           # 学生名簿テーブル
    student_profile_info.rs     # 学生プロファイルパネル
    document_table.rs           # 学生別書類一覧
    custom_button_example.rs    # ボタンスタイル例
  models/
    supabase_models/            # Supabase テーブル行の構造体
      student.rs
      student_profile_data.rs
      file.rs                   # 書類メタデータ
      file_key.rs               # 暗号化鍵レコード
      class.rs
      program.rs
    turso_models/
      file.rs                   # 暗号化ファイル blob レコード
  operations/
    supabase_opp.rs             # Supabase REST クエリ
    turso_opp.rs                # Turso データベース操作
    secret_opp.rs               # RSA + AES 復号ロジック
    login_opp.rs                # パスワード認証・環境ファイル復号
    file_opp.rs                 # ファイル保存・命名ロジック
  types/
    file_status.rs              # FileStatus 列挙型（New, Pending, Approved, Declined）
    task_types.rs               # TaskType 列挙型（書類カテゴリ）
    active_status.rs            # ユーザーアクティブステータス
  styles/
    button_styles.rs            # Iced ボタンスタイル定義
  custom_settings/
    window_settings.rs          # ウィンドウサイズ・タイトル設定
  fonts/
    NotoSansJP-Regular.ttf      # 埋め込み日本語フォント
    NotoSansJP-Bold.ttf
```

## スクリーンショット

スクリーンショットは近日公開予定です。

## 背景

この管理者アプリケーションは、Dream Docs 学生向け Web クライアントの対となるアプリとして開発されました。学生向けクライアントは Next.js 製の Web アプリで、日本人高校生がカナダ留学プログラムへの申請書類を暗号化してアップロードします。この分割アーキテクチャ（Next.js Web クライアント + Rust ネイティブ管理アプリ）が、後に dd-mk2 への完全リライトのきっかけとなりました。dd-mk2 ではすべてを単一の Next.js アプリケーションに統合しています。
