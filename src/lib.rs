//! This crate is a Rust port of [homo].
//!
//! [homo]: https://github.com/itorr/homo



#[cfg(test)]
mod tests;

use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::{Captures, Regex};
use std::{borrow::Cow, cmp::Ordering};
use uint::construct_uint;
pub use uint::FromDecStrErr;

// 匹配 *(1) 或 +(0) 冗余运算
static RE01: Lazy<Regex> = Lazy::new(|| Regex::new(pomsky!("*(1)" | "+(0)" End)).unwrap());
// 匹配数字
static RE_DIGITS: Lazy<Regex> = Lazy::new(|| Regex::new(pomsky!([digit]+)).unwrap());
static RE乘除冗余括号: Lazy<Regex> =
    Lazy::new(|| Regex::new(pomsky!(:(["*/"]) '(' :(!["+-()"]+) ')')).unwrap());
static RE加减冗余括号: Lazy<Regex> =
    Lazy::new(|| Regex::new(pomsky!(:(["+-"]) '(' :(!["()"]+) ')' :(["+-"]))).unwrap());
static RE行末加减冗余括号: Lazy<Regex> =
    Lazy::new(|| Regex::new(pomsky!(:(["+-"]) '(' :(!["()"]+) ')' End)).unwrap());
static RE行冗余括号: Lazy<Regex> =
    Lazy::new(|| Regex::new(pomsky!(Start '(' :(!["()"]+) ')' End)).unwrap());
