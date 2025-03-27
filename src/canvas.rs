use std::cmp::max;
use std::iter;

use crate::cast::As;
use crate::types::{Color, EcLevel, Version};

static ALL_PATTERNS_QR: [MaskPattern; 8] = [
    MaskPattern::Checkerboard,
    MaskPattern::HorizontalLines,
    MaskPattern::VerticalLines,
    MaskPattern::DiagonalLines,
    MaskPattern::LargeCheckerboard,
    MaskPattern::Fields,
    MaskPattern::Diamonds,
    MaskPattern::Meadow,
];


static FORMAT_INFOS_QR: [u16; 32] = [
    0x5412, 0x5125, 0x5e7c, 0x5b4b, 0x45f9, 0x40ce, 0x4f97, 0x4aa0, 0x77c4, 0x72f3, 0x7daa, 0x789d,
    0x662f, 0x6318, 0x6c41, 0x6976, 0x1689, 0x13be, 0x1ce7, 0x19d0, 0x0762, 0x0255, 0x0d0c, 0x083b,
    0x355f, 0x3068, 0x3f31, 0x3a06, 0x24b4, 0x2183, 0x2eda, 0x2bed,
];

static FORMAT_INFO_COORDS_QR_MAIN: [(i16, i16); 15] = [
    (0, 8),
    (1, 8),
    (2, 8),
    (3, 8),
    (4, 8),
    (5, 8),
    (7, 8),
    (8, 8),
    (8, 7),
    (8, 5),
    (8, 4),
    (8, 3),
    (8, 2),
    (8, 1),
    (8, 0),
];

static FORMAT_INFO_COORDS_QR_SIDE: [(i16, i16); 15] = [
    (8, -1),
    (8, -2),
    (8, -3),
    (8, -4),
    (8, -5),
    (8, -6),
    (8, -7),
    (-8, 8),
    (-7, 8),
    (-6, 8),
    (-5, 8),
    (-4, 8),
    (-3, 8),
    (-2, 8),
    (-1, 8),
];

static VERSION_INFO_COORDS_BL: [(i16, i16); 18] = [
    (5, -9),
    (5, -10),
    (5, -11),
    (4, -9),
    (4, -10),
    (4, -11),
    (3, -9),
    (3, -10),
    (3, -11),
    (2, -9),
    (2, -10),
    (2, -11),
    (1, -9),
    (1, -10),
    (1, -11),
    (0, -9),
    (0, -10),
    (0, -11),
];

static VERSION_INFO_COORDS_TR: [(i16, i16); 18] = [
    (-9, 5),
    (-10, 5),
    (-11, 5),
    (-9, 4),
    (-10, 4),
    (-11, 4),
    (-9, 3),
    (-10, 3),
    (-11, 3),
    (-9, 2),
    (-10, 2),
    (-11, 2),
    (-9, 1),
    (-10, 1),
    (-11, 1),
    (-9, 0),
    (-10, 0),
    (-11, 0),
];

static VERSION_INFOS: [u32; 34] = [
    0x07c94, 0x085bc, 0x09a99, 0x0a4d3, 0x0bbf6, 0x0c762, 0x0d847, 0x0e60d, 0x0f928, 0x10b78,
    0x1145d, 0x12a17, 0x13532, 0x149a6, 0x15683, 0x168c9, 0x177ec, 0x18ec4, 0x191e1, 0x1afab,
    0x1b08e, 0x1cc1a, 0x1d33f, 0x1ed75, 0x1f250, 0x209d5, 0x216f0, 0x228ba, 0x2379f, 0x24b0b,
    0x2542e, 0x26a64, 0x27541, 0x28c69,
];


