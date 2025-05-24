# EEPROMマップ定義ツール

YAMLファイルでEEPROMマップを定義し、対応するC言語の構造体 (`.h`) を生成するツールです。

## 📦 使用方法

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

#### ▶ プリミティブ型

```yaml
- name: device_id
  offset: 0
  type: uint16
```

#### ▶ ネスト構造体（inline）

```yaml
- name: settings
  offset: 10
  type:
    struct:
      - name: brightness
        offset: 0
        type: uint8
      - name: volume
        offset: 1
        type: uint8
```

#### ▶ 事前定義構造体（custom）

```yaml
types:
  settings:
    - name: brightness
      offset: 0
      type: uint8
    - name: volume
      offset: 1
      type: uint8

entries:
  - name: settings
    offset: 10
    type: !custom settings
```

※ YAMLタグ `!custom` により `settings` を事前定義型として展開

## 🗂 出力例（eeprom\_map.h）

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