// 基础 114514 算式
static MEMO: [(u32, &'static str); 520] = [
    (0, "(1-1)*4514"),
    (1, "11/(45-1)*4"),
    (2, "-11+4-5+14"),
    (3, "11*-4+51-4"),
    (4, "-11-4+5+14"),
    (5, "11-4*5+14"),
    (6, "1-14+5+14"),
    (7, "11-4+5-1-4"),
    (8, "11-4+5/1-4"),
    (9, "11-4+5+1-4"),
    (10, "-11/4+51/4"),
    (11, "11*-4+51+4"),
    (12, "-11+4+5+14"),
    (13, "1*14-5/1+4"),
    (14, "11+4-5/1+4"),
    (15, "1+14-5+1+4"),
    (16, "11-4-5+14"),
    (17, "11+4*5-14"),
    (18, "1+1+4*5*1-4"),
    (19, "1+1+4*5+1-4"),
    (20, "-11+45-14"),
    (21, "-1-1+4+5+14"),
    (22, "1*14+5-1+4"),
    (23, "1*14-5+14"),
    (24, "1+14-5+14"),
    (25, "11*4-5-14"),
    (26, "11-4+5+14"),
    (27, "11+4*5/1-4"),
    (28, "11+4*5+1-4"),
    (29, "-11+45-1-4"),
    (30, "1*-1+45-14"),
    (31, "1/1*45-14"),
    (32, "1*1+45-14"),
    (33, "1+1+45-14"),
    (34, "1-14+51-4"),
    (35, "11*4+5-14"),
    (36, "11+4*5+1+4"),
    (37, "-11+45-1+4"),
    (38, "-11+45*1+4"),
    (39, "-11+45+1+4"),
    (40, "-11+4*51/4"),
    (41, "1/1*45*1-4"),
    (42, "11+45-14"),
    (43, "1+1*45+1-4"),
    (44, "114-5*14"),
    (45, "11+4*5+14"),
    (46, "11*4+5+1-4"),
    (47, "1/-1+45-1+4"),
    (48, "-11+45+14"),
    (49, "1*1*45/1+4"),
    (50, "1+1*45/1+4"),
    (51, "11+45-1-4"),
    (52, "11+45/1-4"),
    (53, "11+45+1-4"),
    (54, "11-4+51-4"),
    (55, "-1+14*5-14"),
    (56, "1*14*5-14"),
    (57, "1+14*5-14"),
    (58, "-1+1*45+14"),
    (59, "114-51-4"),
    (60, "11+45*1+4"),
    (61, "1+1+45+14"),
    (62, "1+14+51-4"),
    (63, "11*4+5+14"),
    (64, "11*4+5*1*4"),
    (65, "1*14*5-1-4"),
    (66, "1*14*5-1*4"),
    (67, "1-1*4+5*14"),
    (68, "1+1-4+5*14"),
    (69, "1*14+51+4"),
    (70, "11+45+14"),
    (71, "(1+14)*5-1*4"),
    (72, "-1-1+4+5*14"),
    (73, "1*14*5-1+4"),
    (74, "1/1*4+5*14"),
    (75, "1+14*5*1+4"),
    (76, "1+1+4+5*14"),
    (77, "11-4+5*14"),
    (78, "(1+1)*4+5*14"),
    (79, "1*-1+4*5/1*4"),
    (80, "1-1+4*5*1*4"),
    (81, "1/1+4*5*1*4"),
    (82, "1+1+4*5/1*4"),
    (83, "-1+14+5*14"),
    (84, "1*14+5*14"),
    (85, "1+14+5*14"),
    (86, "(1+1)*45*1-4"),
    (87, "11+4*(5+14)"),
    (88, "1*14*(5+1)+4"),
    (89, "(1+14)*5+14"),
    (90, "-114+51*4"),
    (91, "11*4+51-4"),
    (92, "(1+1)*(45-1)+4"),
    (93, "(1+1)*45-1+4"),
    (94, "114-5/1*4"),
    (95, "114-5-14"),
    (96, "11*(4+5)+1-4"),
    (97, "1+1*4*(5+1)*4"),
    (98, "1+1+4*(5+1)*4"),
    (99, "11*4+51+4"),
    (100, "1*(1+4)*5*1*4"),
    (101, "1+1*4*5*(1+4)"),
    (102, "11*(4+5)-1+4"),
    (103, "11*(4+5)+1*4"),
    (104, "114-5-1-4"),
    (105, "114+5-14"),
    (106, "114-5+1-4"),
    (107, "11-4*-(5+1)*4"),
    (110, "-(11-451)/4"),
    (111, "11+4*5*(1+4)"),
    (112, "114-5-1+4"),
    (113, "114-5/1+4"),
    (114, "11*4+5*14"),
    (115, "114+5*1-4"),
    (116, "114+5+1-4"),
    (117, "(1-14)*(5-14)"),
    (118, "(1+1)*(45+14)"),
    (120, "-(1+1)*4*5*(1-4)"),
    (121, "11*(45-1)/4"),
    (122, "114+5-1+4"),
    (123, "114-5+14"),
    (124, "114+5+1+4"),
    (125, "-1-14*(5-14)"),
    (126, "1*(14-5)*14"),
    (127, "1-14*(5-14)"),
    (128, "1+1+(4+5)*14"),
    (129, "114+5*-(1-4)"),
    (130, "-1+145-14"),
    (131, "1*145-14"),
    (132, "1+145-14"),
    (133, "114+5+14"),
    (134, "114+5/1*4"),
    (135, "-1/1*45*(1-4)"),
    (136, "1*1-45*(1-4)"),
    (137, "1+1-45*(1-4)"),
    (138, "-1*(1+45)*(1-4)"),
    (139, "-1+145-1-4"),
    (140, "1*145-1-4"),
    (141, "1+145-1-4"),
    (142, "1+145*1-4"),
    (143, "1+145+1-4"),
    (146, "11+45*-(1-4)"),
    (147, "-1+145-1+4"),
    (148, "1*145-1+4"),
    (149, "1*145*1+4"),
    (150, "1+145*1+4"),
    (151, "1+145+1+4"),
    (152, "(1+1)*4*(5+14)"),
    (154, "11*(4-5)*-14"),
    (157, "1*(1-4)*-51+4"),
    (158, "-1+145+14"),
    (159, "1*145+14"),
    (160, "1+145+14"),
    (161, "114+51-4"),
    (165, "11*-45/(1-4)"),
    (168, "(11+45)*-(1-4)"),
    (169, "114+51+4"),
    (170, "(11-45)*-(1+4)"),
    (171, "114*(5+1)/4"),
    (172, "11*4*(5-1)-4"),
    (174, "-1-1+(45-1)*4"),
    (175, "-1+1*(45-1)*4"),
    (176, "1*1*(45-1)*4"),
    (177, "1+1*(45-1)*4"),
    (178, "-1-1+45*1*4"),
    (179, "-1/1+45*1*4"),
    (180, "1*1*45*1*4"),
    (181, "1+1*45*1*4"),
    (182, "1+1+45/1*4"),
    (183, "-1+1*(45+1)*4"),
    (184, "114+5*14"),
    (185, "1-1*-(45+1)*4"),
    (186, "1+1+(45+1)*4"),
    (187, "1/-1+4*(51-4)"),
    (188, "1-1-(4-51)*4"),
    (189, "-11-4+51*4"),
    (190, "1*-14+51*4"),
    (191, "1-14+51*4"),
    (192, "(1+1)*4*(5+1)*4"),
    (195, "(1-14)*5*(1-4)"),
    (196, "-(1+1)*4+51*4"),
    (197, "-11+4+51*4"),
    (198, "-1-1+4*51-4"),
    (199, "1*-1+4*51-4"),
    (200, "1/1*4*51-4"),
    (201, "1/1-4+51*4"),
    (202, "1+1-4+51*4"),
    (204, "(1-1)/4+51*4"),
    (206, "11*4*5-14"),
    (207, "-1+1*4*51+4"),
    (208, "1*1*4+51*4"),
    (209, "1+1*4*51+4"),
    (210, "1+1+4+51*4"),
    (211, "11-4+51*4"),
    (212, "(1+1)*4+51*4"),
    (214, "-11+45*(1+4)"),
    (215, "11*4*5-1-4"),
    (216, "11*4*5-1*4"),
    (217, "11*4*5+1-4"),
    (218, "1*14+51*4"),
    (219, "1+14+51*4"),
    (220, "1*1*(4+51)*4"),
    (221, "1/1+4*(51+4)"),
    (222, "1+1+4*(51+4)"),
    (223, "11*4*5-1+4"),
    (224, "11*4*5/1+4"),
    (225, "11*4*5+1+4"),
    (226, "1*1+45*(1+4)"),
    (227, "1+1+45*(1+4)"),
    (229, "1145/(1+4)"),
    (230, "1*(1+45)*(1+4)"),
    (231, "11+4*(51+4)"),
    (234, "11*4*5+14"),
    (235, "1*(1+4)*(51-4)"),
    (236, "11+45*(1+4)"),
    (240, "(11+4)*(5-1)*4"),
    (247, "-(1-14)*(5+14)"),
    (248, "11*4+51*4"),
    (251, "1*-(1+4)*-51-4"),
    (252, "(114-51)*4"),
    (257, "(1+1)/4*514"),
    (259, "1*(1+4)*51+4"),
    (260, "1*(14+51)*4"),
    (265, "-1+14*(5+14)"),
    (266, "1*14*(5+14)"),
    (267, "1+14*(5+14)"),
    (268, "11*4*(5+1)+4"),
    (269, "-11+4*5*14"),
    (270, "(1+1)*45*-(1-4)"),
    (275, "1*(1+4)*(51+4)"),
    (278, "-1-1+4*5*14"),
    (279, "1*-1+4*5*14"),
    (280, "1-1+4*5*14"),
    (281, "1+14*5/1*4"),
    (282, "1+1+4*5*14"),
    (285, "(11+4)*(5+14)"),
    (286, "(1145-1)/4"),
    (291, "11+4*5*14"),
    (297, "-11*(4+5)*(1-4)"),
    (300, "(11+4)*5/1*4"),
    (312, "(1-14)*-(5+1)*4"),
    (318, "114+51*4"),
    (325, "-(1-14)*5*(1+4)"),
    (327, "-(114-5)*(1-4)"),
    (329, "(11-4)*(51-4)"),
    (335, "-1+14*(5+1)*4"),
    (336, "1*14*(5+1)*4"),
    (337, "1-14*-(5+1)*4"),
    (341, "11*(45-14)"),
    (349, "-1+14*5*(1+4)"),
    (350, "1*(1+4)*5*14"),
    (351, "1+14*-5*-(1+4)"),
    (352, "(1+1)*(45-1)*4"),
    (353, "(11-4)*51-4"),
    (357, "(114+5)*-(1-4)"),
    (360, "(1+1)*45*1*4"),
    (361, "(11-4)*51+4"),
    (363, "(1+1451)/4"),
    (368, "(1+1)*(45+1)*4"),
    (375, "(1+14)*5*(1+4)"),
    (376, "(1+1)*4*(51-4)"),
    (385, "(11-4)*(51+4)"),
    (396, "11*4*-(5-14)"),
    (400, "-114+514"),
    (404, "(1+1)*4*51-4"),
    (412, "(1+1)*4*51+4"),
    (432, "(1-145)*(1-4)"),
    (434, "-1-145*(1-4)"),
    (435, "-1*145*(1-4)"),
    (436, "-11+451-4"),
    (438, "(1+145)*-(1-4)"),
    (440, "(1+1)*4*(51+4)"),
    (444, "-11+451+4"),
    (445, "-1-1+451-4"),
    (446, "1*-1+451-4"),
    (447, "1/1*451-4"),
    (448, "1+1*451-4"),
    (449, "1+1+451-4"),
    (450, "(1+1)*45*(1+4)"),
    (452, "114*(5-1)-4"),
    (453, "-1-1+451+4"),
    (454, "-1+1*451+4"),
    (455, "1-1+451+4"),
    (456, "1*1+451+4"),
    (457, "1+1+451+4"),
    (458, "11+451-4"),
    (460, "114*(5-1)+4"),
    (466, "11+451+4"),
    (470, "-11*4+514"),
    (476, "(114+5)/1*4"),
    (480, "11*(45-1)-4"),
    (481, "11*45-14"),
    (488, "11*(45-1)+4"),
    (490, "11*45-1-4"),
    (491, "11*45-1*4"),
    (492, "11*45+1-4"),
    (495, "11*(4+5)*(1+4)"),
    (498, "11*45-1+4"),
    (499, "11*45*1+4"),
    (500, "11*45+1+4"),
    (501, "1-14+514"),
    (502, "11*(45+1)-4"),
    (506, "-(1+1)*4+514"),
    (507, "-11+4+514"),
    (508, "-1-1-4+514"),
    (509, "11*45+14"),
    (510, "1-1-4+514"),
    (511, "1*1-4+514"),
    (512, "1+1-4+514"),
    (513, "-11*(4-51)-4"),
    (514, "(1-1)/4+514"),
    (516, "-1-1+4+514"),
    (517, "-1+1*4+514"),
    (518, "1-1+4+514"),
    (519, "1+1*4+514"),
    (520, "1+1+4+514"),
    (521, "11-4+514"),
    (522, "(1+1)*4+514"),
    (527, "-1+14+514"),
    (528, "1*14+514"),
    (529, "1+14+514"),
    (545, "(114-5)*(1+4)"),
    (556, "114*5-14"),
    (558, "11*4+514"),
    (560, "(1+1)*4*5*14"),
    (561, "11/4*51*4"),
    (565, "114*5-1-4"),
    (566, "114*5*1-4"),
    (567, "114*5+1-4"),
    (573, "114*5-1+4"),
    (574, "114*5/1+4"),
    (575, "114*5+1+4"),
    (576, "1*(145-1)*4"),
    (579, "-1+145*1*4"),
    (580, "1*145/1*4"),
    (581, "1+145*1*4"),
    (584, "114*5+14"),
    (595, "(114+5)*(1+4)"),
    (601, "11*(4+51)-4"),
    (609, "11*(4+51)+4"),
    (611, "(1-14)*-(51-4)"),
    (612, "-1*(1-4)*51*4"),
    (616, "1*-(1-45)*14"),
    (619, "-11+45*14"),
    (628, "114+514"),
    (629, "1*-1+45*14"),
    (630, "1*1*45*14"),
    (631, "1*1+45*14"),
    (632, "1+1+45*14"),
    (641, "11+45*14"),
    (644, "1*(1+45)*14"),
    (649, "11*(45+14)"),
    (657, "-1+14*(51-4)"),
    (658, "1*14*(51-4)"),
    (659, "1+14*(51-4)"),
    (660, "(114+51)*4"),
    (667, "-(1-14)*51+4"),
    (680, "114*(5+1)-4"),
    (688, "114*(5+1)+4"),
    (704, "11*4*(5-1)*4"),
    (705, "(1+14)*(51-4)"),
    (709, "-1+14*51-4"),
    (710, "1*14*51-4"),
    (711, "1+14*51-4"),
    (715, "(1-14)*-(51+4)"),
    (717, "-1-14*-51+4"),
    (718, "1*14*51+4"),
    (719, "1+14*51+4"),
    (720, "(1-145)*-(1+4)"),
    (724, "-1-145*-(1+4)"),
    (725, "1*145*(1+4)"),
    (726, "1+145*(1+4)"),
    (730, "(1+145)*(1+4)"),
    (761, "(1+14)*51-4"),
    (769, "(11+4)*51+4"),
    (770, "1*14*(51+4)"),
    (771, "1+14*(51+4)"),
    (784, "(11+45)*14"),
    (805, "-11+4*51*4"),
    (814, "-1-1+4*51*4"),
    (815, "-1+1*4*51*4"),
    (816, "1*1*4*51*4"),
    (817, "1*1+4*51*4"),
    (818, "1+1+4*51*4"),
    (825, "(11+4)*(51+4)"),
    (827, "11+4*51*4"),
    (836, "11*4*(5+14)"),
    (880, "11*4*5*1*4"),
    (894, "(1+1)*(451-4)"),
    (898, "(1+1)*451-4"),
    (906, "(1+1)*451+4"),
    (910, "-(1-14)*5*14"),
    (979, "-1+14*5*14"),
    (980, "1*14*5*14"),
    (981, "1+14*5*14"),
    (1020, "1*(1+4)*51*4"),
    (1026, "114*-(5-14)"),
    (1036, "(1+1)*(4+514)"),
    (1050, "(11+4)*5*14"),
    (1056, "11*4*(5+1)*4"),
    (1100, "11*4*5*(1+4)"),
    (1131, "1145-14"),
    (1140, "(1145-1)-4"),
    (1141, "1145-1*4"),
    (1142, "1145+1-4"),
    (1148, "1145-1+4"),
    (1149, "1145+1*4"),
    (1150, "1145+1+4"),
    (1159, "1145+14"),
    (1260, "(1+1)*45*14"),
    (1386, "11*(4+5)*14"),
    (1428, "(11-4)*51*4"),
    (1446, "-1+1451-4"),
    (1447, "1*1451-4"),
    (1448, "1+1451-4"),
    (1454, "-1+1451+4"),
    (1455, "1*1451+4"),
    (1456, "1+1451+4"),
    (1485, "11*-45*(1-4)"),
    (1526, "(114-5)*14"),
    (1542, "1*-(1-4)*514"),
    (1632, "(1+1)*4*51*4"),
    (1666, "(114+5)*14"),
    (1710, "114*-5*(1-4)"),
    (1760, "-(11-451)*4"),
    (1793, "-11+451*4"),
    (1800, "1*-(1-451)*4"),
    (1802, "-1-1+451*4"),
    (1803, "1*-1+451*4"),
    (1804, "1-1+451*4"),
    (1805, "1+1*451*4"),
    (1806, "1+1+451*4"),
    (1808, "1*(1+451)*4"),
    (1815, "11+451*4"),
    (1824, "114*(5-1)*4"),
    (1848, "(11+451)*4"),
    (1936, "11*(45-1)*4"),
    (1980, "11*45*1*4"),
    (2016, "-(1-145)*14"),
    (2024, "11*(45+1)*4"),
    (2029, "-1+145*14"),
    (2030, "1*145*14"),
    (2031, "1+145*14"),
    (2044, "(1+145)*14"),
    (2045, "-11+4*514"),
    (2054, "-1-1+4*514"),
    (2055, "-1/1+4*514"),
    (2056, "1/1*4*514"),
    (2057, "1/1+4*514"),
    (2058, "1+1+4*514"),
    (2067, "11+4*514"),
    (2068, "11*4*(51-4)"),
    (2166, "114*(5+14)"),
    (2240, "11*4*51-4"),
    (2248, "11*4*51+4"),
    (2280, "114*5*1*4"),
    (2420, "11*4*(51+4)"),
    (2475, "11*45*(1+4)"),
    (2570, "1*(1+4)*514"),
    (2652, "(1-14)*51*-4"),
    (2736, "114*(5+1)*4"),
    (2850, "114*5*(1+4)"),
    (2855, "-1+14*51*4"),
    (2856, "1*14*51*4"),
    (2857, "1+14*51*4"),
    (3060, "(11+4)*51*4"),
    (3080, "11*4*5*14"),
    (3435, "-1145*(1-4)"),
    (3598, "(11-4)*514"),
    (3608, "(1+1)*451*4"),
    (4112, "(1+1)*4*514"),
    (4503, "-11+4514"),
    (4512, "-1-1+4514"),
    (4513, "-1*1+4514"),
    (4514, "1-1+4514"),
    (4515, "1+1*4514"),
    (4516, "1+1+4514"),
    (4525, "11+4514"),
    (4576, "(1145-1)*4"),
    (4580, "1145*1*4"),
    (4584, "(1145+1)*4"),
    (4917, "11*(451-4)"),
    (4957, "11*451-4"),
    (4965, "11*451+4"),
    (5005, "11*(451+4)"),
    (5358, "114*(51-4)"),
    (5610, "-11*(4-514)"),
    (5698, "11*(4+514)"),
    (5725, "1145*(1+4)"),
    (5800, "(1-1451)*-4"),
    (5803, "-1+1451*4"),
    (5804, "1*1451*4"),
    (5805, "1+1451*4"),
    (5808, "(1+1451)*4"),
    (5810, "114*51-4"),
    (5818, "114*51+4"),
    (6270, "114*(51+4)"),
    (6682, "(1-14)*-514"),
    (6930, "11*45*14"),
    (7195, "-1+14*514"),
    (7196, "1*14*514"),
    (7197, "1+14*514"),
    (7710, "(1+14)*514"),
    (7980, "114*5*14"),
    (8976, "11*4*51*4"),
    (9028, "(1+1)*4514"),
    (11447, "11451-4"),
    (11455, "11451+4"),
    (14513, "-1+14514"),
    (14514, "1*14514"),
    (14515, "1+14514"),
    (16030, "1145*14"),
    (19844, "11*451*4"),
    (22616, "11*4*514"),
    (23256, "114*51*4"),
    (45804, "11451*4"),
    (49654, "11*4514"),
    (58596, "114*514"),
    (114514, "114514"),
    (229028, "(114514+114514)"),
];

// 大数，1024 bit 的无符号整型
construct_uint! {
    struct U1024(16);
}

enum Infimum {
    Less(u32),
    Eqaul,
}

// 使用二分搜索，凭数字字符串获取 114514 算式
fn get_by_str(num: &str) -> &'static str {
    let num: u32 = num.parse().unwrap();
    let i = MEMO.binary_search_by_key(&num, |&(k, _)| k).unwrap();

    MEMO[i].1
}

