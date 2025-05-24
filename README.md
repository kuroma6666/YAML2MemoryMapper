# EEPROMマップ定義ツール

YAMLファイルでEEPROMマップを定義し、対応するC言語の構造体 (`.h`) を生成するツールです。

## 💼 使用方法

```powershell
cargo run -- examples\eeprom.yaml
```

生成結果は `eeprom_map.h` に出力されます。

## 📝 YAML書式ガイド

### 🔹 必須項目

| フィールド名         | 内容                 |
| -------------- | ------------------ |
| `version`      | フォーマットのバージョン       |
| `base_address` | EEPROMのベースアドレス     |
| `endianness`   | `little` または `big` |
| `entries`      | メモリマップ定義           |
| `types`        | ユーザー定義型（任意）        |

### 🔹 型定義の方法

#### ✅ サポートされるプリミティブ型

| 型名       | 説明          |
| -------- | ----------- |
| `uint8`  | 8ビット符号なし整数  |
| `uint16` | 16ビット符号なし整数 |
| `uint32` | 32ビット符号なし整数 |

本ツールは、型を自動で判別・補正する機能を備えています。
そのため `type: uint8` のように文字列で指定した場合も、プリミティブ型やユーザー定義型として自動で解釈されます。

次のYAMLファイルがそれぞれの型定義方法のサンプルとして提供されています：

* [primitive.yaml](examples/primitive.yaml)：プリミティブ型の定義例
* [nested\_struct.yaml](examples/nested_struct.yaml)：ネスト構造体（inline定義）
* [custom\_autodetect.yaml](examples/custom_autodetect.yaml)：カスタム型（補正対応）の定義例

各形式の詳細はこれらのファイルをご参照ください。

## 📂 出力例（eeprom\_map.h）

```c
typedef struct {
    uint8_t brightness;
    uint8_t volume;
} settings_t;

typedef struct {
    uint16_t device_id;
    uint8_t firmware_version;
    settings_t settings;
} eeprom_map_t;
```