static ALIGNMENT_PATTERN_POSITIONS: [&[i16]; 34] = [
    &[6, 22, 38],
    &[6, 24, 42],
    &[6, 26, 46],
    &[6, 28, 50],
    &[6, 30, 54],
    &[6, 32, 58],
    &[6, 34, 62],
    &[6, 26, 46, 66],
    &[6, 26, 48, 70],
    &[6, 26, 50, 74],
    &[6, 30, 54, 78],
    &[6, 30, 56, 82],
    &[6, 30, 58, 86],
    &[6, 34, 62, 90],
    &[6, 28, 50, 72, 94],
    &[6, 26, 50, 74, 98],
    &[6, 30, 54, 78, 102],
    &[6, 28, 54, 80, 106],
    &[6, 32, 58, 84, 110],
    &[6, 30, 58, 86, 114],
    &[6, 34, 62, 90, 118],
    &[6, 26, 50, 74, 98, 122],
    &[6, 30, 54, 78, 102, 126],
    &[6, 26, 52, 78, 104, 130],
    &[6, 30, 56, 82, 108, 134],
    &[6, 34, 60, 86, 112, 138],
    &[6, 30, 58, 86, 114, 142],
    &[6, 34, 62, 90, 118, 146],
    &[6, 30, 54, 78, 102, 126, 150],
    &[6, 24, 50, 76, 102, 128, 154],
    &[6, 28, 54, 80, 106, 132, 158],
    &[6, 32, 58, 84, 110, 136, 162],
    &[6, 26, 54, 82, 110, 138, 166],
    &[6, 30, 58, 86, 114, 142, 170],
];

mod mask_functions {
    pub const fn checkerboard(x: i16, y: i16) -> bool {
        (x + y) % 2 == 0
    }
    pub const fn horizontal_lines(_: i16, y: i16) -> bool {
        y % 2 == 0
    }
    pub const fn vertical_lines(x: i16, _: i16) -> bool {
        x % 3 == 0
    }
    pub const fn diagonal_lines(x: i16, y: i16) -> bool {
        (x + y) % 3 == 0
    }
    pub const fn large_checkerboard(x: i16, y: i16) -> bool {
        ((y / 2) + (x / 3)) % 2 == 0
    }
    pub const fn fields(x: i16, y: i16) -> bool {
        (x * y) % 2 + (x * y) % 3 == 0
    }
    pub const fn diamonds(x: i16, y: i16) -> bool {
        ((x * y) % 2 + (x * y) % 3) % 2 == 0
    }
    pub const fn meadow(x: i16, y: i16) -> bool {
        ((x + y) % 2 + (x * y) % 3) % 2 == 0
    }
}

fn get_mask_function(pattern: MaskPattern) -> fn(i16, i16) -> bool {
    match pattern {
        MaskPattern::Checkerboard => mask_functions::checkerboard,
        MaskPattern::HorizontalLines => mask_functions::horizontal_lines,
        MaskPattern::VerticalLines => mask_functions::vertical_lines,
        MaskPattern::DiagonalLines => mask_functions::diagonal_lines,
        MaskPattern::LargeCheckerboard => mask_functions::large_checkerboard,
        MaskPattern::Fields => mask_functions::fields,
        MaskPattern::Diamonds => mask_functions::diamonds,
        MaskPattern::Meadow => mask_functions::meadow,
    }
}

struct DataModuleIter {
    x: i16,
    y: i16,
    width: i16,
    timing_pattern_column: i16,
}
impl DataModuleIter{
    const fn new(version: Version) -> Self {
        let width = version.width();
        Self {
            x: width - 1,
            y: width - 1,
            width,
            timing_pattern_column: match version {
                Version::Normal(_) => 6,
            },
        }
    }
}
impl Iterator for DataModuleIter {
    type Item = (i16, i16);

    fn next(&mut self) -> Option<(i16, i16)> {
        let adjusted_ref_col = if self.x <= self.timing_pattern_column {
            self.x + 1
        } else {
            self.x
        };
        if adjusted_ref_col <= 0 {
            return None;
        }

        let res = (self.x, self.y);
        let column_type = (self.width - adjusted_ref_col) % 4;

        match column_type {
            2 if self.y > 0 => {
                self.y -= 1;
                self.x += 1;
            }
            0 if self.y < self.width - 1 => {
                self.y += 1;
                self.x += 1;
            }
            0 | 2 if self.x == self.timing_pattern_column + 1 => {
                self.x -= 2;
            }
            _ => {
                self.x -= 1;
            }
        }

        Some(res)
    }
}
#[derive(Debug, Copy, Clone)]
pub enum MaskPattern {
    /// QR code pattern 000: `(x + y) % 2 == 0`.
    Checkerboard = 0b000,

