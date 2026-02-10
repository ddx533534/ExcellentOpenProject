# ExcellentOpenProject

## 1.[resvg](https://github.com/linebender/resvg)
### 1.核心定位：一个快速、小巧、可移植的 SVG 渲染库，目标是支持完整的 SVG 规范。

### 2.主要特性
- 为边缘情况设计。<br/>
   测试套件独立开放，可供其他 SVG 库使用;比浏览器更正确地处理静态 SVG（浏览器侧重动态特性）。
- 安全性。完全用 Rust 编写，二进制文件中无非 Rust 代码，几乎无 unsafe 代码；安全处理多种格式：SVG/XML、CSS、TTF、PNG、JPEG、GIF、GZIP。
-  零膨胀 📦<br/>
CLI 应用小于 3MB，无外部依赖，仅包含 SVG 渲染所需代码。
- 可移植性 🌍<br/>
   支持所有 Rust 可编译的平台，包括 WASM；在不同架构上表现一致。
- SVG 预处理 🔄<br/>
   解析和渲染完全分离：usvg：SVG 解析库 ；resvg：渲染库。可基于 usvg 使用任何 2D 库自定义渲染器。
- 性能 ⚡<br/>
   用 Rust 编写 ，使用 tiny-skia 进行渲染 ；速度快，但仍有优化空间。
- 可重现性 🔁<br/>
   **跨平台像素级一致的渲染结果。Windows x86 和 macOS ARM 渲染结果完全相同，**

### 3.库内容
* resvg – 实际的 SVG 渲染器，负责渲染 SVG 图像。
* usvg – SVG 预处理器和简化器，优化 SVG 文件以便渲染。
* tiny-skia – 一个将 Skia（图形库）子集移植到 Rust 的库，用于渲染矢量图形。
* rustybuzz – 将 HarfBuzz（文本排版引擎）移植到 Rust 的子集，负责文本布局。
* ttf-parser – 用于解析 TrueType 和 OpenType 字体的库。
* fontdb – 一个简单的内存中字体数据库，支持类似 CSS 的查询来查找字体。
* roxmltree – 一个用于处理 SVG XML 数据的 XML 解析库。
* simplecss – 一个 CSS 2 解析器，处理 CSS 样式规则和选择器。
* pico-args – 一个极简的命令行参数解析库，广受欢迎。

**虽然 resvg 本身非常简洁，但依赖的这些支持库使它能够处理字体渲染、CSS 解析、SVG 优化等多种任务。即便整体代码量较大，它仍然被认为是目前最小且高效的 SVG 渲染库之一。**
[使用示例](./rust/tests/resvg_test.rs)