// 使用二分搜索，寻找 x 的最大下界，用于计算除数
fn infimum_of(x: u32) -> Infimum {
    // 特判，对于过大的 x 立即返回
    if x > 229028 {
        return Infimum::Less(229028);
    }

    let mut low = 0;
    let mut high = MEMO.len() - 1; // 避免 mid + 1 == len

    while low < high {
        let mid = (low + high) / 2;
        let infimum = MEMO[mid].0; // 可能的最大下确界

        match x.cmp(&infimum) {
            Ordering::Equal => {
                return Infimum::Eqaul;
            }

            Ordering::Greater => {
                if x < MEMO[mid + 1].0 {
                    return Infimum::Less(infimum);
                } else {
                    low = mid + 1;
                }
            }

            Ordering::Less => {
                high = mid;
            }
        }
    }

    // 二分迭代结束，却仍未退出，
    // 结合开头，说明是 229028
    return Infimum::Eqaul;
}

// 递归分解大数至不完全 114514 算式
fn decompose(num: U1024) -> String {
    let div = match num.try_into() {
        Ok(x) => match infimum_of(x) {
            // 取最大下确界
            Infimum::Less(infimum) => infimum,
            // 已缓存，则返回相应键作为基本元素，稍后替换
            Infimum::Eqaul => return x.to_string(),
        },

        // num 过大，使用最大键作为除数
        Err(_) => 229028u32,
    };

    let quotient = decompose(num / div);
    let remainder = decompose(num % div);

    RE01.replace_all(&format!("{div}*({quotient})+({remainder})"), "")
        .to_string()
}