    /// QR code pattern 001: `y % 2 == 0`.
    HorizontalLines = 0b001,

    /// QR code pattern 010: `x % 3 == 0`.
    VerticalLines = 0b010,

    /// QR code pattern 011: `(x + y) % 3 == 0`.
    DiagonalLines = 0b011,

    /// QR code pattern 100: `((x/3) + (y/2)) % 2 == 0`.
    LargeCheckerboard = 0b100,

    /// QR code pattern 101: `(x*y)%2 + (x*y)%3 == 0`.
    Fields = 0b101,

    /// QR code pattern 110: `((x*y)%2 + (x*y)%3) % 2 == 0`.
    Diamonds = 0b110,

    /// QR code pattern 111: `((x+y)%2 + (x*y)%3) % 2 == 0`.
    Meadow = 0b111,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Module {
    /// The module is empty.
    Empty,

    /// The module is of functional patterns which cannot be masked, or pixels
    /// which have been masked.
    Masked(Color),

    /// The module is of data and error correction bits before masking.
    Unmasked(Color),
}
impl From<Module> for Color {
    fn from(module: Module) -> Self {
        match module {
            Module::Empty => Self::Light,
            Module::Masked(c) | Module::Unmasked(c) => c,
        }
    }
}

impl Module {
    pub fn is_dark(self) -> bool {
        Color::from(self) == Color::Dark
    }

    pub fn mask(self, should_invert: bool) -> Self {
        match (self, should_invert) {
            (Self::Empty, true) => Self::Masked(Color::Dark),
            (Self::Empty, false) => Self::Masked(Color::Light),
            (Self::Unmasked(c), true) => Self::Masked(!c),
            (Self::Unmasked(c), false) | (Self::Masked(c), _) => Self::Masked(c),
        }
    }
}

#[derive(Clone)]
pub struct Canvas {
    /// The width and height of the canvas (cached as it is needed frequently).
    width: i16,

    /// The version of the QR code.
    version: Version,

    /// The error correction level of the QR code.
    ec_level: EcLevel,

