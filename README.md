Rust Zombie Shooter

这是一个用 Rust 实现的简单命令行僵尸射击小游戏（单文件实现）。

构建与运行（如果你已安装 Rust toolchain）：

```powershell
cd c:\cangjie\code\rust_zombie_shooter
cargo run --release
```

如果你没有安装 Rust，请访问 https://rustup.rs/ 安装工具链，然后运行上述命令。

游戏控制：
- s = shoot
- r = reload
- q = quit

可选资源 (sprites)
---------------------

本项目支持使用外部图片作为玩家和僵尸贴图。

将图片放到项目根目录下的 `assets/` 文件夹中：

- `assets/player.png` — 推荐大小约 64x64，会以图片中心对齐玩家位置。
- `assets/zombie.png` — 推荐大小约 40x40 或 64x64，取决于风格。

若文件不存在，游戏会退回到简单形状（玩家为圆，僵尸为矩形）。

运行：

```powershell
cargo run --release
```
