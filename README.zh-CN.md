# Octoscript-Android

[English](README.md) | 简体中文

## 共享的 Octoscript-Makepad 运行时

`native-runtime.lock.json` 选定一个
[Octoscript-Makepad](https://github.com/OctoSense-org/Octoscript-Makepad)
发布版本。该版本的 `runtime.json` 负责锁定确切的 Makepad 和 Octoscript 版本，
AppCards、Mail 以及其他 OctoSense 应用共用同一份。

构建之前先运行 `python3 tools/setup-native.py`（需要 Python 3.9+）。框架仓库与本应用
并列存放：`../octoscript-makepad`、`../makepad` 和 `../octoscript`。本地改动会被保留；
`--update` 只更新没有改动的 checkout。CI 会校验所选的发布版本，并拒绝重复的 Makepad 来源。
用 `python3 tools/setup-native.py --check --cargo-manifest catalog/rust/Cargo.toml`
检查本地依赖图。现有的各平台渲染后端仍然属于各自的应用；框架负责的是共享的 VM 和 UI 源码。


用 Rust 把 Octoscript DSL 渲染成 **Android 原生控件**——它是
[Octoscript-OH](https://github.com/OctoSense-org/Octoscript-OH/blob/master/README.zh-CN.md)
在 Android 上的对应项目，后者面向 OpenHarmony 的 ArkUI 做同样的事。

```
probe/      the feasibility probe   — octoscript-render -> android.widget.*, framework widgets only
catalog/    the Material catalog    — 42 screens of Octoscript DSL -> com.google.android.material.*
```

即：`probe/` 是可行性验证，只用框架自带控件；`catalog/` 是 Material 组件目录，
42 个 Octoscript DSL 页面渲染为 `com.google.android.material.*`。

两者都在真机上运行（OnePlus 6T，Android 11 / SDK 30）。

## 整体形态

```
.octoscript  ──►  makepad-script VM  ──►  node tree  ──►  flat buffer  ──►  Java builder  ──►  Views
              (via octoscript-render;        (Rust)      ONE JNI call      (owns every View)
               no makepad renderer)
```

**Java 持有每一个 `View`；Rust 只持有整数 id 和卡片状态。** Rust 里从不保存任何
`jobject`，所以 ART 的 512 个局部引用上限导致的 abort，以及 `FindClass`
的 classloader 陷阱，从结构上就不可能触发。

进程里没有任何 makepad 渲染器——没有 `makepad-platform`，没有 `makepad-draw`，
没有 `makepad-widgets`，也没有 GL surface。唯一链接进来的 makepad 代码是
`makepad-script`，也就是语言 VM，它自身的依赖只有
`error_log`、`math`、`live_id`、`script-derive`、`smallvec`、`regex`、`html`。

## 为什么 Android 版不是 Octoscript-OH 的移植

OpenHarmony 提供 `arkui/native_node.h`——一套用于构建控件的 C NDK。
**Android 没有对应物**：NDK 的 `android/` 目录下有 62 个头文件，没有一个是控件 API。
每个 `android.widget.*` 对象都必须通过 JNI 进入 ART 来构造，而且与 ArkUI 不同，
托管对象之下没有原生层。因此 Octoscript-OH 那 2.5–3 倍的构建速度优势无法照搬过来；
这里的设计目标是*尽量减少跨边界调用*，而不是避开托管语言对象。

完整分析见 octos-one 的 `docs/`（`OCTOSCRIPT-ANDROID-NATIVE-WIDGETS.md`）。

## catalog/

复刻了
[material-components-android](https://github.com/material-components/material-components-android)
的 catalog：**42 个页面**，每一个都用 Octoscript DSL 编写，并在真机上求值。
其中包括真正的 `MaterialAlertDialogBuilder` / `Snackbar` /
`MaterialDatePicker` / `MaterialTimePicker` / `BottomSheetDialog` /
`SideSheetDialog` / `PopupMenu` / `DrawerLayout`，一个真正的 Carousel，以及实时运行的
Material 动效（`MaterialContainerTransform`、`MaterialSharedAxis`、
`MaterialFadeThrough`）。

它还包含 **octos-one 自己的 makepad 控件移植成的 Android View**——
`WeatherIconView`、`NavMapView`、`GlassPanelView`——也就是此前被认为在
`android.widget` 里没有对应物、因而放弃的那几个。

见 [`catalog/README.zh-CN.md`](catalog/README.zh-CN.md)。

## 构建

两者都需要 Android NDK 和 JDK。catalog 用 Gradle（正是 Gradle 才让 androidx 和
Material 可用）；probe 则直接用 `aapt2`/`javac`/`d8` 手工打出 APK。

```sh
# catalog
cd catalog/rust && cargo build --release --target aarch64-linux-android
cp target/aarch64-linux-android/release/liboctoscript_catalog.so ../app/src/main/jniLibs/arm64-v8a/
cd .. && gradle assembleDebug && adb install -r app/build/outputs/apk/debug/app-debug.apk

# probe
cd probe && ./build.sh
```

`catalog/rust` 还有一个在宿主机上运行、不需要设备的检查：

```sh
cd catalog/rust && cargo run --release --example probe   # evaluates all 42 routes
```

## 状态

- ✅ 可行性验证——框架控件、真正的输入法（IME）、完整的无障碍树
- ✅ Material catalog——42 个页面，真机上 0 个占位、0 个异常
- ✅ octos-one 控件移植——WeatherIcon（8 种天气状况）、MapView（3 种导航模式）、玻璃面板
- ⏳ `UiNode` 增量/事件契约——构建已完成，增量更新尚未完成
- ⏳ 向 `octoscript-render` 上游提交：`Native`/`Custom` 节点类型、用于协调（reconciliation）的
  `key` 属性，以及 `Serialize` derive

## 许可证

Apache-2.0（见 [LICENSE](LICENSE) 和 [NOTICE](NOTICE)）。取自
material-components-android catalog 的 drawable 保留其原有的 Android Open Source Project
版权声明（同样是 Apache-2.0）。
