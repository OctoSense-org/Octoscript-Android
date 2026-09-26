# Octoscript Catalog——由 Octoscript DSL 驱动的 Material Components Android catalog

[English](README.md) | 简体中文

复刻了 [material-components-android](https://github.com/material-components/material-components-android)
的 catalog 应用，其中**每个页面都用 Octoscript DSL 编写，在设备上由 makepad-script VM
求值，并渲染成真正的 `com.google.android.material.*` View。**

没有 makepad 渲染器，没有 GL surface，也没有 `Octoscript` 控件。进程里唯一的
makepad 代码是语言 VM。

```
42 .octoscript screens
   │
   ▼  makepad-script VM (via octoscript-render's re-export)   ── Rust
generic node tree  (kind + attr bag + children)
   │
   ▼  flat binary buffer, one direct ByteBuffer
   │  ── ONE JNI crossing per render ──
   ▼
Java builder → MaterialButton / Chip / TextInputLayout / Slider / …   ── Java owns every View
```

## 构建

```sh
cd rust && cargo build --release --target aarch64-linux-android   # needs the NDK env
cp target/aarch64-linux-android/release/liboctoscript_catalog.so ../app/src/main/jniLibs/arm64-v8a/
cd .. && gradle assembleDebug && adb install -r app/build/outputs/apk/debug/app-debug.apk
```

通过深链接直接打开任意页面：`adb shell am start -n dev.octoscript.catalog/.MainActivity --es route button`

不需要设备、在宿主机上检查所有路由：`cd rust && cargo run --release --example probe`

## 覆盖范围——41 个页面

全部 41 个路由都已在真机上验证（OnePlus 6T，Android 11 / SDK 30，Material 1.13.0）：
能渲染、没有占位、没有异常。每个页面 22–99 个节点。

`allcomponents` `adaptive` `badge` `bottomappbar` `bottomnav` `bottomsheet`
`button` `card` `carousel` `checkbox` `chip` `color` `datepicker` `dialog`
`divider` `dockedtoolbar` `elevation` `fab` `floatingtoolbar` `font` `imageview`
`listitem` `loadingindicator` `materialswitch` `menu` `musicplayer`
`navigationdrawer` `navigationrail` `preferences` `progressindicator`
`radiobutton` `search` `shapetheming` `sidesheet` `slider` `snackbar` `tabs`
`textfield` `timepicker` `topappbar` `transition`

### 已在真机上验证的交互

| 流程 | 证据 |
|---|---|
| `MaterialAlertDialogBuilder`——alert / icon / single / multi / long / full-screen | 对话框以 M3 形状和遮罩渲染 |
| 带操作按钮的 `Snackbar` | 无障碍树中出现 "Message archived" + "Undo" |
| `MaterialDatePicker`——calendar / range / input | 完整日历、日期正确、Cancel/OK |
| `MaterialTimePicker`——12h / 24h / keyboard | "Select time" |
| `BottomSheetDialog`——modal / list / tall | 圆角底部面板 + 列表行 |
| `SideSheetDialog`——left / right / detached | "Side sheet" |
| `PopupMenu` | "Refresh" |
| `DrawerLayout` + `NavigationView` | Inbox / Starred / Settings |
| **状态往返** | 点标签页 → `Content for tab 0` → `Content for tab 1`；拖动滑块 → `Value: 50` → `Value: 97` |

真正要紧的是状态往返：Java 控件事件写入 Rust 状态，Rust **在 VM 中重新对 DSL 求值**，
新的树再重建 View。决定屏幕上显示什么的是 DSL，而不是 Java。

## 发现的 VM 限制（来之不易的部分）

`octoscript-render` 锁定的 makepad-script 版本 `e1c2164b` 有三种写法会悄无声息地
生成错误的树，而不是报错。三者都是先表现为空白页面，再通过宿主机侧探测
（`examples/probe.rs`）找出来的：

| 写法 | 结果 | 替代做法 |
|---|---|---|
| 以顶层**函数调用**作为模块结果——`page([...])` | 根节点求值后没有 `t` 标签 | 以**字面量对象**结尾 |
| `let k = [ {…}, {…} ]` 然后 `c: k` | 数组到达时是**空的**——子节点被悄悄丢弃 | 把数组内联，或 `let k = []` + `k.push(…)` |
| 对普通对象取 `st.missing_key` | VM 硬错误，整个求值失败 | 用返回默认值的宿主函数——见 `S()` / `N()` |

`S(key)` / `N(key, default)` 作为 VM 全局变量注入（`set_injected_global`），
与 Octoscript-OH 注入其网络辅助函数的方式完全相同。这也是状态读取不会失败的原因：
缺失的键返回 `""` 或默认值，而不会让求值中断。

**返回对象**的辅助函数没有问题——`section()`、`caption()`、`group()` 都是如此。
只有上面三种写法不安全。

## 设计说明

- **Java 持有 View；Rust 持有 id 和状态。** Rust 里从不保存任何 `jobject`，
  所以 512 个局部引用上限导致的 abort 和 `FindClass` 的 classloader 陷阱
  从结构上就不可能触发。
- **每次渲染只跨一次 JNI。** 整棵树以扁平缓冲区的形式装在一个 direct `ByteBuffer`
  中传递；字符串放在一块附属数据区里，按 (offset, len) 寻址。
- **通用属性包，而非固定结构体。** `octoscript-render` 的 `Attrs` 大约有 30 个固定字段；
  43 个 Material 组件需要的远不止这些，所以这里的遍历器携带
  `Vec<(String, Val)>`，并对照一份显式声明的约 56 个名称的词表。LiveId 键是
  单向哈希，所以词表必须声明，无法反推。
- **用 Gradle，而不是 cargo-makepad。** 正是 Gradle 才让 androidx 和 Material
  可用——`cargo_makepad/src/android/compile.rs` 中 `-classpath android.jar`
  的限制是那条构建路径本身的特性，而不是 Android 的限制。
- **图标**是 catalog 自带的 112 个矢量 drawable，取自 MDC 仓库。
  一张小型别名表把 DSL 中的名称映射到最接近的现有图标，用于图标集里没有完全匹配的情况。

## 动效、旋转、图标——全部接通

**Material 动效是真正在运行的**，通过 `androidx.transition.TransitionManager`
作用于 View 层级来驱动（不需要 fragment 回退栈）：

| 过渡 | 验证结果 |
|---|---|
| `MaterialContainerTransform` | 卡片 → 详情面：`Tap to expand` → `Expanded`，宿主高度随变换从 96dp 动画到 240dp |
| `MaterialSharedAxis` X / Y / Z | `Pane 1` → `Pane 2` |
| `MaterialFadeThrough` / `MaterialFade` | `Pane 2` → `Pane 3` |

**旋转会重新渲染。** Activity 自己处理 `orientation|screenSize|…`，
把窗口尺寸等级重新写入 Rust 状态，并重新对 DSL 求值：
旋转时 `Compact (384dp)` → `Medium (803dp)`。自适应布局示例也随之重新布局。

**Carousel** 使用 `CarouselSnapHelper`；`fullscreen` 策略以页面大小的条目
纵向翻页。

**图标都是真正的矢量图，没有近似替代。** 共 122 个 drawable：MDC
catalog 自带的一套，加上此处为其缺少的图标绘制的 Material Symbols（`share`、`shopping_cart`、
`notifications`、`more_vert`、`shuffle`、`repeat`、`place`、`call`、`album`、`chat`、
`bookmark`、`payments`）。别名表现在只保留真正的同义词（`mail`→`mail_outline`、`person`→
`account_circle`，……），不再有替身。

## 靠看真机发现的 bug

值得记录，因为它们都没有以错误的形式出现：

- **`PaintDrawable` + `ShaderFactory` 从未绘制出来**，放在 `CENTER_CROP` 的
  `ImageView` 里时没有固有尺寸。carousel 画出的是空卡片。改用
  `GradientDrawable` 修复。
- **`spacer` 在两个轴上都占了权重**，导致音乐播放器的时间行长到
  682px，把播放控制按钮挤没了。现在 spacer 只沿父容器的方向占权重。
- **父容器的 `addChildren` 覆盖了宿主自身的高度**，导致
  container-transform 的宿主一直是 220dp，折叠后的卡片悬在一片空白里。
  现在由 DSL 声明折叠（`h`）和展开（`max`）两种高度。