/// Decompose integer into the combination of 114514 formulae.
///
/// # Failure
/// - The integer doesn't meet the condition, --2<sup>1024</sup> + 1 ≤ `num` ≤ 2<sup>1024</sup> -- 1 .
/// - There are invalid characters in `num`.
pub fn roar(mut num: &str) -> Result<String, FromDecStrErr> {
    // 若传入参数为负，则标记，并脱去负号
    let minus = match num.strip_prefix('-') {
        Some(positive) => {
            num = positive;
            true
        }
        None => false,
    };
    let x = U1024::from_dec_str(num)?;
    let mut s = decompose(x);

    // 若执行展开，则改变 s
    if let Cow::Owned(expansion) = RE_DIGITS.replace_all(&s, |caps: &Captures| {
        get_by_str(caps.get(0).unwrap().as_str())
    }) {
        s = expansion;
    }

    if minus {
        s = format!("(11-4-5+1-4)*({s})");
    }

    while let Cow::Owned(peeled) = RE乘除冗余括号.replace(&s, "$1$2") {
        s = peeled;
    }

    while let Cow::Owned(peeled) = RE加减冗余括号.replace(&s, "$1$2$3") {
        s = peeled;
    }

    while let Cow::Owned(peeled) = RE行末加减冗余括号.replace(&s, "$1$2") {
        s = peeled;
    }

    // 使用 Cow<str> ，避免最后不必要的复制
    let peeled = RE行冗余括号.replace(&s, "$1");

    Ok(peeled.replace("+-", "-"))
}
