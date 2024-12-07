# コマンドライン引数のパーサーサンプル
```cmd
cargo run -- firstinput --name "John" --debug --list item1 item2
```

## clapのArgAction
- `ArgAction::SetTrue` フラグをセット
- `ArgAction::SetFalse` フラグをリセット
- `ArgAction::Set` 値をセット
- `ArgAction::Append` 値を追加
- `ArgAction::Count` カウント