    /// The modules of the QR code. Modules are arranged in left-to-right, then
    /// top-to-bottom order.
    modules: Vec<Module>,
}

impl Canvas{
    fn draw_number(
        &mut self,
        number: u32,
        bits: u32,
        on_color: Color,
        off_color: Color,
        coords: &[(i16, i16)],
    ) {
        let mut mask = 1 << (bits - 1);
        for &(x, y) in coords {
            let color = if (mask & number) == 0 {
                off_color
            } else {
                on_color
            };
            self.put(x, y, color);
            mask >>= 1;
        }
    }
    fn draw_line(
        &mut self,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        color_even: Color,
        color_odd: Color,
    ) {
        debug_assert!(x1 == x2 || y1 == y2);

        if y1 == y2 {
            // Horizontal line.
            for x in x1..=x2 {
                self.put(x, y1, if x % 2 == 0 { color_even } else { color_odd });
            }
        } else {
            // Vertical line.
            for y in y1..=y2 {
                self.put(x1, y, if y % 2 == 0 { color_even } else { color_odd });
            }
        }
    }
    fn coords_to_index(&self, x: i16, y: i16) -> usize {
        let x = if x < 0 { x + self.width } else { x }.as_usize();
        let y = if y < 0 { y + self.width } else { y }.as_usize();
        y * self.width.as_usize() + x
    }
    fn compute_block_penalty_score(&self) -> u16 {
        let mut total_score = 0;

        for i in 0..self.width - 1 {
            for j in 0..self.width - 1 {
                let this = self.get(i, j);
                let right = self.get(i + 1, j);
                let bottom = self.get(i, j + 1);
                let bottom_right = self.get(i + 1, j + 1);
                if this == right && right == bottom && bottom == bottom_right {
                    total_score += 3;
                }
            }
        }

        total_score
    }
    fn compute_adjacent_penalty_score(&self, is_horizontal: bool) -> u16 {
        let mut total_score = 0;

        for i in 0..self.width {
            let map_fn = |j| {
                if is_horizontal {
                    self.get(j, i)
                } else {
                    self.get(i, j)
                }
            };

            let colors = (0..self.width).map(map_fn).chain(iter::once(Module::Empty));
            let mut last_color = Module::Empty;
            let mut consecutive_len = 1_u16;

            for color in colors {
                if color == last_color {
                    consecutive_len += 1;
                } else {
                    last_color = color;
                    if consecutive_len >= 5 {
                        total_score += consecutive_len - 2;
                    }
                    consecutive_len = 1;
                }
            }
        }

        total_score
    }
    fn compute_finder_penalty_score(&self, is_horizontal: bool) -> u16 {
        static PATTERN: [Color; 7] = [
            Color::Dark,
            Color::Light,
            Color::Dark,
            Color::Dark,
            Color::Dark,
            Color::Light,
            Color::Dark,
        ];

        let mut total_score = 0;

        for i in 0..self.width {
            for j in 0..self.width - 6 {
                // TODO a ref to a closure should be enough?
                let get: Box<dyn Fn(i16) -> Color> = if is_horizontal {
                    Box::new(|k| self.get(k, i).into())
                } else {
                    Box::new(|k| self.get(i, k).into())
                };

                if (j..(j + 7)).map(&*get).ne(PATTERN.iter().copied()) {
                    continue;
                }

                let check = |k| 0 <= k && k < self.width && get(k) != Color::Light;
                if !((j - 4)..j).any(&check) || !((j + 7)..(j + 11)).any(&check) {
                    total_score += 40;
                }
            }
        }

        total_score - 360
    }
    fn compute_total_penalty_scores(&self) -> u16 {
        match self.version {
            Version::Normal(_) => {
                let s1_a = self.compute_adjacent_penalty_score(true);
                let s1_b = self.compute_adjacent_penalty_score(false);
                let s2 = self.compute_block_penalty_score();
                let s3_a = self.compute_finder_penalty_score(true);
                let s3_b = self.compute_finder_penalty_score(false);
                let s4 = self.compute_balance_penalty_score();
                s1_a + s1_b + s2 + s3_a + s3_b + s4
            }
        }
    }
    fn draw_format_info_patterns_with_number(&mut self, format_info: u16) {
        let format_info = u32::from(format_info);
        match self.version {
            Version::Normal(_) => {
                self.draw_number(
                    format_info,
                    15,
                    Color::Dark,
                    Color::Light,
                    &FORMAT_INFO_COORDS_QR_MAIN,
                );
                self.draw_number(
                    format_info,
                    15,
                    Color::Dark,
                    Color::Light,
                    &FORMAT_INFO_COORDS_QR_SIDE,
                );
                self.put(8, -8, Color::Dark); // Dark module.
            }
        }
    }
    fn draw_format_info_patterns(&mut self, pattern: MaskPattern) {
        let format_number = match self.version {
            Version::Normal(_) => {
                let simple_format_number = ((self.ec_level as usize) ^ 1) << 3 | (pattern as usize);
                FORMAT_INFOS_QR[simple_format_number]
            }
        };
        self.draw_format_info_patterns_with_number(format_number);
    }
    fn compute_light_side_penalty_score(&self) -> u16 {
        let h = (1..self.width)
            .filter(|j| !self.get(*j, -1).is_dark())
            .count();
        let v = (1..self.width)
            .filter(|j| !self.get(-1, *j).is_dark())
            .count();

        (h + v + 15 * max(h, v)).as_u16()
    }
    fn draw_alignment_pattern_at(&mut self, x: i16, y: i16) {
        if self.get(x, y) != Module::Empty {
            return;
        }
        for j in -2..=2 {
            for i in -2..=2 {
                self.put(
                    x + i,
                    y + j,
                    match (i, j) {
                        (2 | -2, _) | (_, 2 | -2) | (0, 0) => Color::Dark,
                        _ => Color::Light,
                    },
                );
            }
        }
    }
    fn draw_reserved_format_info_patterns(&mut self) {
        self.draw_format_info_patterns_with_number(0);
    }
    fn draw_timing_patterns(&mut self) {
        let width = self.width;
        let (y, x1, x2) = match self.version {
            Version::Normal(_) => (6, 8, width - 9),
        };
        self.draw_line(x1, y, x2, y, Color::Dark, Color::Light);
        self.draw_line(y, x1, y, x2, Color::Dark, Color::Light);
    }
    fn draw_alignment_patterns(&mut self) {
        match self.version {
            Version::Normal(1) => {}
            Version::Normal(2..=6) => self.draw_alignment_pattern_at(-7, -7),
            Version::Normal(a) => {
                let positions = ALIGNMENT_PATTERN_POSITIONS[(a - 7).as_usize()];
                for x in positions {
                    for y in positions {
                        self.draw_alignment_pattern_at(*x, *y);
                    }
                }
            }
        }
    }
    /**
     * 在指定坐标处绘制一个位置探测图案
     * 
     * 位置探测图案是QR码中最基本的功能图案，呈现为7×7的方块图案，包含三个嵌套的正方形：
     * - 外层：7×7的黑色正方形边框
     * - 中层：5×5的白色正方形
     * - 内层：3×3的黑色正方形
     * 
     * 该函数处理绘制的核心逻辑：
     * 1. 根据坐标的正负调整绘制范围，确保图案正确绘制在QR码边缘
     * 2. 使用嵌套循环绘制整个7×7区域
     * 3. 根据相对位置确定每个模块的颜色：
     *    - 最外层(i,j为±4)：白色
     *    - 次外层(i,j为±3)：黑色
     *    - 中间层(i,j为±2)：白色
     *    - 内部(其余)：黑色
     * 
     * 参数:
     * - x: 位置探测图案中心的x坐标
     * - y: 位置探测图案中心的y坐标
     * 
     * 注意：坐标可以是负数，函数内部会对负坐标进行特殊处理，
     * 这对于在QR码右下角和右上角绘制位置探测图案很重要
     */
    fn draw_finder_pattern_at(&mut self, x: i16, y: i16) {
        let (dx_left, dx_right) = if x >= 0 { (-3, 4) } else { (-4, 3) };
        let (dy_top, dy_bottom) = if y >= 0 { (-3, 4) } else { (-4, 3) };
        for j in dy_top..=dy_bottom {
            for i in dx_left..=dx_right {
                self.put(
                    x + i,
                    y + j,
                    #[allow(clippy::match_same_arms)]
                    match (i, j) {
                        (4 | -4, _) | (_, 4 | -4) => Color::Light,
                        (3 | -3, _) | (_, 3 | -3) => Color::Dark,
                        (2 | -2, _) | (_, 2 | -2) => Color::Light,
                        _ => Color::Dark,
                    },
                );
            }
        }
    }
    /**
     * 绘制QR码的位置探测图案（Finder Patterns）
     * 
     * 该函数负责在QR码上绘制位置探测图案：
     * 1. 对于普通QR码：在左上角、右上角和左下角绘制三个位置探测图案
     * 2. 对于Micro QR码：仅在左上角绘制一个位置探测图案
     * 
     * 位置探测图案是QR码中最显著的特征，它们是由三个嵌套的正方形组成：
     * - 最外层是7×7的正方形
     * - 中间是5×5的黑色正方形
     * - 最内层是3×3的白色正方形，内含1×1的黑色中心点
     * 
     * 这些图案帮助扫描器：
     * - 快速识别和定位QR码
     * - 确定QR码的方向
     * - 计算QR码的大小和版本
     */
    fn draw_finder_patterns(&mut self) {
        self.draw_finder_pattern_at(3, 3);

        match self.version {
            Version::Normal(_) => {
                self.draw_finder_pattern_at(-4, 3);
                self.draw_finder_pattern_at(3, -4);
            }
        }
    }
    fn compute_balance_penalty_score(&self) -> u16 {
        let dark_modules = self.modules.iter().filter(|m| m.is_dark()).count();
        let total_modules = self.modules.len();
        let ratio = dark_modules * 200 / total_modules;
        if ratio >= 100 {
            ratio - 100
        } else {
            100 - ratio
        }
        .as_u16()
    }
    fn draw_version_info_patterns(&mut self) {
        match self.version {
            Version::Normal(1..=6) => {}
            Version::Normal(a) => {
                let version_info = VERSION_INFOS[(a - 7).as_usize()];
                self.draw_number(
                    version_info,
                    18,
                    Color::Dark,
                    Color::Light,
                    &VERSION_INFO_COORDS_BL,
                );
                self.draw_number(
                    version_info,
                    18,
                    Color::Dark,
                    Color::Light,
                    &VERSION_INFO_COORDS_TR,
                );
            }
        }
    }
    fn draw_codewords<I>(&mut self, codewords: &[u8], is_half_codeword_at_end: bool, coords: &mut I)
    where
        I: Iterator<Item = (i16, i16)>,
    {
        let length = codewords.len();
        let last_word = if is_half_codeword_at_end {
            length - 1
        } else {
            length
        };
        for (i, b) in codewords.iter().enumerate() {
            let bits_end = if i == last_word { 4 } else { 0 };
            'outside: for j in (bits_end..=7).rev() {
                let color = if (*b & (1 << j)) == 0 {
                    Color::Light
                } else {
                    Color::Dark
                };
                for (x, y) in coords.by_ref() {
                    let r = self.get_mut(x, y);
                    if *r == Module::Empty {
                        *r = Module::Unmasked(color);
                        continue 'outside;
                    }
                }
                return;
            }
        }
    }

    pub fn new(version: Version, ec_level: EcLevel) -> Self {
        let width = version.width();
        Self {
            width,
            version,
            ec_level,
            modules: vec![Module::Empty; (width * width).as_usize()],
        }
    }
    pub fn apply_mask(&mut self, pattern: MaskPattern) {
        let mask_fn = get_mask_function(pattern);
        for x in 0..self.width {
            for y in 0..self.width {
                let module = self.get_mut(x, y);
                *module = module.mask(mask_fn(x, y));
            }
        }

        self.draw_format_info_patterns(pattern);
    }
    /**
     * 绘制QR码中的所有功能图案
     * 
     * 该函数按顺序绘制以下功能图案：
     * 1. 位置探测图案（Finder Patterns）：用于定位QR码的三个大正方形图案
     * 2. 对齐图案（Alignment Patterns）：用于辅助定位的小正方形图案
     * 3. 格式信息预留区域：用于存储纠错级别和掩码模式信息
     * 4. 时序图案（Timing Patterns）：用于确定模块坐标的黑白交替图案
     * 5. 版本信息图案：用于存储QR码版本信息的图案（仅版本7及以上需要）
     * 
     * 这些功能图案是QR码的重要组成部分，它们帮助扫描器：
     * - 准确定位和识别QR码
     * - 确定QR码的方向
     * - 获取纠错级别和掩码模式
     * - 计算模块坐标
     */
    pub fn draw_all_functional_patterns(&mut self) {
        self.draw_finder_patterns();
        self.draw_alignment_patterns();
        self.draw_reserved_format_info_patterns();
        self.draw_timing_patterns();
        self.draw_version_info_patterns();
    }
    pub fn draw_data(&mut self, data: &[u8], ec: &[u8]) {
        let mut coords = DataModuleIter::new(self.version);
        self.draw_codewords(data, false, &mut coords);
        self.draw_codewords(ec, false, &mut coords);
    }
    pub fn apply_best_mask(&self) -> Self {
        match self.version {
            Version::Normal(_) => ALL_PATTERNS_QR.iter(),
        }
            .map(|ptn| {
                let mut c = self.clone();
                c.apply_mask(*ptn);
                c
            })
            .min_by_key(Self::compute_total_penalty_scores)
            .expect("at least one pattern")
    }
    pub fn into_colors(self) -> Vec<Color> {
        self.modules.into_iter().map(Color::from).collect()
    }
    pub fn get(&self, x: i16, y: i16) -> Module {
        self.modules[self.coords_to_index(x, y)]
    }

    /// Obtains a mutable module at the given coordinates. For convenience,
    /// negative coordinates will wrap around.
    pub fn get_mut(&mut self, x: i16, y: i16) -> &mut Module {
        let index = self.coords_to_index(x, y);
        &mut self.modules[index]
    }

    /// Sets the color of a functional module at the given coordinates. For
    /// convenience, negative coordinates will wrap around.
    pub fn put(&mut self, x: i16, y: i16, color: Color) {
        *self.get_mut(x, y) = Module::Masked(color);
    }
}
